use crate::channel;
use crate::error;
use crate::hash;
use crate::options;
use crate::results;

mod cpu;
mod gpu;
mod opencl;

pub use cpu::OPTIMAL_HASHES_PER_THREAD;

pub fn execute<H: hash::Hash>(
    options: &options::Decrypt<H>,
    channel: &impl channel::Channel,
) -> Result<results::Summary, error::Error> {
    match options.device() {
        options::Device::Gpu => gpu::execute(options, channel),
        options::Device::Cpu => cpu::execute(options, channel),
    }
}
