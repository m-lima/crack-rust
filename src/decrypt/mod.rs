use crate::hash;
use crate::options;
use crate::summary;

mod cpu;
mod gpu;
mod opencl;

pub static OPTIMAL_HASHES_PER_THREAD: u64 = 1024 * 16;

pub fn execute<H: hash::Hash>(options: &options::Decrypt<H>) -> summary::Summary {
    match options.device() {
        options::Device::GPU => gpu::execute(options),
        options::Device::CPU => cpu::execute(options),
    }
}
