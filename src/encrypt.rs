use crate::hash;
use crate::options;
use crate::summary;

fn execute_typed<D: digest::Digest, C: hash::Converter<D>>(
    options: &options::Encrypt,
) -> summary::Mode {
    for input in &options.shared.input {
        if options.shared.input.len() == 1 {
            println!("{:x}", C::digest(&options.shared.salt, &input));
        } else {
            println!(
                "{} :: {:x}",
                &input,
                C::digest(&options.shared.salt, &input)
            );
        }
    }

    summary::Mode::Encrypt()
}

pub fn execute(options: &options::Encrypt) -> summary::Mode {
    match &options.shared.algorithm {
        options::Algorithm::MD5 => execute_typed::<_, hash::Md5>(options),
        options::Algorithm::SHA256 => execute_typed::<_, hash::Sha256>(options),
    }
}
