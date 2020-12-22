#![deny(warnings, clippy::pedantic)]
#![warn(rust_2018_idioms)]

#[macro_use]
mod error;

mod cli;
mod decrypt;
mod encrypt;
mod files;
mod gui;
mod hash;
mod options;
mod results;
mod secrets;

pub trait Input:
    'static + std::hash::Hash + std::fmt::Display + ToString + PartialEq + Eq + PartialOrd + Ord
{
}
impl Input for String {}

fn main() {
    if std::env::args().len() > 1 {
        cli::run();
    } else {
        gui::run();
    }
}
