use crate::hash::Hash;
use crate::options;

use crate::options::SharedAccessor;

pub fn execute<H: Hash>(options: &options::Encrypt<H>) {
    for input in options.input() {
        if options.input().len() == 1 {
            println!("{:x}", H::digest(&options.salt(), &input));
        } else {
            println!("{}:{:x}", &input, H::digest(&options.salt(), &input));
        }
    }
}
