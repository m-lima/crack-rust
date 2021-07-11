use crate::decrypt;
use crate::encrypt;
use crate::files;
use crate::hash;
use crate::options;

mod args;
mod channel;
mod print;

pub fn run() {
    setup_panic();

    if !match args::algorithm() {
        hash::Algorithm::sha256 => run_algorithm(args::parse_sha256()),
        hash::Algorithm::md5 => run_algorithm(args::parse_md5()),
    } {
        std::process::exit(-1);
    }
}

fn setup_panic() {
    std::panic::set_hook(Box::new(|info| {
        let payload = info.payload();
        if let Some(message) = payload.downcast_ref::<&str>() {
            print_error(message);
        } else if let Some(message) = payload.downcast_ref::<String>() {
            print_error(message);
        } else {
            print_error("unhandled exception");
        }
    }));
}

fn print_error<E: std::fmt::Display>(error: E) {
    use colored::Colorize;
    eprintln!("{} {}", "Error:".bright_red(), error);
}

fn run_algorithm<H: hash::Hash>((options, printer): (options::Mode<H>, print::Printer)) -> bool {
    let channel: channel::Channel = printer.into();

    if let Err(err) = ctrlc::set_handler(move || {
        channel::cancel();
    }) {
        eprintln!("Failed to capture SIGINT: {}", err);
        eprintln!("CTRL + C will not interrupt the threads");
    }

    channel.options(&options);

    match &options {
        options::Mode::Encrypt(options) => encrypt::execute(options, &channel),
        options::Mode::Decrypt(options) => decrypt(options, channel),
    }
}

fn decrypt<H: hash::Hash>(options: &options::Decrypt<H>, channel: channel::Channel) -> bool {
    let summary = match decrypt::execute(options, &channel) {
        Ok(summary) => summary,
        Err(err) => {
            print_error(err);
            return false;
        }
    };

    channel.clear_progress();
    channel.summary(&summary);

    if !options.files().is_empty() {
        channel.files();
        for file in options.files() {
            channel.write_start(file.display().to_string());
            channel.write_done(files::write(H::regex(), file, None, &summary.results));
        }
    }

    summary.results.len() == summary.total_count
}
