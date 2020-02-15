mod cpu;
mod gpu;

use crate::options;
use crate::summary;

pub fn execute(options: &options::Decrypt) -> summary::Mode {
    match options.core {
        options::Core::GPU => gpu::execute(options),
        options::Core::CPU => cpu::execute(options),
    }
}
