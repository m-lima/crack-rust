macro_rules! split_by_algorithm {
    ($func:ident) => {
        pub(super) fn execute(options: &crate::options::Decrypt) -> summary::Decrypt {
            match options.algorithm() {
                crate::options::Algorithm::MD5 => $func::<crate::hash::Md5>(&options),
                crate::options::Algorithm::SHA256 => $func::<crate::hash::Sha256>(&options),
            }
        }
    };
}

mod cpu;
mod gpu;
mod opencl;
mod summary_writer;

use crate::options;
use crate::summary;

pub fn execute(options: &options::Decrypt) -> summary::Mode {
    let summary = match options.device() {
        options::Device::GPU => gpu::execute(options),
        options::Device::CPU => cpu::execute(options),
    };

    summary_writer::export(options, &summary);

    summary::Mode::Decrypt(summary)
}
