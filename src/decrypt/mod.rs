macro_rules! split_by_algorithm {
    ($func:ident) => {
        pub(super) fn execute(options: &crate::options::Decrypt) -> summary::Mode {
            match &options.shared.algorithm {
                crate::options::Algorithm::MD5 => $func::<_, crate::hash::Md5>(&options),
                crate::options::Algorithm::SHA256 => $func::<_, crate::hash::Sha256>(&options),
            }
        }
    };
}

mod cpu;
mod gpu;
mod opencl;

use crate::options;
use crate::summary;

pub fn execute(options: &options::Decrypt) -> summary::Mode {
    match options.device {
        options::Device::GPU => gpu::execute(options),
        options::Device::CPU => cpu::execute(options),
    }
}
