use crate::hash;
use crate::options;
use crate::summary;

use crate::options::SharedAccessor;

fn execute_typed<C: hash::Converter>(options: &options::Encrypt) -> summary::Mode {
    for input in options.input() {
        if options.input().len() == 1 {
            println!("{:x}", C::digest(&options.salt(), &input));
        } else {
            println!("{}:{:x}", &input, C::digest(&options.salt(), &input));
        }
    }

    summary::Mode::Encrypt()
}

pub fn execute(options: &options::Encrypt) -> summary::Mode {
    match &options.algorithm() {
        options::Algorithm::MD5 => execute_typed::<hash::Md5>(options),
        options::Algorithm::SHA256 => execute_typed::<hash::Sha256>(options),
    }
}
