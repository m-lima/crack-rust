use crate::channel;
use crate::hash;
use crate::options;

use crate::options::SharedAccessor;

pub fn execute<H: hash::Hash>(options: &options::Encrypt<H>, channel: impl channel::Channel) {
    for input in options.input() {
        channel.result(&input, &format!("{:x}", H::digest(&options.salt(), &input)));
        if channel.should_terminate() {
            break;
        }
    }
}
