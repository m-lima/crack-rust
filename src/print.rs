use crate::options;
use crate::summary;

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
    println!(separator!());
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
    print!("{:1$}", prefix, width);

    {
        let minutes = millis / 60_000;
        if minutes > 0 {
            print!("{}m ", minutes);
        }
    }

    // Allowed because modulo 60000 is never grater than u16::MAX (65,536)
    #[allow(clippy::cast_possible_truncation)]
    let seconds = {
        let seconds = f32::from((millis % 60_000) as u16);
        seconds / 1000_f32
    };
    println!("{:.2}s ({}ms)", seconds, millis);
}

pub fn input(verboseness: options::Verboseness, input: &[String]) {
    high_verbose_gate!(verboseness);

    println!();
    println!("Input:");
    separator();
    for i in input {
        println!("{}", i);
    }
}

fn shared_options<O: options::SharedAccessor>(options: &O) {
    println!("{:15}{}", "Algorithm:", options.algorithm());
    if !options.salt().is_empty() {
        println!("{:15}{}", "Salt:", options.salt());
    }
}

fn encrypt_options(options: &options::Encrypt) {
    shared_options(options);
}

fn decrypt_options(options: &options::Decrypt) {
    shared_options(options);
    println!("{:15}{}", "Device:", options.device());
    if options::Device::CPU == options.device() {
        println!(
            "{:15}{}",
            "Threads:",
            if options.thread_count() == 0 {
                String::from("Auto")
            } else {
                format!("{}", options.thread_count())
            }
        );
    }
    println!("{:15}{}", "Prefix:", options.prefix());
    println!(
        "{:15}{}",
        "Length:",
        options.length() + options.prefix_length()
    );
    println!("{:15}{}", "Possibilities:", number(options.number_space()));
}

pub fn options(verboseness: options::Verboseness, options: &options::Mode) {
    high_verbose_gate!(verboseness);

    println!("Options:");
    separator();
    match options {
        options::Mode::Encrypt(options) => encrypt_options(options),
        options::Mode::Decrypt(options) => decrypt_options(options),
    }

    println!();
}

pub fn output(verboseness: options::Verboseness) {
    no_verbose_gate!(verboseness);
    println!("Output:");
    separator();
}

pub fn summary(verboseness: options::Verboseness, summary: &summary::Mode) {
    no_verbose_gate!(verboseness);
    if let summary::Mode::Decrypt(summary) = summary {
        println!();
        println!("Summary:");
        separator();
        println!(
            "{:21}{}",
            "Threads launched:",
            number(u64::from(summary.thread_count))
        );
        duration("Time elapsed:", 21, &summary.duration);
        println!("{:21}{}", "Hashes:", number(summary.hash_count));
        if summary.duration.as_micros() == 0 {
            println!("Hashes per millisec: NaN");
        } else {
            // Allowed because division by micros will not go over u64::max_value()
            #[allow(clippy::cast_possible_truncation)]
            {
                println!(
                    "Hashes per millisec: {}",
                    number(
                        ((u128::from(summary.hash_count) * 1_000) / summary.duration.as_micros())
                            as u64
                    )
                );
            }
        };
        println!(
            "{:21}{}/{} ({}%)",
            "Values found:",
            summary.results.len(),
            summary.total_count,
            summary.results.len() * 100 / summary.total_count
        );
    }
}
