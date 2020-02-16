mod cpu;
mod gpu;
mod sources;

use crate::options;
use crate::summary;

pub fn execute(options: &options::Decrypt) -> summary::Mode {
    match options.device {
        options::Device::GPU => gpu::execute(options),
        options::Device::CPU => cpu::execute(options),
    }
}
