#![deny(warnings, clippy::pedantic)]
#![warn(rust_2018_idioms)]

#[macro_use]
mod error;

mod decrypt;
mod encrypt;
mod hash;
mod options;
mod print;
mod secrets;
mod summary;

use crate::options::SharedAccessor;

pub static SALT_ENV: &str = "HASHER_SALT";

fn run() -> i32 {
    let options = options::parse();

    print::options(options.verboseness(), &options);
    print::input(options.verboseness(), options.input().iter());
    print::output(options.verboseness());

    let summary = match &options {
        options::Mode::Encrypt(options) => encrypt::execute(options),
        options::Mode::Decrypt(options) => decrypt::execute(options),
    };

    print::summary(options.verboseness(), &summary);

    if let summary::Mode::Decrypt(decrypt) = summary {
        if decrypt.results.len() < options.input().len() {
            return -1;
        }
    }
    0
}

fn main() {
    std::panic::set_hook(Box::new(|info| {
        let payload = info.payload();
        if let Some(message) = payload.downcast_ref::<&str>() {
            eprintln!("{}", message);
            return;
        }
        if let Some(message) = payload.downcast_ref::<String>() {
            eprintln!("{}", message);
            return;
        }
        eprintln!("unhandled exception");
    }));

    std::process::exit(run());
}
