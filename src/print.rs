use super::options;
use super::summary;

macro_rules! no_verbose_gate {
    ($self:ident) => {
        if let Verboseness::None = $self.verboseness {
            return;
        }
    };
}

macro_rules! high_verbose_gate {
    ($self:ident) => {
        if let Verboseness::High = $self.verboseness {
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

pub enum Verboseness {
    None,
    Low,
    High,
}

pub struct Print {
    verboseness: Verboseness,
}

pub fn new(verboseness: Verboseness) -> Print {
    Print { verboseness }
}

fn separator() {
    println!(separator!());
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

    let seconds = (millis % 60_000) as f32 / 1000 as f32;
    println!("{:.2}s ({}ms)", seconds, millis);
}

impl Print {
    fn input(&self, input: &Vec<String>) {
        high_verbose_gate!(self);

        println!();
        println!("Input:");
        separator();
        for i in input {
            println!("{}", i);
        }
    }

    fn shared_options(&self, options: &options::Shared) {
        println!("{:15}{}", "Algorithm:", options.algorithm);
        if !options.salt.is_empty() {
            println!("{:15}{}", "Salt:", options.salt);
        }
    }

    fn encrypt_options(&self, options: &options::Encrypt) {
        self.shared_options(&options.shared);
        self.input(&options.shared.input);
    }

    fn decrypt_options(&self, options: &options::Decrypt) {
        self.shared_options(&options.shared);
        println!(
            "{:15}{}",
            "Threads:",
            if options.thread_count == 0 {
                String::from("Auto")
            } else {
                format!("{}", options.thread_count)
            }
        );
        println!("{:15}{}", "Prefix:", options.prefix);
        println!(
            "{:15}{}",
            "Length:",
            options.length + options.prefix.len() as u8
        );
        println!("{:15}{}", "Possibilities:", options.number_space);
        self.input(&options.shared.input);
    }

    pub fn options(&self, options: &options::Variant) {
        no_verbose_gate!(self);

        println!("Options:");
        separator();
        match options {
            options::Variant::Encrypt(options) => self.encrypt_options(options),
            options::Variant::Decrypt(options) => self.decrypt_options(options),
        }

        println!();
        println!("Output:");
        separator();
    }

    pub fn summary(&self, summary: &summary::Variant) {
        no_verbose_gate!(self);
        if let summary::Variant::Decrypt(summary) = summary {
            println!();
            println!("Summary:");
            separator();
            println!("{:19}{}", "Threads launched:", summary.thread_count);
            duration("Time elapsed:", 19,  &summary.duration);
            println!("{:19}{}", "Hashes:", summary.hash_count);
            println!(
                "Hashes per second: {}",
                summary.hash_count / summary.duration.as_millis() as u64
            );
            println!(
                "{:19}{}/{} ({}%)", "Values found:",
                summary.cracked_count,
                summary.total_count,
                summary.cracked_count * 100 / summary.total_count
            );
        }
    }
}
