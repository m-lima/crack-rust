use crate::hash;
use crate::options;
use crate::summary;

pub fn execute(options: &options::Encrypt) -> summary::Mode {
    for input in &options.shared.input {
        match &options.shared.algorithm {
            options::Algorithm::MD5 => {
                let result = hash::compute::<hash::h128::Hash>(&options.shared.salt, &input);
                if options.shared.input.len() == 1 {
                    println!("{:x}", &result);
                } else {
                    println!("{} :: {:x}", &input, &result);
                }
            }
            options::Algorithm::SHA256 => {
                let result = hash::compute::<hash::h256::Hash>(&options.shared.salt, &input);
                if options.shared.input.len() == 1 {
                    println!("{:x}", &result);
                } else {
                    println!("{} :: {:x}", &input, &result);
                }
            }
        }
    }

    summary::Mode::Encrypt()
}
