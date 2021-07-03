use super::opencl;

use crate::channel;
use crate::error;
use crate::hash;
use crate::options;
use crate::results;

use crate::options::SharedAccessor;

fn compute_results<'a, H: hash::Hash>(
    environment: &opencl::Environment<'a, H>,
    input: &[H],
    out_buffer: &ocl::Buffer<opencl::Output>,
    options: &options::Decrypt<H>,
) -> Result<Vec<results::Pair>, error::Error> {
    let mut output = vec![opencl::Output::default(); out_buffer.len()];
    out_buffer
        .read(&mut output)
        .enq()
        .map_err(|err| error!(err; "OpenCL: Failed to read output buffer"))?;

    let mut results = Vec::with_capacity(out_buffer.len());

    for (i, plain) in output.iter().enumerate() {
        if plain.is_valid() {
            results.push(results::Pair::new(
                input[i].to_string(),
                format!("{}{}", &options.prefix(), plain.printable(&environment)),
            ));
        }
    }

    // The kernel will output zeros if nothing is found
    // We should hash this in the CPU to make sure it doesn't match anything
    if results.len() < input.len() {
        let salted_prefix = format!("{}{}", &options.salt(), &options.prefix());

        for i in 0..environment.cpu_iterations() {
            use eytzinger::SliceExt;

            let zeros = opencl::Output::new(0, i).printable(&environment);
            let hash = H::digest(&salted_prefix, &zeros);

            if input.eytzinger_search(&hash).is_some() {
                let result = format!("{}{}", &options.prefix(), &zeros);
                results.push(results::Pair::new(hash.to_string(), result));
            }

            if results.len() == input.len() {
                break;
            }
        }
    }

    Ok(results)
}

pub fn execute<H: hash::Hash>(
    options: &options::Decrypt<H>,
    channel: &impl channel::Channel,
) -> Result<results::Summary, error::Error> {
    let time = std::time::Instant::now();

    if (options.input().len() as u64) >= (i32::max_value() as u64) {
        bail!("Input count too large. GPU kernel defines are fixed at i32 (2,147,483,647)");
    }

    let input = options.input_as_eytzinger();

    let environment = opencl::setup_for(options)?;
    let program = environment.make_program()?;

    let in_buffer = if environment.memory() < H::bytes() * input.len() as u64 {
        unsafe { ocl::Buffer::builder().use_host_slice(&input) }
    } else {
        ocl::Buffer::builder().copy_host_slice(&input)
    }
    .flags(ocl::MemFlags::READ_ONLY)
    .len(options.input().len())
    .queue(environment.queue().clone())
    .build()
    .map_err(|err| error!(err; "OpenCL: Failed to create input buffer"))?;

    let out_buffer = ocl::Buffer::builder()
        .flags(ocl::MemFlags::WRITE_ONLY)
        .len(options.input().len())
        .queue(environment.queue().clone())
        .build()
        .map_err(|err| error!(err; "OpenCL: Failed to create output buffer"))?;

    channel.progress(0);
    for i in 0..environment.cpu_iterations() {
        let kernel = ocl::Kernel::builder()
            .program(&program)
            .name("crack")
            .queue(environment.queue().clone())
            .global_work_size(environment.range())
            .arg(&in_buffer)
            .arg(&out_buffer)
            .arg(i)
            .build()
            .map_err(|err| error!(err; "OpenCL: Failed to build kernel"))?;

        unsafe {
            kernel
                .enq()
                .map_err(|err| error!(err; "OpenCL: Failed to enqueue kernel"))?;
        }

        // If we enqueue too many, OpenCL will abort
        // Send every 7th iteration
        if i & 0b111 == 0b111 {
            // Allowed because it will always be <= 100
            #[allow(clippy::cast_possible_truncation)]
            channel.progress((i * 100 / environment.cpu_iterations()) as u8);
            if channel.should_terminate() {
                break;
            }
            environment
                .queue()
                .finish()
                .map_err(|err| error!(err; "OpenCL: Failed to wait for queue segment to finish"))?;
        }
    }

    environment
        .queue()
        .finish()
        .map_err(|err| error!(err; "OpenCL: Failed to wait for queue to finish"))?;

    let results = compute_results(&environment, &input, &out_buffer, &options)?;

    for result in &results {
        channel.result(&result.hash, &result.plain);
    }

    Ok(results::Summary {
        total_count: input.len(),
        duration: time.elapsed(),
        hash_count: options.number_space(),
        threads: environment.range(),
        results,
    })
}
