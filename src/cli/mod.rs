use crate::decrypt;
use crate::encrypt;
use crate::options;
use crate::summary;

pub mod print;

pub fn run() -> ! {
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

    let exit_code = run_with_exit_code();
    std::process::exit(exit_code);
}

fn run_with_exit_code() -> i32 {
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
