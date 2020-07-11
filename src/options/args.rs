use clap::Clap;

use crate::{error::Error, hash};

use super::{Algorithm, Decrypt, Device, Encrypt, Mode, Shared, Verboseness};

/// MD5 and SHA256 hasher/cracker
#[derive(Clap, Debug)]
#[clap(
    name = "Hasher",
    version,
    after_help = "Input can be provided through stdin or as parameters"
)]
pub enum RawMode {
    /// Generate sha256 hashes
    Hash(RawHash),
    /// Generate md5 hashes
    HashMd5(RawHash),
    /// Crack sha256 hashes
    #[clap(
        after_help = "The cracker will exit with an error if any of the input hashes could not be cracked"
    )]
    Crack(RawCrack),
    /// Crack md5 hashes
    #[clap(
        after_help = "The cracker will exit with an error if any of the input hashes could not be cracked"
    )]
    CrackMd5(RawCrackMd5),
}

#[derive(Clap, Debug)]
pub struct RawShared {
    /// Salt to prepend when generating hash [env: HASHER_SALT]
    #[clap(short, long)]
    #[allow(clippy::option_option)]
    salt: Option<Option<String>>,

    /// Verbose mode (-v, -vv)
    ///
    /// All verboseness will be printed to stderr
    #[clap(short, parse(from_occurrences))]
    verbose: u8,
}

#[derive(Clap, Debug)]
pub struct RawHash {
    #[clap(flatten)]
    shared: RawShared,

    /// Values to hash
    ///
    /// If a single input is given, only the hash will be printed to stdout. If more than one input
    /// is given, the pairs <input>:<hash> will be printed to stdout, one per line
    input: Vec<String>,
}

#[derive(Clap, Debug)]
pub struct RawCrack {
    #[clap(flatten)]
    shared: RawCrackShared,

    /// Sha256 values to crack. Expected to be the hash of a numeric value
    ///
    /// If a single hash is given, only the cracked value will be printed to stdout.
    /// If more than one hash is given, the pairs <hash>:<cracked value> will be printed to stdout,
    /// one per line
    #[clap(parse(try_from_str = <hash::Sha256 as hash::Converter>::from_str))]
    input: Vec<<hash::Sha256 as hash::Converter>::Output>,
}

#[derive(Clap, Debug)]
pub struct RawCrackMd5 {
    #[clap(flatten)]
    shared: RawCrackShared,

    /// MD5 values to crack. Expected to be the hash of a numeric value
    ///
    /// If a single hash is given, only the cracked value will be printed to stdout.
    /// If more than one hash is given, the pairs <hash>:<cracked value> will be printed to stdout,
    /// one per line
    #[clap(parse(try_from_str = <hash::Md5 as hash::Converter>::from_str))]
    input: Vec<<hash::Md5 as hash::Converter>::Output>,
}

#[derive(Clap, Debug)]
pub struct RawCrackShared {
    #[clap(flatten)]
    shared: RawShared,

    /// Input files. Will be scanned for hashes to crack
    ///
    /// If any hash from a given file is cracked, a copy of the file will be created in the same
    /// directory with the ".cracked" extension containing all cracked hashes substituted in place
    #[clap(short, long, parse(try_from_str = to_path))]
    files: Vec<std::path::PathBuf>,

    /// Known prefix of hashed values
    #[clap(short, long)]
    prefix: Option<String>,

    /// Number of threads to spawn (0 for auto)
    #[clap(short, long, default_value = "0")]
    threads: u8,

    /// Device to run in (auto-detection if omitted)
    #[clap(short, long, possible_values = Device::variants(), parse(try_from_str = to_device))]
    device: Option<Device>,

    /// Length of hashed values
    #[clap(short, long, default_value = "12")]
    length: u8,
}

impl std::convert::Into<Mode> for RawMode {
    fn into(self) -> Mode {
        match self {
            Self::Hash(encrypt) => compose_hash(encrypt.input, encrypt.shared, Algorithm::SHA256),
            Self::HashMd5(encrypt) => compose_hash(encrypt.input, encrypt.shared, Algorithm::MD5),
            Self::Crack(decrypt) => compose_crack(decrypt.shared, Algorithm::SHA256, decrypt.input),
            Self::CrackMd5(decrypt) => compose_crack(decrypt.shared, Algorithm::MD5, decrypt.input),
        }
    }
}

fn compose_hash(input: Vec<String>, shared: RawShared, algorithm: Algorithm) -> Mode {
    Mode::Encrypt(Encrypt {
        shared: shared.into(input, algorithm),
    })
}

fn compose_crack<T: std::fmt::Display>(
    shared: RawCrackShared,
    algorithm: Algorithm,
    input: Vec<T>,
) -> Mode {
    let prefix = if let Some(prefix) = shared.prefix {
        prefix
    } else {
        String::new()
    };

    let total_length = shared.length;
    if prefix.len() > usize::from(total_length) {
        panic!("Prefix is too long");
    }

    // Allowed because the length was checked for overflow
    #[allow(clippy::cast_possible_truncation)]
    let length = total_length - prefix.len() as u8;

    let number_space = 10_u64.pow(u32::from(length));

    let threads = optimal_thread_count(shared.threads, number_space);

    let device = if let Some(device) = shared.device {
        device
    } else if number_space > u64::from(threads) * super::OPTIMAL_HASHES_PER_THREAD {
        Device::GPU
    } else {
        Device::CPU
    };

    Mode::Decrypt(Decrypt {
        shared: shared.shared.into(input, algorithm),
        files: shared.files.into_iter().collect(),
        length,
        threads,
        number_space,
        prefix,
        device,
    })
}

impl RawShared {
    fn into<T: std::fmt::Display>(self, input: Vec<T>, algorithm: Algorithm) -> Shared {
        Shared {
            input: input.into_iter().map(|i| i.to_string()).collect(),
            verboseness: match self.verbose {
                0 => Verboseness::None,
                1 => Verboseness::Low,
                _ => Verboseness::High,
            },
            algorithm,
            salt: if let Some(salt) = self.salt {
                salt.unwrap_or_default()
            } else {
                std::env::var(crate::SALT_ENV)
                    .unwrap_or_else(|_| String::from(crate::secrets::SALT))
            },
        }
    }
}

// Allowed because the count was checked for overflow
#[allow(clippy::cast_possible_truncation)]
fn optimal_thread_count(requested_count: u8, number_space: u64) -> u8 {
    let threads = std::cmp::min(
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
    threads as u8
}

fn to_path(value: &str) -> Result<std::path::PathBuf, Error> {
    let path = std::path::PathBuf::from(value);
    if path.exists() && path.is_file() {
        Ok(path)
    } else {
        Err(Error::Simple(format!("'{}' is not a file", value)))
    }
}

fn to_device(value: &str) -> Result<Device, Error> {
    match value.to_uppercase().as_str() {
        "CPU" => Ok(Device::CPU),
        "GPU" => Ok(Device::GPU),
        _ => Err(Error::Simple(String::from(
            "possible values are [CPU, GPU]",
        ))),
    }
}
