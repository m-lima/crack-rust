use crate::error;
use crate::files;
use crate::hash;
use crate::options;
use crate::secrets;

use super::print;

const SALT_ENV: &str = "HASHER_SALT";
const XOR_ENV: &str = "HASHER_XOR";

type Result<T> = std::result::Result<T, error::Error>;

/// SHA256 hasher/cracker
#[derive(clap::Parser, Debug)]
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
#[derive(clap::Parser, Debug)]
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

#[derive(clap::Parser, Debug)]
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

    // Allowed because this is done for the help rendering, but fetched manually from the params
    #[allow(dead_code)]
    /// Algorithm to use
    #[clap(short, long, default_value = "sha256", possible_values = hash::Algorithm::variants(), parse(try_from_str = to_algorithm))]
    algorithm: hash::Algorithm,
}

#[derive(clap::Parser, Debug)]
pub struct RawHash {
    #[clap(flatten)]
    shared: RawShared,

    /// Values to hash
    ///
    /// If a single input is given, only the hash will be printed to stdout. If more than one input
    /// is given, the pairs <input>:<hash> will be printed to stdout, one per line
    input: Vec<String>,
}

#[derive(clap::Parser, Debug)]
pub struct RawCrackShared {
    #[clap(flatten)]
    shared: RawShared,

    /// Input files. Will be scanned for hashes to crack
    ///
    /// If any hash from a given file is cracked, a copy of the file will be created in the same
    /// directory with the ".cracked" extension containing all cracked hashes substituted in place
    #[clap(short, long, parse(try_from_str = to_path))]
    files: Vec<std::path::PathBuf>,

    /// XOR mask to apply to plain values prior to hashing [env: HASHER_XOR]
    ///
    /// The mask is expected to be given as a base64 encoded representation
    #[clap(short, long)]
    #[allow(clippy::option_option)]
    xor: Option<Option<String>>,

    /// Known prefix of original values
    #[clap(short, long)]
    prefix: Option<String>,

    /// Number of threads to spawn, automatic deduction if omitted
    #[clap(short, long)]
    threads: Option<u8>,

    /// Device to run in (auto-detection if omitted)
    #[clap(short, long, possible_values = options::Device::variants(), parse(try_from_str = to_device))]
    device: Option<options::Device>,

    /// Length of original values
    #[clap(short, long, default_value = "12")]
    length: u8,
}

#[derive(clap::Parser, Debug)]
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

#[derive(clap::Parser, Debug)]
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

fn to_algorithm(value: &str) -> Result<hash::Algorithm> {
    match value.to_uppercase().as_str() {
        "SHA256" => Ok(hash::Algorithm::sha256),
        "MD5" => Ok(hash::Algorithm::md5),
        _ => bail!("possible values are [sha256, md5]",),
    }
}

fn to_path(value: &str) -> Result<std::path::PathBuf> {
    let path = std::path::PathBuf::from(value);
    if !path.exists() {
        bail!("{} does not exist", value)
    } else if !path.is_file() {
        bail!("{} is not a file", value)
    } else if let Err(e) = std::fs::File::open(&path) {
        bail!(e; "could not open {}", value)
    }
    Ok(path)
}

fn to_device(value: &str) -> Result<options::Device> {
    match value.to_uppercase().as_str() {
        "CPU" => Ok(options::Device::Cpu),
        "GPU" => Ok(options::Device::Gpu),
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

pub fn parse_sha256() -> Result<(options::Mode<hash::sha256::Hash>, print::Printer)> {
    use clap::Parser;
    use hash::sha256::Hash as H;

    let (mode, mut printer) = match RawModeSha256::parse() {
        RawModeSha256::Hash(encrypt) => compose_hash::<H>(encrypt),
        RawModeSha256::Crack(decrypt) => compose_crack::<H>(decrypt.shared, decrypt.input),
    }?;

    if mode.input_len() == 1 {
        printer.set_single_input_mode();
    }

    Ok((mode, printer))
}

pub fn parse_md5() -> Result<(options::Mode<hash::md5::Hash>, print::Printer)> {
    use clap::Parser;
    use hash::md5::Hash as H;

    let (mode, mut printer) = match RawModeMd5::parse() {
        RawModeMd5::Hash(encrypt) => compose_hash::<H>(encrypt),
        RawModeMd5::Crack(decrypt) => compose_crack::<H>(decrypt.shared, decrypt.input),
    }?;

    if mode.input_len() == 1 {
        printer.set_single_input_mode();
    }

    Ok((mode, printer))
}

fn compose_hash<H: hash::Hash>(encrypt: RawHash) -> Result<(options::Mode<H>, print::Printer)> {
    let printer = print::new(encrypt.shared.verbose, encrypt.shared.colored);

    Ok((
        options::Mode::Encrypt(options::Encrypt::<H>::new(
            read_string_from_stdin(encrypt.input.into_iter().collect(), printer),
            salt(encrypt.shared.salt.map(Option::unwrap_or_default)),
        )?),
        printer,
    ))
}

fn compose_crack<H: hash::Hash>(
    shared: RawCrackShared,
    input: Vec<H>,
) -> Result<(options::Mode<H>, print::Printer)> {
    let printer = print::new(shared.shared.verbose, shared.shared.colored);

    let prefix = shared.prefix.unwrap_or_default();

    let files = shared
        .files
        .into_iter()
        .collect::<std::collections::HashSet<_>>();
    let mut input: std::collections::HashSet<H> = input.into_iter().collect();

    for file in &files {
        printer.read_start(file.display().to_string());
        printer.read_done(files::read(&mut input, file));
    }

    if !atty::is(atty::Stream::Stdin) {
        printer.read_start("stdin");
        printer.read_done(files::read_from_stream(&mut input, std::io::stdin().lock()));
    }

    Ok((
        options::Mode::Decrypt(
            options::DecryptBuilder::new(input, shared.length)
                .device(shared.device)
                .files(files)
                .prefix(prefix)
                .salt(salt(shared.shared.salt.map(Option::unwrap_or_default)))
                .threads(shared.threads)
                .xor(xor(shared.xor)?)
                .build()?,
        ),
        printer,
    ))
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

fn salt(maybe_salt: Option<String>) -> String {
    maybe_salt
        .unwrap_or_else(|| std::env::var(SALT_ENV).unwrap_or_else(|_| String::from(secrets::SALT)))
}

#[allow(clippy::option_option)]
fn xor(maybe_xor: Option<Option<String>>) -> Result<Option<Vec<u8>>> {
    maybe_xor.map_or(Ok(None), |maybe_xor| {
        let xor = maybe_xor.unwrap_or_else(|| {
            std::env::var(XOR_ENV).unwrap_or_else(|_| String::from(secrets::XOR))
        });
        base64::decode(xor)
            .map(Option::Some)
            .map_err(|err| error!(err; "Failed to decode XOR mask"))
    })
}
