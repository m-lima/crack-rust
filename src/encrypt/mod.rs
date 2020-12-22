use crate::hash;
use crate::options;
use crate::results;

use crate::options::SharedAccessor;

pub fn execute<H: hash::Hash>(options: &options::Encrypt<H>, reporter: impl results::Reporter) {
    for input in options.input() {
        reporter.report(&input, &format!("{:x}", H::digest(&options.salt(), &input)));
    }
}
