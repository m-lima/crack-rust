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
    reporter: impl results::Reporter,
) -> Result<results::Summary, error::Error> {
    match options.device() {
        options::Device::GPU => gpu::execute(options, reporter),
        options::Device::CPU => cpu::execute(options, reporter),
    }
}
