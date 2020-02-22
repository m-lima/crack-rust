use crate::hash;
use crate::options;
use crate::summary;

static MAX_GPU_LENGTH: u8 = 7;
static MAX_GPU_RANGE: i32 = 10_000_000; // 10 ^ MAX_LENGTH

mod source {
    static MD5: &str = include!(concat!(env!("OUT_DIR"), "/cl/md5.rs"));
    static SHA256: &str = include!(concat!(env!("OUT_DIR"), "/cl/sha256.rs"));

    pub(super) fn from<'a>(algorithm: &super::options::Algorithm) -> &'a str {
        match algorithm {
            super::options::Algorithm::MD5 => MD5,
            super::options::Algorithm::SHA256 => SHA256,
        }
    }
}

struct KernelParameters {
    iterations: u32,
    range: i32,
    length_on_cpu: u8,
    length_on_gpu: u8,
}

impl KernelParameters {
    fn from(options: &options::Decrypt) -> Self {
        let length_on_cpu = if MAX_GPU_LENGTH > options.length {
            0
        } else {
            options.length - MAX_GPU_LENGTH
        };
        let iterations = 10_u32.pow(u32::from(length_on_cpu));

        // Allowed because min(MAX_GPU_RANGE, ...) will always fit in i32
        // Allowed because MAX_GPU_RANGE is positive
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let range = std::cmp::min(MAX_GPU_RANGE as u64, options.number_space) as i32;

        let length_on_gpu = options.length - length_on_cpu;

        Self {
            iterations,
            length_on_cpu,
            range,
            length_on_gpu,
        }
    }

