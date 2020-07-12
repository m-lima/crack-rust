// use crate::error;
use crate::hash;
use crate::options;
use crate::summary;
use crate::Input;

macro_rules! section {
    ($title:literal) => {
        eprintln!();
        eprintln!($title);
        eprintln!("----------");
    };
}

pub fn setup(options: &options::Mode) {
    let verboseness = options.verboseness() as u8;
    if verboseness > 1 {
        mode_options(&options);
        input(&options);
    }
    if verboseness > 0 {
        output();
    }
}

pub fn progress(progress: u32) {
    use std::io::Write;
    eprint!("\rProgress: {:02}%", progress);
    let _ = std::io::stderr().flush();
}

pub fn clear_progress() {
    use std::io::Write;
    eprint!("\r                  \r");
    let _ = std::io::stderr().flush();
}

// pub fn loading_start(verboseness: options::Verboseness, file: &str) {
//     if verboseness as u8 > 1 {
//         use std::io::Write;
//
//         eprint!("Loading '{}'.. ", file);
//         let _ = std::io::stderr().flush();
//     }
// }
//
// pub fn loading_done(verboseness: options::Verboseness, result: Result<(), error::Error>) {
//     if verboseness as u8 > 1 {
//         match result {
//             Ok(_) => eprintln!("Done"),
//             Err(e) => eprintln!("Fail: {}", e),
//         }
//     }
// }

pub fn summary(verboseness: options::Verboseness, summary: &summary::Mode) {
    if verboseness as u8 > 0 {
        if let summary::Mode::Decrypt(summary) = summary {
            section!("Summary");
            eprintln!(
                "{:21}{}",
                "Threads launched:",
                number(u64::from(summary.threads))
            );
            duration("Time elapsed:", 21, &summary.duration);
            eprintln!("{:21}{}", "Hashes:", number(summary.hash_count));
            if summary.duration.as_micros() == 0 {
                eprintln!("Hashes per millisec: NaN");
            } else {
                // Allowed because division by micros will not go over u64::max_value()
                #[allow(clippy::cast_possible_truncation)]
                {
                    eprintln!(
                        "Hashes per millisec: {}",
                        number(
                            ((u128::from(summary.hash_count) * 1_000)
                                / summary.duration.as_micros()) as u64
                        )
                    );
                }
            };
            eprintln!(
                "{:21}{}/{} ({}%)",
                "Values found:",
                summary.results.len(),
                summary.total_count,
                summary.results.len() * 100 / summary.total_count
            );
        }
    }
}

fn mode_options(options: &options::Mode) {
    section!("Options");
    match options {
        options::Mode::Encrypt(options) => encrypt_options(options),
        options::Mode::EncryptMd5(options) => encrypt_options(options),
        options::Mode::Decrypt(options) => decrypt_options(options),
        options::Mode::DecryptMd5(options) => decrypt_options(options),
    }

    eprintln!();
}

fn shared_options<T: Input, O: options::SharedAccessor<T>>(options: &O, algorithm: &str) {
    eprintln!("{:15}{}", "Algorithm:", algorithm);
    if !options.salt().is_empty() {
        eprintln!("{:15}{}", "Salt:", options.salt());
    }
}

fn encrypt_options<H: hash::Hash>(options: &options::Encrypt<H>) {
    shared_options(options, H::name());
}

fn decrypt_options<H: hash::Hash>(options: &options::Decrypt<H>) {
    shared_options(options, H::name());
    eprintln!("{:15}{}", "Device:", options.device());
    if options::Device::CPU == options.device() {
        eprintln!(
            "{:15}{}",
            "Threads:",
            if options.threads() == 0 {
                String::from("Auto")
            } else {
                format!("{}", options.threads())
            }
        );
    }
    eprintln!("{:15}{}", "Prefix:", options.prefix());
    eprintln!(
        "{:15}{}",
        "Length:",
        options.length() + options.prefix_length()
    );
    eprintln!("{:15}{}", "Possibilities:", number(options.number_space()));
}

// Allowed because all casts are prepended with check
#[allow(clippy::cast_precision_loss)]
fn number(number: u64) -> String {
    if number < 1000 {
        format!("{}", number)
    } else if number < 1_000_000 {
        let fraction = number as f32 / 1000_f32;
        format!("{} thousand", fraction)
    } else if number < 1_000_000_000 {
        let fraction = (number / 1000) as f32 / 1000_f32;
        format!("{} million", fraction)
    } else if number < 1_000_000_000_000 {
        let fraction = (number / 1_000_000) as f32 / 1000_f32;
        format!("{} billion", fraction)
    } else if number < 1_000_000_000_000_000 {
        let fraction = (number / 1_000_000_000) as f32 / 1000_f32;
        format!("{} trillion", fraction)
    } else {
        format!("{}", number)
    }
}

fn duration(prefix: &str, width: usize, duration: &std::time::Duration) {
    let millis = duration.as_millis();
    eprint!("{:1$}", prefix, width);

    {
        let minutes = millis / 60_000;
        if minutes > 0 {
            eprint!("{}m ", minutes);
        }
    }

    // Allowed because modulo 60000 is never grater than u16::MAX (65,536)
    #[allow(clippy::cast_possible_truncation)]
    let seconds = {
        let seconds = f32::from((millis % 60_000) as u16);
        seconds / 1000_f32
    };
    eprintln!("{:.2}s ({}ms)", seconds, millis);
}

fn input(options: &options::Mode) {
    use options::SharedAccessor;
    section!("Input");
    match options {
        options::Mode::Encrypt(mode) => mode.input().iter().for_each(|i| eprintln!("{}", i)),
        options::Mode::EncryptMd5(mode) => mode.input().iter().for_each(|i| eprintln!("{}", i)),
        options::Mode::Decrypt(mode) => mode.input().iter().for_each(|i| eprintln!("{}", i)),
        options::Mode::DecryptMd5(mode) => mode.input().iter().for_each(|i| eprintln!("{}", i)),
    }
}

fn output() {
    section!("Output");
}
