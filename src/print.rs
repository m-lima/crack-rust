use crate::{error::Error, options, summary};

macro_rules! no_verbose_gate {
    ($verboseness:ident) => {
        if let options::Verboseness::None = $verboseness {
            return;
        }
    };
}

macro_rules! high_verbose_gate {
    ($verboseness:ident) => {
        if let options::Verboseness::High = $verboseness {
        } else {
            return;
        }
    };
}

macro_rules! separator {
    () => {
        "----------"
    };
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

fn separator() {
    eprintln!(separator!());
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

pub fn loading_start(verboseness: options::Verboseness, file: &str) {
    use std::io::Write;

    no_verbose_gate!(verboseness);
    eprint!("Loading '{}'.. ", file);
    let _ = std::io::stderr().flush();
}

pub fn loading_done(verboseness: options::Verboseness, result: Result<(), Error>) {
    no_verbose_gate!(verboseness);
    match result {
        Ok(_) => eprintln!("Done"),
        Err(e) => eprintln!("Fail: {}", e),
    }
}

pub fn input<'a>(
    verboseness: options::Verboseness,
    input: impl std::iter::Iterator<Item = &'a String>,
) {
    high_verbose_gate!(verboseness);

    eprintln!("Input:");
    separator();
    input.for_each(|i| eprintln!("{}", i));
}

fn shared_options<O: options::SharedAccessor>(options: &O) {
    eprintln!("{:15}{}", "Algorithm:", options.algorithm());
    if !options.salt().is_empty() {
        eprintln!("{:15}{}", "Salt:", options.salt());
    }
}

fn encrypt_options(options: &options::Encrypt) {
    shared_options(options);
}

fn decrypt_options(options: &options::Decrypt) {
    shared_options(options);
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

pub fn options(verboseness: options::Verboseness, options: &options::Mode) {
    high_verbose_gate!(verboseness);

    eprintln!();
    eprintln!("Options:");
    separator();
    match options {
        options::Mode::Encrypt(options) => encrypt_options(options),
        options::Mode::Decrypt(options) => decrypt_options(options),
    }

    eprintln!();
}

pub fn output(verboseness: options::Verboseness) {
    no_verbose_gate!(verboseness);
    eprintln!();
    eprintln!("Output:");
    separator();
}

pub fn summary(verboseness: options::Verboseness, summary: &summary::Mode) {
    no_verbose_gate!(verboseness);
    if let summary::Mode::Decrypt(summary) = summary {
        eprintln!();
        eprintln!("Summary:");
        separator();
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
                        ((u128::from(summary.hash_count) * 1_000) / summary.duration.as_micros())
                            as u64
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
