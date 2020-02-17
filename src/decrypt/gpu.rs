use crate::hash;
use crate::options;
use crate::summary;

static _MAX_RANGE: i32 = 1_000_000_000;

fn get_source_for<'a>(algorithm: &options::Algorithm) -> &'a str {
    match algorithm {
        options::Algorithm::MD5 => super::sources::MD5,
        options::Algorithm::SHA256 => super::sources::SHA256,
    }
}

split_by_algorithm!(execute_typed);

// TODO #[split_algorithm]
fn execute_typed<D: digest::Digest, C: hash::Converter<D>>(
    options: &options::Decrypt,
) -> summary::Mode {
    let source = get_source_for(&options.shared.algorithm);

    let hashes = options
        .shared
        .input
        .iter()
        .map(String::as_str)
        .map(C::from_str)
        .collect::<Vec<_>>();

    let context = ocl::Context::builder().build().unwrap_or_else(|err| {
        eprintln!("Failed to create context: {}", err);
        std::process::exit(-1);
    });

    let _input = ocl::Buffer::builder()
        .flags(ocl::MemFlags::READ_ONLY)
        .len(options.shared.input.len())
        .copy_host_slice(&hashes)
        .context(&context)
        .build()
        .unwrap_or_else(|err| {
            eprintln!("Failed to create input buffer: {}", err);
            std::process::exit(-1);
        });

    let output = ocl::Buffer::builder()
        .flags(ocl::MemFlags::WRITE_ONLY)
        .len(options.shared.input.len())
        .context(&context)
        .build()
        .unwrap_or_else(|err| {
            eprintln!("Failed to create output buffer: {}", err);
            std::process::exit(-1);
        });

    if (options.shared.input.len() as u64) < (i32::max_value() as u64) {
        eprintln!("Input count too large. GPU kernel defines are fixed at i32 (2,147,483,647)");
        std::process::exit(-1);
    }

    // Allowed because of previous assert
    #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
    let _program = ocl::Program::builder()
        .source(source)
        .cmplr_def("CONST_BEGIN", 4)
        .cmplr_def("CONST_END", 8)
        .cmplr_def("CONST_SUFFIX", 4000)
        .cmplr_def("CONST_PREFIX_DECIMAL_PLACES", 4)
        .cmplr_def("CONST_TARGET_COUNT", options.shared.input.len() as i32)
        .build(&context)
        .unwrap_or_else(|err| {
            eprintln!("Failed to build kernel: {}", err);
            std::process::exit(-1);
        });

    let mut results = vec![0_u64; output.len()];
    output.read(&mut results).enq().unwrap_or_else(|err| {
        eprintln!("Failed to read output buffer: {}", err);
        std::process::exit(-1);
    });
    super::cpu::execute(options)
}
