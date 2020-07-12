mod cpu;
mod gpu;
mod opencl;
mod summary_writer;

use crate::hash::Hash;
use crate::options;
use crate::summary;

pub fn execute<H: Hash>(options: &options::Decrypt<H>) -> summary::Mode {
    let summary = match options.device() {
        options::Device::GPU => gpu::execute(options),
        options::Device::CPU => cpu::execute(options),
    };

    summary_writer::export(options, &summary);

    summary::Mode::Decrypt(summary)
}