    #[inline(always)]
    fn cpu_length_definition(&self) -> &'static str {
        if self.iterations > 1 {
            "CONST_LENGTH_ON_CPU"
        } else {
            "INVALID_VALUE"
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
struct OpenclOutput {
    data: [u32; 2],
}

unsafe impl ocl::OclPrm for OpenclOutput {}

impl OpenclOutput {
    fn new(gpu_value: u32, cpu_value: u32) -> Self {
        Self {
            data: [gpu_value, cpu_value],
        }
    }

    #[inline(always)]
    fn gpu_value(&self) -> u32 {
        self.data[0]
    }

    #[inline(always)]
    fn cpu_value(&self) -> u32 {
        self.data[1]
    }

    #[inline(always)]
    fn is_valid(&self) -> bool {
        self.data[0] > 0
    }

    fn printable(&self, kernel_parameters: &KernelParameters) -> String {
        if kernel_parameters.iterations > 1 {
            format!(
                "{:02$}{:03$}",
                self.cpu_value(),
                self.gpu_value(),
                usize::from(kernel_parameters.length_on_cpu),
                usize::from(kernel_parameters.length_on_gpu)
            )
        } else {
            format!(
                "{:01$}",
                self.gpu_value(),
                usize::from(kernel_parameters.length_on_gpu)
            )
        }
    }
}

fn inject_prefix(salted_prefix: &str, src: &str) -> String {
    let mut injected_code = String::new();
    for (i, c) in salted_prefix.chars().enumerate() {
        injected_code.push_str(format!("value.bytes[{}] = \'{}\';", i, c).as_str());
    }

    let mut output = String::new();
    for line in src.lines() {
        if line.ends_with("// %%PREFIX%%") {
            output.push_str(injected_code.as_str());
        } else {
            output.push_str(line);
        };
        output.push('\n');
    }

    output
}

// Allowed because of previous check for options.shared.input.len() <= i32.max_value()
// Allowed because salted prefix is limited in size
#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
fn make_program(
    salted_prefix: &str,
    options: &options::Decrypt,
    kernel_parameters: &KernelParameters,
    context: &ocl::Context,
    device: ocl::Device,
) -> ocl::Program {
    let source = inject_prefix(salted_prefix, source::from(&options.shared.algorithm));

    ocl::Program::builder()
        .source(source)
        .devices(device)
        .cmplr_def("CONST_BEGIN", salted_prefix.len() as i32)
        .cmplr_def(
            "CONST_END",
            i32::from(salted_prefix.len() as u8 + options.length),
        )
        .cmplr_def("CONST_TARGET_COUNT", options.shared.input.len() as i32)
        .cmplr_def(
            kernel_parameters.cpu_length_definition(),
            i32::from(kernel_parameters.length_on_cpu),
        )
        .build(context)
        .unwrap_or_else(|err| {
            eprintln!("OpenCL: Failed to build program: {}", err);
            std::process::exit(-1);
        })
}

fn first_gpu() -> (ocl::Platform, ocl::Device) {
    let mut out = Vec::new();
    for platform in ocl::Platform::list() {
        if let Ok(all_devices) = ocl::Device::list_all(&platform) {
            for device in all_devices {
                out.push((platform, device));
            }
        }
    }

    // Prefer GPU
    out.sort_by(|&(_, ref a), &(_, ref b)| {
        let a_type = a.info(ocl::core::DeviceInfo::Type);
        let b_type = b.info(ocl::core::DeviceInfo::Type);
        if let (
            Ok(ocl::core::DeviceInfoResult::Type(a_type)),
            Ok(ocl::core::DeviceInfoResult::Type(b_type)),
        ) = (a_type, b_type)
        {
            b_type.cmp(&a_type)
        } else {
            (0).cmp(&0)
        }
    });

    if out.first().is_none() {
        eprintln!("OpenCL: Failed to find any OpenCL devices");
        std::process::exit(-1);
    }
    *out.first().unwrap()
}

fn setup_opencl() -> (ocl::Device, ocl::Context, ocl::Queue) {
    let (platform, device) = first_gpu();
    let context = ocl::Context::builder()
        .platform(platform)
        .devices(device)
        .build()
        .unwrap_or_else(|err| {
            eprintln!("OpenCL: Failed to create context: {}", err);
            std::process::exit(-1);
        });
    let queue = ocl::Queue::new(&context, device, None).unwrap();
    (device, context, queue)
}

fn compute_results<D: digest::Digest, C: hash::Converter<D>>(
    input: &[C::Output],
    output: &ocl::Buffer<OpenclOutput>,
    options: &options::Decrypt,
    salted_prefix: &str,
    kernel_parameters: &KernelParameters,
) -> Vec<summary::Decrypted> {
    let mut cracked = vec![OpenclOutput::default(); output.len()];
    output.read(&mut cracked).enq().unwrap_or_else(|err| {
        eprintln!("OpenCL: Failed to read output buffer: {}", err);
        std::process::exit(-1);
    });

    let mut results = Vec::with_capacity(output.len());

    for (i, plain) in cracked.iter().enumerate() {
        if plain.is_valid() {
            results.push(summary::Decrypted::new(
                input[i].to_string(),
                format!("{}{}", &options.prefix, plain.printable(&kernel_parameters)),
            ));
        }
    }

    // The kernel will output zeros if nothing is found
    // We should hash this in the CPU to make sure it doesn't match anything
    if results.len() < input.len() {
        for i in 0..kernel_parameters.iterations {
            use eytzinger::SliceExt;

            let zeros = OpenclOutput::new(0, i).printable(&kernel_parameters);
            let hash = C::digest(&salted_prefix, &zeros);

            if input.eytzinger_search(&hash).is_some() {
                let result = format!("{}{}", &options.prefix, &zeros);
                results.push(summary::Decrypted::new(hash.to_string(), result));
            }

            if results.len() == input.len() {
                break;
            }
        }
    }

    results
}

split_by_algorithm!(execute_typed);

fn execute_typed<D: digest::Digest, C: hash::Converter<D>>(
    options: &options::Decrypt,
) -> summary::Mode {
    let time = std::time::Instant::now();

    // Allowed because i32::max_value() will always fit in u64 without sign loss
    #[allow(clippy::checked_conversions)]
    {
        if (options.shared.input.len() as u64) >= (i32::max_value() as u64) {
            eprintln!("Input count too large. GPU kernel defines are fixed at i32 (2,147,483,647)");
            std::process::exit(-1);
        }
    }

    let salted_prefix = format!("{}{}", options.shared.salt, options.prefix);
    let hashes = options.input_as_eytzinger::<_, C>();

    let kernel_parameters = KernelParameters::from(options);
    let (device, context, queue) = setup_opencl();
    let program = make_program(
        &salted_prefix,
        options,
        &kernel_parameters,
        &context,
        device,
    );

    let input = ocl::Buffer::builder()
        .flags(ocl::MemFlags::READ_ONLY)
        .len(options.shared.input.len())
        .queue(queue.clone())
        .copy_host_slice(&hashes)
        .build()
        .unwrap_or_else(|err| {
            eprintln!("OpenCL: Failed to create input buffer: {}", err);
            std::process::exit(-1);
        });

    let output = ocl::Buffer::builder()
        .flags(ocl::MemFlags::WRITE_ONLY)
        .len(options.shared.input.len())
        .queue(queue.clone())
        .build()
        .unwrap_or_else(|err| {
            eprintln!("OpenCL: Failed to create output buffer: {}", err);
            std::process::exit(-1);
        });

    for i in 0..kernel_parameters.iterations {
        let kernel = ocl::Kernel::builder()
            .program(&program)
            .name("crack")
            .queue(queue.clone())
            .global_work_size(kernel_parameters.range)
            .arg(&input)
            .arg(&output)
            .arg(i)
            .build()
            .unwrap_or_else(|err| {
                eprintln!("OpenCL: Failed to build kernel: {}", err);
                std::process::exit(-1);
            });

        unsafe {
            kernel.enq().unwrap_or_else(|err| {
                eprintln!("OpenCL: Failed to enqueue kernel: {}", err);
                std::process::exit(-1);
            });
        }

        // If we enqueue too many, OpenCL will abort
        // Send every 7th iteration
        if i & 0b111 == 0b111 {
            queue.finish().unwrap_or_else(|err| {
                eprintln!("OpenCL: Failed to flush queue: {}", err);
                std::process::exit(-1);
            });
        }
    }

    queue.finish().unwrap_or_else(|err| {
        eprintln!("OpenCL: Failed to wait for queue to finish: {}", err);
        std::process::exit(-1);
    });

    let results = compute_results::<_, C>(
        &hashes,
        &output,
        &options,
        &salted_prefix,
        &kernel_parameters,
    );

    if !results.is_empty() {
        if hashes.len() == 1 {
            println!("{}", results[0].plain);
        } else {
            for result in &results {
                println!("{} :: {}", result.hash, result.plain);
            }
        }
    }

    // Allowed because range is always positive
    #[allow(clippy::cast_sign_loss)]
    summary::Mode::Decrypt(summary::Decrypt {
        total_count: hashes.len(),
        duration: time.elapsed(),
        hash_count: options.number_space,
        thread_count: kernel_parameters.range as u32,
        results,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_prefix_injection() {
        let src = r#"One line
        Another line
        // %%PREFIX%%
        // %%PREFIX%% 
        Final line"#;

        let expected = r#"One line
        Another line
value.bytes[0] = '0';value.bytes[1] = '1';value.bytes[2] = '2';
        // %%PREFIX%% 
        Final line
"#;

        let output = inject_prefix("012", src);
        assert_eq!(output, expected);
    }
}
