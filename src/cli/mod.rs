use crate::decrypt;
use crate::encrypt;
use crate::options;
use crate::summary;

mod args;
pub mod print;

pub fn run() {
    setup_panic();

    let options = args::parse();
    let printer = options.printer();

    printer.options(&options);

    match &options {
        options::Mode::Encrypt(options) => encrypt::execute(options),
        options::Mode::EncryptMd5(options) => encrypt::execute(options),
        options::Mode::Decrypt(options) => finalize(printer, &decrypt::execute(options)),
        options::Mode::DecryptMd5(options) => finalize(printer, &decrypt::execute(options)),
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

fn finalize(printer: print::Printer, summary: &summary::Summary) {
    printer.summary(&summary);

    if summary.results.len() < summary.total_count {
        std::process::exit(-1);
    }
}
