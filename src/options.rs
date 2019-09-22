extern crate clap;

use clap::arg_enum;

pub enum Verboseness {
    None,
    Low,
    High,
}

clap::arg_enum! {
    #[derive(PartialEq, Debug, Clone)]
    pub enum Algorithm {
        MD5 = 32,
        SHA256 = 64,
    }
}

pub struct Shared {
    pub verboseness: Verboseness,
    pub input: Vec<String>,
    pub algorithm: Algorithm,
    pub salt: String,
}

impl std::fmt::Display for Shared {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Verboseness::None = self.verboseness {
            Ok(())
        } else {
            writeln!(fmt, "Algorithm: {}", self.algorithm)?;

            if let Verboseness::Low = self.verboseness {
                write!(fmt, "Salt: {}", self.salt)
            } else {
                writeln!(fmt, "Salt: {}", self.salt)?;
                write!(
                    fmt,
                    "{}",
                    self.input
                        .iter()
                        .fold(String::from("Input:"), |prev, curr| format!(
                            "{}\n  {}",
                            prev, curr
                        ))
                )
            }
        }
    }
}

pub struct Encrypt {
    pub shared: Shared,
}

impl std::fmt::Display for Encrypt {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", self.shared)
    }
}

pub struct Decrypt {
    pub shared: Shared,
    pub length: u8,
    pub thread_count: u8,
    pub number_space: u64,
    pub prefix: String,
}

impl std::fmt::Display for Decrypt {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.shared.verboseness {
            Verboseness::None => Ok(()),
            _ => {
                writeln!(fmt, "Threads: {}", self.thread_count)?;
                writeln!(fmt, "Length: {}", self.length + self.prefix.len() as u8)?;
                writeln!(fmt, "Prefix: {}", self.prefix)?;
                write!(fmt, "{}", self.shared)
            }
        }
    }
}

pub enum Variant {
    Encrypt(Encrypt),
    Decrypt(Decrypt),
}

impl std::fmt::Display for Variant {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Variant::Encrypt(options) => write!(fmt, "{}", options),
            Variant::Decrypt(options) => write!(fmt, "{}", options),
        }
    }
}
