use crate::hash;
use crate::options;
use crate::summary;

static MAX_GPU_LENGTH: u8 = 9;
static MAX_GPU_RANGE: i32 = 1_000_000_000; // 10 ^ MAX_LENGTH

struct KernelParameters {
    iterations: i32,
    range: i32,
    length_on_cpu: u8,
}

impl KernelParameters {
    fn cpu_length_definition(&self) -> &'static str {
        if self.iterations > 1 {
            "CONST_LENGTH_ON_CPU"
        } else {
            "INVALID_VALUE"
        }
    }
}

fn get_source_for<'a>(algorithm: &options::Algorithm) -> &'a str {
    match algorithm {
        options::Algorithm::MD5 => super::sources::MD5,
        options::Algorithm::SHA256 => super::sources::SHA256,
    }
}

fn inject_prefix(prefix: &str, src: &str) -> String {
    let mut injected_code = String::new();
    for (i, c) in prefix.chars().enumerate() {
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

fn derive_kernel_parameters(options: &options::Decrypt) -> KernelParameters {
    let length_on_cpu = if MAX_GPU_LENGTH > options.length {
        0
    } else {
        MAX_GPU_LENGTH - options.length
    };
    let iterations = 10_i32.pow(length_on_cpu as u32);

    // Allowed because min(MAX_GPU_RANGE, ...) will always fit in i32
    // Allowed because MAX_GPU_RANGE is positive
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let range = std::cmp::min(MAX_GPU_RANGE as u64, options.number_space) as i32;

    KernelParameters {
        iterations,
        length_on_cpu,
        range,
    }
}

split_by_algorithm!(execute_typed);

fn execute_typed<D: digest::Digest, C: hash::Converter<D>>(
    options: &options::Decrypt,
) -> summary::Mode {
    // Allowed because i32::max_value() will always fit in u64 without sign loss
    #[allow(clippy::checked_conversions)]
    {
        if (options.shared.input.len() as u64) >= (i32::max_value() as u64) {
            eprintln!("Input count too large. GPU kernel defines are fixed at i32 (2,147,483,647)");
            std::process::exit(-1);
        }
    }

    let kernel_parameters = derive_kernel_parameters(options);
    let hashes = options.input_as_eytzinger::<_, C>();
    let source = inject_prefix(&options.prefix, get_source_for(&options.shared.algorithm));

    let context = ocl::Context::builder()
        .devices(
            ocl::Device::specifier()
                .type_flags(ocl::flags::DEVICE_TYPE_GPU)
                .first(),
        )
        .build()
        .unwrap_or_else(|err| {
            eprintln!("OpenCL: Failed to create context: {}", err);
            std::process::exit(-1);
        });
    let device = context.devices()[0];
    let queue = ocl::Queue::new(&context, device, None).unwrap();

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
        //        .copy_host_slice(&results)
        .build()
        .unwrap_or_else(|err| {
            eprintln!("OpenCL: Failed to create output buffer: {}", err);
            std::process::exit(-1);
        });

    // Allowed because of previous check for options.shared.input.len() <= i32.max_value()
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    let program = ocl::Program::builder()
        .source(source)
        .devices(device)
        .cmplr_def("CONST_BEGIN", i32::from(options.prefix_length()))
        .cmplr_def(
            "CONST_END",
            i32::from(options.prefix_length() + options.length),
        )
        .cmplr_def("CONST_TARGET_COUNT", options.shared.input.len() as i32)
        .cmplr_def(
            kernel_parameters.cpu_length_definition(),
            i32::from(kernel_parameters.length_on_cpu),
        )
        .build(&context)
        .unwrap_or_else(|err| {
            eprintln!("OpenCL: Failed to build program: {}", err);
            std::process::exit(-1);
        });

    for i in 0..kernel_parameters.iterations {
        println!("Running iteration {}", i);
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
    }

    queue.finish().unwrap_or_else(|err| {
        eprintln!("OpenCL: Failed to flush queue: {}", err);
        std::process::exit(-1);
    });

    let mut results = vec![0_u64; output.len()];
    output.read(&mut results).enq().unwrap_or_else(|err| {
        eprintln!("OpenCL: Failed to read output buffer: {}", err);
        std::process::exit(-1);
    });

    eprintln!("{:?}", results);
    super::cpu::execute(options)
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
