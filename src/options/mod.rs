use clap::Clap;

use crate::{error::Error, hash, print};

mod args;

pub static OPTIMAL_HASHES_PER_THREAD: u64 = 1024 * 16;

pub fn parse() -> Mode {
    let mut mode: Mode = args::RawMode::parse().into();

    if !atty::is(atty::Stream::Stdin) {
        print::loading_start(mode.verboseness(), "stdin");
        print::loading_done(
            mode.verboseness(),
            mode.insert_input_from_stream(std::io::stdin().lock()),
        );
    }

    if let Mode::Decrypt(ref mut decrypt) = mode {
        decrypt
            .files
            .iter()
            .inspect(|file| {
                print::loading_start(decrypt.shared.verboseness, &file.display().to_string());
            })
            .filter_map(|file| match std::fs::File::open(file) {
                Ok(f) => Some(f),
                Err(e) => {
                    print::loading_done(
                        decrypt.shared.verboseness,
                        error!(e; "Could not open '{}'", file.display()),
                    );
                    None
                }
            })
            .collect::<Vec<_>>()
            .iter()
            .for_each(|file| {
                let reader = std::io::BufReader::new(file);
                print::loading_done(mode.verboseness(), mode.insert_input_from_stream(reader));
            });
    }

    if mode.input().is_empty() {
        panic!("No valid input provided");
    }

    mode
}

#[derive(Copy, Clone)]
pub enum Verboseness {
    None,
    Low,
    High,
}

#[derive(Clap, PartialEq, Debug, Copy, Clone)]
pub enum Algorithm {
    MD5 = 32,
    SHA256 = 64,
}

impl Algorithm {
    pub fn regex(self) -> &'static regex::Regex {
        use lazy_static::lazy_static;

        match self {
            Algorithm::MD5 => {
                lazy_static! {
                    static ref RE: regex::Regex = regex::Regex::new("\\b[0-9a-fA-F]{32}\\b")
                        .expect("Could not build regex for MD5");
                }
                &RE
            }
            Algorithm::SHA256 => {
                lazy_static! {
                    static ref RE: regex::Regex = regex::Regex::new("\\b[0-9a-fA-F]{64}\\b")
                        .expect("Could not build regex for SHA256");
                }
                &RE
            }
        }
    }

    fn variants() -> &'static [&'static str] {
        &["MD5", "SHA256"]
    }
}

impl std::fmt::Display for Algorithm {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MD5 => write!(fmt, "MD5"),
            Self::SHA256 => write!(fmt, "SHA256"),
        }
    }
}

#[derive(Clap, PartialEq, Debug, Copy, Clone)]
pub enum Device {
    CPU,
    GPU,
}

impl Device {
    fn variants() -> &'static [&'static str] {
        &["CPU", "GPU"]
    }
}

impl std::fmt::Display for Device {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CPU => write!(fmt, "CPU"),
            Self::GPU => write!(fmt, "GPU"),
        }
    }
}

pub struct Shared {
    input: std::collections::HashSet<String>,
    algorithm: Algorithm,
    salt: String,
    verboseness: Verboseness,
}

pub trait SharedAccessor {
    fn shared(&self) -> &Shared;

    fn input(&self) -> &std::collections::HashSet<String> {
        &self.shared().input
    }

    fn algorithm(&self) -> Algorithm {
        self.shared().algorithm
    }

    fn salt(&self) -> &str {
        &self.shared().salt
    }

    fn verboseness(&self) -> Verboseness {
        self.shared().verboseness
    }
}

pub struct Encrypt {
    shared: Shared,
}

impl SharedAccessor for Encrypt {
    fn shared(&self) -> &Shared {
        &self.shared
    }
}

pub struct Decrypt {
    shared: Shared,
    files: std::collections::HashSet<std::path::PathBuf>,
    length: u8,
    threads: u8,
    number_space: u64,
    prefix: String,
    device: Device,
}

impl SharedAccessor for Decrypt {
    fn shared(&self) -> &Shared {
        &self.shared
    }
}

impl Decrypt {
    // Allowed because it is only for tests
    #[cfg(test)]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        input: std::collections::HashSet<String>,
        files: std::collections::HashSet<std::path::PathBuf>,
        algorithm: Algorithm,
        salt: String,
        verboseness: Verboseness,
        length: u8,
        threads: u8,
        number_space: u64,
        prefix: String,
        device: Device,
    ) -> Self {
        Self {
            shared: Shared {
                input,
                algorithm,
                salt,
                verboseness,
            },
            files,
            length,
            threads,
            number_space,
            prefix,
            device,
        }
    }

    pub fn files(&self) -> &std::collections::HashSet<std::path::PathBuf> {
        &self.files
    }

    pub fn length(&self) -> u8 {
        self.length
    }

    pub fn threads(&self) -> u8 {
        self.threads
    }

    pub fn number_space(&self) -> u64 {
        self.number_space
    }

    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    pub fn device(&self) -> Device {
        self.device
    }

    pub fn input_as_eytzinger<C: hash::Converter>(&self) -> Vec<C::Output> {
        use eytzinger::SliceExt;
        let mut data = self
            .shared
            .input
            .iter()
            .map(String::as_str)
            .map(C::from_str)
            .filter_map(Result::ok)
            .collect::<Vec<_>>();
        data.sort_unstable();
        data.as_mut_slice()
            .eytzingerize(&mut eytzinger::permutation::InplacePermutator);
        data
    }

    // Allowed because prefix is always less than total_size
    #[allow(clippy::cast_possible_truncation)]
    pub fn prefix_length(&self) -> u8 {
        self.prefix.len() as u8
    }
}

pub enum Mode {
    Encrypt(Encrypt),
    Decrypt(Decrypt),
}

impl Mode {
    fn insert_input_from_stream(&mut self, mut stream: impl std::io::BufRead) -> Result<(), Error> {
        let mut buffer = String::new();
        match self {
            Self::Encrypt(ref mut mode) => {
                if let Ok(bytes) = stream.read_to_string(&mut buffer) {
                    if bytes > 0 {
                        mode.shared.input.insert(buffer);
                    }
                }
                Ok(())
            }
            Mode::Decrypt(ref mut mode) => {
                let regex = mode.algorithm().regex();

                loop {
                    buffer.clear();
                    match stream.read_line(&mut buffer) {
                        Ok(bytes) => {
                            if bytes == 0 {
                                break;
                            }

                            mode.shared
                                .input
                                .extend(regex.find_iter(&buffer).map(|m| String::from(m.as_str())));
                        }
                        Err(e) => {
                            return error!(e; "Error reading");
                        }
                    }
                }
                Ok(())
            }
        }
    }
}

impl SharedAccessor for Mode {
    fn shared(&self) -> &Shared {
        match &self {
            Self::Encrypt(mode) => mode.shared(),
            Self::Decrypt(mode) => mode.shared(),
        }
    }
}
