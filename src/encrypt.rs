use crate::hash;
use crate::options;
use crate::summary;

pub fn execute(options: &options::Encrypt) -> summary::Mode {
    for input in &options.shared.input {
        let result = match &options.shared.algorithm {
            options::Algorithm::MD5 => hash::compute::<md5::Md5>(&options.shared.salt, &input),
            options::Algorithm::SHA256 => {
                hash::compute::<sha2::Sha256>(&options.shared.salt, &input)
            }
        };
        if options.shared.input.len() == 1 {
            println!("{:x}", &result);
        } else {
            println!("{} :: {:x}", &input, &result);
        }
    }

    summary::Mode::Encrypt()
}
