use crate::files;
use crate::hash;
use crate::options;
use crate::summary;

mod cpu;
mod gpu;
mod opencl;

use options::SharedAccessor;

pub fn execute<H: hash::Hash>(options: &options::Decrypt<H>) -> summary::Mode {
    let summary = match options.device() {
        options::Device::GPU => gpu::execute(options),
        options::Device::CPU => cpu::execute(options),
    };

    if !options.files().is_empty() {
        options.printer().files();
        files::write(options, &summary, options.printer());
    }

    summary::Mode::Decrypt(summary)
}
