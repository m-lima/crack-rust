#![deny(warnings, clippy::pedantic)]
#![warn(rust_2018_idioms)]

mod decrypt;
mod encrypt;
mod hash;
mod options;
mod print;
mod secrets;
mod summary;

use crate::options::SharedAccessor;

fn main() {
    let options = options::parse();

    print::options(options.verboseness(), &options);
    print::input(options.verboseness(), &options.input());
    print::output(options.verboseness());

    let summary = match &options {
        options::Mode::Encrypt(options) => encrypt::execute(options),
        options::Mode::Decrypt(options) => decrypt::execute(options),
    };

    print::summary(options.verboseness(), &summary);
}
