extern crate md5;
extern crate sha2;

use super::options;
use super::summary;

pub struct Result {
    input: String,
    output: String,
}

impl Result {
    fn md5(prefix: &String, input: &String) -> Self {
        use md5::Digest;
        let value = format!("{}{}", prefix, input);
        Result {
            input: input.clone(),
            output: format!("{:x}", md5::Md5::digest(value.as_bytes())),
        }
    }

    fn sha256(prefix: &String, input: &String) -> Self {
        use sha2::Digest;
        let value = format!("{}{}", prefix, input);
        Result {
            input: input.clone(),
            output: format!("{:x}", sha2::Sha256::digest(value.as_bytes())),
        }
    }
}

pub fn execute(options: options::Encrypt) -> summary::Variant {
    for input in &options.shared.input {
        let result = match options.shared.algorithm {
            options::Algorithm::MD5 => Result::md5(&options.shared.salt, input),
            options::Algorithm::SHA256 => Result::sha256(&options.shared.salt, input),
        };
        if options.shared.input.len() == 1 {
            println!("{}", result.output);
        } else {
            println!("{} :: {}", result.input, result.output);
        }
    }

    summary::Variant::Encrypt()
}
