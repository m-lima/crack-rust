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
    let channel: channel::Channel = printer.into();

    if let Err(err) = ctrlc::set_handler(move || {
        channel::cancel();
    }) {
        eprintln!("Failed to capture SIGINT: {}", err);
        eprintln!("CTRL + C will not interrupt the threads");
    }

    channel.options(&options);

    match &options {
        options::Mode::Encrypt(options) => encrypt::execute(options, channel),
        options::Mode::Decrypt(options) => decrypt(options, channel),
    }
}

fn decrypt<H: hash::Hash>(options: &options::Decrypt<H>, channel: channel::Channel) {
    let summary = decrypt::execute(options, channel).unwrap();

    channel.clear_progress();
    channel.summary(&summary);

    if !options.files().is_empty() {
        channel.files();
        for file in options.files() {
            channel.write_start(file.display().to_string());
            channel.write_done(files::write(H::regex(), file, &summary));
        }
    }

    if summary.results.len() < summary.total_count {
        std::process::exit(-1);
    }
}
