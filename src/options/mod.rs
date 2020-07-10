mod args;

use crate::hash;
use crate::print;

use clap::Clap;

pub static OPTIMAL_HASHES_PER_THREAD: u64 = 1024 * 16;

pub fn parse() -> Mode {
    let mut mode: Mode = args::RawMode::parse().into();

    if !atty::is(atty::Stream::Stdin) {
        print::loading(mode.verboseness(), "stdin");
        mode.insert_input_from_stream(std::io::stdin().lock(), None);
    }

    if let Mode::Decrypt(ref mut decrypt) = mode {
        decrypt
            .files
            .iter()
            .inspect(|file| {
                print::loading(decrypt.shared.verboseness, &file.display().to_string());
            })
            .filter_map(|file| match std::fs::File::open(file) {
                Ok(f) => Some(f),
                Err(e) => {
                    eprintln!("Could not open '{}': {}", file.display(), e);
                    None
                }
            })
            .collect::<Vec<_>>()
            .iter()
            .for_each(|file| {
                let total_bytes = file.metadata().map(|f| f.len()).ok();
                let reader = std::io::BufReader::new(file);
                mode.insert_input_from_stream(reader, total_bytes);
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
    thread_count: u8,
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
        thread_count: u8,
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
            thread_count,
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

    pub fn thread_count(&self) -> u8 {
        self.thread_count
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

    pub fn input_as_eytzinger<D: digest::Digest, C: hash::Converter<D>>(&self) -> Vec<C::Output> {
        use eytzinger::SliceExt;
        let mut data = self
            .shared
            .input
            .iter()
            .map(String::as_str)
            .map(C::from_str)
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
    fn insert_input_from_stream(
        &mut self,
        mut stream: impl std::io::BufRead,
        total_bytes: Option<u64>,
    ) {
        let mut buffer = String::new();
        match self {
            Self::Encrypt(ref mut mode) => {
                if let Ok(bytes) = stream.read_to_string(&mut buffer) {
                    if bytes > 0 {
                        mode.shared.input.insert(buffer);
                    } else {
                        return;
                    }
                }
            }
            Mode::Decrypt(ref mut mode) => {
                let regex = mode.algorithm().regex();

                if total_bytes.is_some() {
                    print::progress(0);
                }

                let mut bytes_read = 0;
                while let Ok(bytes) = stream.read_line(&mut buffer) {
                    if bytes == 0 {
                        print::clear_progress();
                        return;
                    }

                    if let Some(total) = total_bytes {
                        bytes_read += bytes;

                        // Allowed because this will be a percentage (less than 100)
                        #[allow(clippy::cast_possible_truncation)]
                        print::progress(((bytes_read * 100) as u64 / total) as u32);
                    }

                    mode.shared
                        .input
                        .extend(regex.find_iter(&buffer).map(|m| String::from(m.as_str())));

                    buffer.clear();
                }
            }
        }
        eprintln!("There was a problem reading the file");
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
