use crate::hash;
use crate::options;
use crate::summary;

fn execute_typed<D: digest::Digest>(options: &options::Encrypt) -> summary::Mode {
    for input in &options.shared.input {
        //        use hash::AlgorithmConverter;
        if options.shared.input.len() == 1 {
            println!(
                "{:x}",
                32 //hash::Converter::digest(&options.shared.salt, &input)
            );
        } else {
            println!(
                "{} :: {:x}",
                &input,
                32 //hash::Converter::digest(&options.shared.salt, &input)
            );
        }
    }

    summary::Mode::Encrypt()
}

pub fn execute(options: &options::Encrypt) -> summary::Mode {
    match &options.shared.algorithm {
        options::Algorithm::MD5 => execute_typed::<md5::Md5>(options),
        options::Algorithm::SHA256 => execute_typed::<sha2::Sha256>(options),
    }
}
