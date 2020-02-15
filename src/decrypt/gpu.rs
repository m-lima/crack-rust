use crate::options;
use crate::summary;

pub(super) fn execute(options: &options::Decrypt) -> summary::Mode {
    super::cpu::execute(options)
}
