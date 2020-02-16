use crate::options;
use crate::summary;

static _MAX_RANGE: i32 = 1_000_000_000;

fn get_source_for<'a>(algorithm: &options::Algorithm) -> &'a str {
    match algorithm {
        options::Algorithm::MD5 => super::sources::MD5,
        options::Algorithm::SHA256 => super::sources::SHA256,
    }
    //    r#"
    //        __kernel void add(__global float* buffer, float scalar) {
    //            buffer[get_global_id(0)] += scalar;
    //        }
    //    "#
}

pub(super) fn execute(options: &options::Decrypt) -> summary::Mode {
    let source = get_source_for(&options.shared.algorithm);

    //    let hashes = options
    //        .shared
    //        .input
    //        .iter()
    //        .map(|h| {
    //            use crate::hash::Into;
    //            h.into_hash().unwrap_or_else(|err| {
    //                eprintln!("Failed to build build hash: {}", err);
    //                std::process::exit(-1);
    //            })
    //        })
    //        .collect::<Vec<_>>();
    //
    //    let buffer = ocl::Buffer::builder()
    //        .flags(ocl::MemFlags::READ_ONLY)
    //        .len(options.shared.input.len())
    //        .copy_host_slice(&hashes)
    //        .build()
    //        .unwrap_or_else(|err| {
    //            eprintln!("Failed to create input buffer: {}", err);
    //            std::process::exit(-1);
    //        });

    //    let program = ocl::Program::builder()
    //        .source(source)
    //        .cmplr_def("CONST_BEGIN", 4)
    //        .cmplr_def("CONST_END", 8)
    //        .cmplr_def("CONST_SUFFIX", 4000)
    //        .cmplr_def("CONST_PREFIX_DECIMAL_PLACES", 4)
    //        .cmplr_def("CONST_TARGET_COUNT", options.shared.input.len() as i32);
    let pro_que = ocl::ProQue::builder()
        .src(source)
        .dims(1 << 20)
        .build()
        .unwrap_or_else(|err| {
            eprintln!("Failed to create process queue: {}", err);
            std::process::exit(-1);
        });

    let buffer = pro_que.create_buffer::<f32>().unwrap_or_else(|err| {
        eprintln!("Failed to create process buffer: {}", err);
        std::process::exit(-1);
    });

    let kernel = pro_que
        .kernel_builder("add")
        .arg(&buffer)
        .arg(10_f32)
        .build()
        .unwrap_or_else(|err| {
            eprintln!("Failed to create kernel: {}", err);
            std::process::exit(-1);
        });

    unsafe {
        kernel.enq().unwrap_or_else(|err| {
            eprintln!("Failed to enqueue kernel: {}", err);
            std::process::exit(-1);
        });
    }

    let mut vec = vec![0_f32; buffer.len()];
    buffer.read(&mut vec).enq().unwrap_or_else(|err| {
        eprintln!("Failed to read kernel: {}", err);
        std::process::exit(-1);
    });

    println!("The value at index [200007] is now '{}'!", vec[200_007]);
    //    Ok(())
    super::cpu::execute(options)
}
