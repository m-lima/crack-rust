use super::{Algorithm, Decrypt, Device, Encrypt, Mode, Shared, Verboseness};

use clap::Clap;

#[derive(Clap, Debug)]
#[clap(
    name = "Hasher",
    version = "0.3",
    author = "Marcelo Lima",
    about = "MD5 and SHA256 hasher/cracker"
)]
pub enum RawMode {
    Encrypt(RawEncrypt),
    Decrypt(RawDecrypt),
}

#[derive(Clap, Debug)]
pub struct RawShared {
    #[clap(short, long, about = "Salt to use")]
    salt: Option<String>,

    #[clap(short, long, about = "Verbose", parse(from_occurrences))]
    verbose: u8,

    #[clap(short, long, about = "Hashing algorithm", default_value = "SHA256", parse(try_from_str = to_algorithm))]
    algorithm: Algorithm,

    #[clap(short, long, about = "Input values")]
    input: Vec<String>,
}

#[derive(Clap, Debug)]
pub struct RawEncrypt {
    #[clap(flatten)]
    shared: RawShared,
}

#[derive(Clap, Debug)]
pub struct RawDecrypt {
    #[clap(flatten)]
    shared: RawShared,

    #[clap(short, long, about = "Input files", parse(try_from_str = to_path))]
    files: Vec<std::path::PathBuf>,

    #[clap(short, long, about = "Known prefix of hashed values")]
    prefix: Option<String>,

    #[clap(
        short,
        long,
        about = "Number of threads to spawn (0 for auto)",
        default_value = "0"
    )]
    thread_count: u8,

    #[clap(short, long, about = "Device to run in (auto-detection if omitted)", parse(try_from_str = to_device))]
    device: Option<Device>,

    #[clap(short, long, about = "Length of hashed values", default_value = "12")]
    length: u8,
}

impl std::convert::Into<Mode> for RawMode {
    fn into(self) -> Mode {
        match self {
            Self::Encrypt(encrypt) => Mode::Encrypt(Encrypt {
                shared: encrypt.shared.into(),
            }),
            Self::Decrypt(decrypt) => {
                let prefix = if let Some(prefix) = decrypt.prefix {
                    prefix
                } else {
                    String::new()
                };

                let total_length = decrypt.length;
                if prefix.len() > usize::from(total_length) {
                    panic!("Prefix is too long");
                }

                // Allowed because the length was checked for overflow
                #[allow(clippy::cast_possible_truncation)]
                let length = total_length - prefix.len() as u8;

                let number_space = 10_u64.pow(u32::from(length));

                let thread_count = get_optimal_thread_count(decrypt.thread_count, number_space);

                let device = if let Some(device) = decrypt.device {
                    device
                } else if number_space > u64::from(thread_count) * super::OPTIMAL_HASHES_PER_THREAD
                {
                    Device::GPU
                } else {
                    Device::CPU
                };

                Mode::Decrypt(Decrypt {
                    shared: decrypt.shared.into(),
                    files: decrypt.files.into_iter().collect(),
                    length,
                    thread_count,
                    number_space,
                    prefix,
                    device,
                })
            }
        }
    }
}

impl std::convert::Into<Shared> for RawShared {
    fn into(self) -> Shared {
        Shared {
            input: self.input.into_iter().collect(),
            verboseness: match self.verbose {
                0 => Verboseness::None,
                1 => Verboseness::Low,
                _ => Verboseness::High,
            },
            algorithm: self.algorithm,
            salt: if let Some(salt) = self.salt {
                salt
            } else {
                String::from(crate::secrets::SALT)
            },
        }
    }
}

// Allowed because the count was checked for overflow
#[allow(clippy::cast_possible_truncation)]
fn get_optimal_thread_count(requested_count: u8, number_space: u64) -> u8 {
    let thread_count = std::cmp::min(
        number_space / super::OPTIMAL_HASHES_PER_THREAD + 1,
        if requested_count == 0 {
            let cores = num_cpus::get();
            if cores > usize::from(u8::max_value()) {
                panic!("Too many cores.. You have one powerful computer!");
            }
            cores as u64
        } else {
            u64::from(requested_count)
        },
    );

    // Due to `min`, it will always be less than u8::MAX (255)
    thread_count as u8
}

fn to_path(value: &str) -> Result<std::path::PathBuf, ParseError> {
    let path = std::path::PathBuf::from(value);
    if path.exists() && path.is_file() {
        Ok(path)
    } else {
        Err(ParseError::new(format!("'{}' is not a file", value)))
    }
}

fn to_algorithm(value: &str) -> Result<Algorithm, ParseError> {
    match value.to_uppercase().as_str() {
        "MD5" => Ok(Algorithm::MD5),
        "SHA256" => Ok(Algorithm::SHA256),
        _ => Err(ParseError::new(String::from(
            "possible values are [MD5, SHA256]",
        ))),
    }
}

fn to_device(value: &str) -> Result<Device, ParseError> {
    match value.to_uppercase().as_str() {
        "CPU" => Ok(Device::CPU),
        "GPU" => Ok(Device::GPU),
        _ => Err(ParseError::new(String::from(
            "possible values are [CPU, GPU]",
        ))),
    }
}

#[derive(Debug)]
struct ParseError {
    message: String,
}

impl ParseError {
    fn new(message: String) -> Self {
        Self { message }
    }
}

impl std::error::Error for ParseError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.message)
    }
}
