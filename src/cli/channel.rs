use super::print;

use crate::channel;

static mut SHOULD_TERMINATE: bool = false;

#[derive(Copy, Clone)]
pub struct Channel(print::Printer);

impl channel::Channel for Channel {
    fn progress(&self, progress: u8) {
        self.0.progress(progress);
    }

    fn result(&self, input: &str, output: &str) {
        self.0.report(input, output);
    }

    fn should_terminate(&self) -> bool {
        unsafe { SHOULD_TERMINATE }
    }
}

pub fn cancel() {
    unsafe {
        SHOULD_TERMINATE = true;
    }
}

impl std::convert::From<print::Printer> for Channel {
    fn from(printer: print::Printer) -> Self {
        Self(printer)
    }
}

impl std::ops::Deref for Channel {
    type Target = print::Printer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
