#![deny(warnings, clippy::pedantic)]
#![warn(rust_2018_idioms)]

#[macro_use]
mod error;

mod decrypt;
mod encrypt;
mod files;
mod hash;
mod options;
mod print;
mod secrets;
mod summary;

pub static SALT_ENV: &str = "HASHER_SALT";

pub trait Input:
    'static + std::hash::Hash + std::fmt::Display + ToString + PartialEq + Eq + PartialOrd + Ord
{
}
impl Input for String {}

fn run() -> i32 {
    let options = options::parse();

    options.printer().options(&options);

    let summary = match &options {
        options::Mode::Encrypt(options) => encrypt::execute(options),
        options::Mode::EncryptMd5(options) => encrypt::execute(options),
        options::Mode::Decrypt(options) => decrypt::execute(options),
        options::Mode::DecryptMd5(options) => decrypt::execute(options),
    };

    options.printer().summary(&summary);

    if let summary::Mode::Decrypt(decrypt) = summary {
        if decrypt.results.len() < options.input_len() {
            -1
        } else {
            0
        }
    } else {
        0
    }
}

fn main() {
    std::panic::set_hook(Box::new(|info| {
        use colored::Colorize;
        let payload = info.payload();
        if let Some(message) = payload.downcast_ref::<&str>() {
            eprintln!("{} {}", "Error:".bright_red(), message);
            return;
        }
        if let Some(message) = payload.downcast_ref::<String>() {
            eprintln!("{} {}", "Error:".bright_red(), message);
            return;
        }
        eprintln!("{} unhandled exception", "Error:".bright_red());
    }));

    std::process::exit(run());
}
