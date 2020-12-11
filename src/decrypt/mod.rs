use crate::files;
use crate::hash;
use crate::options;
use crate::summary;

mod cpu;
mod gpu;
mod opencl;

use options::SharedAccessor;

pub static OPTIMAL_HASHES_PER_THREAD: u64 = 1024 * 16;

pub fn execute<H: hash::Hash>(options: &options::Decrypt<H>) -> summary::Summary {
    let summary = match options.device() {
        options::Device::GPU => gpu::execute(options),
        options::Device::CPU => cpu::execute(options),
    };

    if !options.files().is_empty() {
        options.printer().files();
        files::write(options, &summary, options.printer());
    }

    summary
}
