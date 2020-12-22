use crate::hash;
use crate::options;
use crate::results;

mod cpu;
mod gpu;
mod opencl;

pub static OPTIMAL_HASHES_PER_THREAD: u64 = 1024 * 16;

pub fn execute<H: hash::Hash>(
    options: &options::Decrypt<H>,
    reporter: impl results::Reporter,
) -> results::Summary {
    match options.device() {
        options::Device::GPU => gpu::execute(options, reporter),
        options::Device::CPU => cpu::execute(options, reporter),
    }
}
