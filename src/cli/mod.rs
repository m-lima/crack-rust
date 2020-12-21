use crate::decrypt;
use crate::encrypt;
use crate::files;
use crate::hash;
use crate::options;

mod args;
pub mod print;

pub fn run() {
    setup_panic();

    match args::algorithm() {
        hash::Algorithm::sha256 => run_algorithm(&args::parse_sha256()),
        hash::Algorithm::md5 => run_algorithm(&args::parse_md5()),
    }
}

fn setup_panic() {
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
}

fn run_algorithm<H: hash::Hash>(options: &options::Mode<H>) {
    let printer = options.printer();

    printer.options(&options);

    match &options {
        options::Mode::Encrypt(options) => encrypt::execute(options),
        options::Mode::Decrypt(options) => decrypt(printer, options),
    }
}

fn decrypt<H: hash::Hash>(printer: print::Printer, options: &options::Decrypt<H>) {
    let summary = decrypt::execute(options);

    printer.summary(&summary);

    if !options.files().is_empty() {
        printer.files();
        files::write(options, &summary, printer);
    }

    if summary.results.len() < summary.total_count {
        std::process::exit(-1);
    }
}
