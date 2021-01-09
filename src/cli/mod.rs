use crate::decrypt;
use crate::encrypt;
use crate::files;
use crate::hash;
use crate::options;

mod args;
mod print;

pub fn run() {
    setup_panic();

    match args::algorithm() {
        hash::Algorithm::sha256 => run_algorithm(args::parse_sha256()),
        hash::Algorithm::md5 => run_algorithm(args::parse_md5()),
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

fn run_algorithm<H: hash::Hash>((options, printer): (options::Mode<H>, print::Printer)) {
    printer.options(&options);

    match &options {
        options::Mode::Encrypt(options) => encrypt::execute(options, printer),
        options::Mode::Decrypt(options) => decrypt(options, printer),
    }
}

fn decrypt<H: hash::Hash>(options: &options::Decrypt<H>, printer: print::Printer) {
    let summary = decrypt::execute(options, printer).unwrap();

    printer.clear_progress();
    printer.summary(&summary);

    if !options.files().is_empty() {
        printer.files();
        for file in options.files() {
            printer.write_start(file.display().to_string());
            printer.write_done(files::write(H::regex(), file, &summary));
        }
    }

    if summary.results.len() < summary.total_count {
        std::process::exit(-1);
    }
}
