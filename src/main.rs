#![deny(warnings, clippy::pedantic)]
#![warn(rust_2018_idioms)]

#[macro_use]
mod error;

mod channel;
mod cli;
mod decrypt;
mod encrypt;
mod files;
mod hash;
mod options;
mod results;
mod secrets;

#[cfg(feature = "qt")]
mod gui;

pub trait Input:
    'static + std::hash::Hash + std::fmt::Display + ToString + PartialEq + Eq + PartialOrd + Ord
{
}
impl Input for String {}

fn main() {
    #[cfg(feature = "qt")]
    if std::env::args().len() == 1 {
        gui::run();
    }

    cli::run();
}
