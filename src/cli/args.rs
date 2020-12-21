use clap::Clap;

use crate::cli::print;
use crate::decrypt;
use crate::error;
use crate::files;
use crate::hash;

use crate::options;

static SALT_ENV: &str = "HASHER_SALT";

pub fn algorithm() -> hash::Algorithm {
    std::env::args()
        .position(|arg| arg == "-a" || arg == "--algorithm")
        .and_then(|index| {
            std::env::args()
                .nth(index + 1)
                .and_then(|string| to_algorithm(&string).ok())
        })
        .unwrap_or(hash::Algorithm::sha256)
}

pub fn parse_sha256() -> options::Mode<hash::sha256::Hash> {
    use hash::sha256::Hash as H;

    let mode = match RawModeSha256::parse() {
        RawModeSha256::Hash(encrypt) => options::Mode::Encrypt(compose_hash::<H>(encrypt)),
        RawModeSha256::Crack(decrypt) => {
            options::Mode::Decrypt(compose_crack::<H>(decrypt.shared, decrypt.input))
        }
    };

    if mode.input_len() == 0 {
        panic!("No valid input provided");
    }

    mode
}

pub fn parse_md5() -> options::Mode<hash::md5::Hash> {
    use hash::md5::Hash as H;

    let mode = match RawModeMd5::parse() {
        RawModeMd5::Hash(encrypt) => options::Mode::Encrypt(compose_hash::<H>(encrypt)),
        RawModeMd5::Crack(decrypt) => {
            options::Mode::Decrypt(compose_crack::<H>(decrypt.shared, decrypt.input))
        }
    };

    if mode.input_len() == 0 {
        panic!("No valid input provided");
    }

    mode
}

/// SHA256 hasher/cracker
#[derive(Clap, Debug)]
#[clap(
    name = "Hasher",
    version,
    after_help = "Input can be provided through stdin or as parameters"
)]
pub enum RawModeSha256 {
    /// Generate hashes
    Hash(RawHash),
    /// Crack hashes
    #[clap(
        after_help = "The cracker will exit with an error if any of the input hashes could not be cracked"
    )]
    Crack(RawCrackSha256),
}

/// Md5 hasher/cracker
#[derive(Clap, Debug)]
#[clap(
    name = "Hasher",
    version,
    after_help = "Input can be provided through stdin or as parameters"
)]
pub enum RawModeMd5 {
    /// Generate hashes
    Hash(RawHash),
    /// Crack hashes
    #[clap(
        after_help = "The cracker will exit with an error if any of the input hashes could not be cracked"
    )]
    Crack(RawCrackMd5),
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
    verbose: print::Verboseness,

    /// Disable colors
    #[clap(short('n'), long("no-colors"), parse(from_flag = std::ops::Not::not))]
    colored: bool,

    /// Algorithm to use
    #[clap(short, long, default_value = "sha256", possible_values = hash::Algorithm::variants(), parse(try_from_str = to_algorithm))]
    algorithm: hash::Algorithm,
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
    #[clap(short, long, possible_values = options::Device::variants(), parse(try_from_str = to_device))]
    device: Option<options::Device>,

    /// Length of hashed values
    #[clap(short, long, default_value = "12")]
    length: u8,
}

#[derive(Clap, Debug)]
pub struct RawCrackSha256 {
    #[clap(flatten)]
    shared: RawCrackShared,

    /// Hashed values to crack. Expected to be the hash of a numeric value
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

    /// Hashed values to crack. Expected to be the hash of a numeric value
    ///
    /// If a single hash is given, only the cracked value will be printed to stdout.
    /// If more than one hash is given, the pairs <hash>:<cracked value> will be printed to stdout,
    /// one per line
    #[clap(parse(try_from_str = <hash::md5::Hash as hash::Hash>::from_str))]
    input: Vec<hash::md5::Hash>,
}

fn to_algorithm(value: &str) -> Result<hash::Algorithm, error::Error> {
    match value.to_uppercase().as_str() {
        "SHA256" => Ok(hash::Algorithm::sha256),
        "MD5" => Ok(hash::Algorithm::md5),
        _ => bail!("possible values are [sha256, md5]",),
    }
}

fn to_path(value: &str) -> Result<std::path::PathBuf, error::Error> {
    let path = std::path::PathBuf::from(value);
    if !path.exists() {
        bail!("{} does not exist", value)
    } else if !path.is_file() {
        bail!("{} is not a file", value)
    } else if let Err(e) = std::fs::File::open(&path) {
        bail!(e; "could not open {}", value)
    } else {
        Ok(path)
    }
}

fn to_device(value: &str) -> Result<options::Device, error::Error> {
    match value.to_uppercase().as_str() {
        "CPU" => Ok(options::Device::CPU),
        "GPU" => Ok(options::Device::GPU),
        _ => bail!("possible values are [CPU, GPU]",),
    }
}

fn to_verboseness(value: u64) -> print::Verboseness {
    match value {
        0 => print::Verboseness::None,
        1 => print::Verboseness::Low,
        _ => print::Verboseness::High,
    }
}

fn compose_hash<H: hash::Hash>(encrypt: RawHash) -> options::Encrypt<H> {
    let printer = print::new(encrypt.shared.verbose, encrypt.shared.colored);
    options::Encrypt::<H>::new(
        read_string_from_stdin(encrypt.input.into_iter().collect(), printer),
        salt(encrypt.shared),
        printer,
    )
}

fn compose_crack<H: hash::Hash>(shared: RawCrackShared, input: Vec<H>) -> options::Decrypt<H> {
    let printer = print::new(shared.shared.verbose, shared.shared.colored);

    let prefix = shared.prefix.unwrap_or_default();

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
    } else if number_space > u64::from(threads) * decrypt::OPTIMAL_HASHES_PER_THREAD {
        options::Device::GPU
    } else {
        options::Device::CPU
    };

    let files = shared.files.into_iter().collect();
    let input = files::read(input.into_iter().collect(), &files, printer);

    options::Decrypt::new(
        read_hash_from_stdin(input, printer),
        salt(shared.shared),
        printer,
        files,
        length,
        threads,
        number_space,
        prefix,
        device,
    )
}

// Allowed because the count was checked for overflow
#[allow(clippy::cast_possible_truncation)]
fn optimal_thread_count(requested_count: u8, number_space: u64) -> u8 {
    let threads = std::cmp::min(
        number_space / decrypt::OPTIMAL_HASHES_PER_THREAD + 1,
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

fn salt(shared: RawShared) -> String {
    match shared.salt {
        Some(salt) => salt.unwrap_or_default(),
        None => std::env::var(SALT_ENV).unwrap_or_else(|_| String::from(crate::secrets::SALT)),
    }
}

fn read_string_from_stdin(
    mut input: std::collections::HashSet<String>,
    printer: print::Printer,
) -> std::collections::HashSet<String> {
    if !atty::is(atty::Stream::Stdin) {
        use std::io::Read;

        printer.read_start("stdin");
        let mut buffer = String::new();
        if let Ok(bytes) = std::io::stdin().read_to_string(&mut buffer) {
            if bytes > 0 {
                input.insert(buffer);
            }
        }
    }
    printer.read_done(Ok(()));
    input
}

fn read_hash_from_stdin<H: hash::Hash>(
    input: std::collections::HashSet<H>,
    printer: print::Printer,
) -> std::collections::HashSet<H> {
    if atty::is(atty::Stream::Stdin) {
        input
    } else {
        printer.read_start("stdin");
        files::insert_from_stream(input, std::io::stdin().lock(), printer)
    }
}
