use clap::Clap;

use crate::error;
use crate::files;
use crate::hash;
use crate::Input;

use super::{Decrypt, Device, Encrypt, Mode, Shared, Verboseness};

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
    #[clap(short, parse(from_occurrences = to_verboseness))]
    verbose: Verboseness,
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
    #[clap(parse(try_from_str = <hash::sha256::Hash as hash::Hash>::from_str))]
    input: Vec<hash::sha256::Hash>,
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
    #[clap(parse(try_from_str = <hash::md5::Hash as hash::Hash>::from_str))]
    input: Vec<hash::md5::Hash>,
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
            Self::Hash(encrypt) => Mode::Encrypt(Encrypt::<hash::sha256::Hash>::new(
                encrypt.shared.into(encrypt.input.into_iter().collect()),
            )),
            Self::HashMd5(encrypt) => Mode::EncryptMd5(Encrypt::<hash::md5::Hash>::new(
                encrypt.shared.into(encrypt.input.into_iter().collect()),
            )),
            Self::Crack(decrypt) => Mode::Decrypt(compose_crack(decrypt.shared, decrypt.input)),
            Self::CrackMd5(decrypt) => {
                Mode::DecryptMd5(compose_crack(decrypt.shared, decrypt.input))
            }
        }
    }
}

fn compose_crack<H: hash::Hash>(shared: RawCrackShared, input: Vec<H>) -> Decrypt<H> {
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

    let files = shared.files.into_iter().collect();
    let input = files::read(input.into_iter().collect(), &files);

    Decrypt {
        shared: shared.shared.into(files::read_hash_from_stdin(input)),
        files,
        length,
        threads,
        number_space,
        prefix,
        device,
    }
}

impl RawShared {
    fn into<T: Input>(self, input: std::collections::HashSet<T>) -> Shared<T> {
        Shared {
            input,
            verboseness: self.verbose,
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

fn to_path(value: &str) -> Result<std::path::PathBuf, error::Error> {
    let path = std::path::PathBuf::from(value);
    if path.exists() && path.is_file() {
        Ok(path)
    } else {
        error!("'{}' is not a file", value)
    }
}

fn to_device(value: &str) -> Result<Device, error::Error> {
    match value.to_uppercase().as_str() {
        "CPU" => Ok(Device::CPU),
        "GPU" => Ok(Device::GPU),
        _ => error!("possible values are [CPU, GPU]",),
    }
}

fn to_verboseness(value: u64) -> Verboseness {
    match value {
        0 => Verboseness::None,
        1 => Verboseness::Low,
        _ => Verboseness::High,
    }
}
