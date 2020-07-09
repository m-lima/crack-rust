mod args;

use crate::hash;
use clap::arg_enum;

pub static OPTIMAL_HASHES_PER_THREAD: u64 = 1024 * 16;

pub fn parse() -> Mode {
    let mut args = args::parse();

    if !atty::is(atty::Stream::Stdin) {
        args.insert_input_from_stream(std::io::stdin().lock());
    }

    if let Mode::Decrypt(ref mut mode) = args {
        mode.files
            .iter()
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
                let reader = std::io::BufReader::new(file);
                args.insert_input_from_stream(reader);
            });
    }

    if args.input().is_empty() {
        panic!("No valid input provided");
    }
    args
}

#[derive(Copy, Clone)]
pub enum Verboseness {
    None,
    Low,
    High,
}

clap::arg_enum! {
    #[derive(PartialEq, Debug, Copy, Clone)]
    pub enum Algorithm {
        MD5 = 32,
        SHA256 = 64,
    }
}

clap::arg_enum! {
    #[derive(PartialEq, Debug, Copy, Clone)]
    pub enum Device {
        CPU,
        GPU,
    }
}

fn regex_for(algorithm: Algorithm) -> &'static regex::Regex {
    use lazy_static::lazy_static;

    match algorithm {
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

pub struct Shared {
    input: Vec<String>,
    algorithm: Algorithm,
    salt: String,
    verboseness: Verboseness,
}

pub trait SharedAccessor {
    fn shared(&self) -> &Shared;

    fn input(&self) -> &[String] {
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
    files: Vec<std::path::PathBuf>,
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
        input: Vec<String>,
        files: Vec<std::path::PathBuf>,
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
    fn insert_input_from_stream(&mut self, mut stream: impl std::io::BufRead) {
        let mut buffer = String::new();
        match self {
            Self::Encrypt(ref mut mode) => {
                if let Ok(bytes) = stream.read_to_string(&mut buffer) {
                    if bytes > 0 {
                        mode.shared.input.push(buffer);
                    }
                }
            }
            Self::Decrypt(ref mut mode) => {
                let regex = regex_for(mode.shared.algorithm);
                while let Ok(bytes) = stream.read_line(&mut buffer) {
                    if bytes == 0 {
                        return;
                    }

                    mode.shared
                        .input
                        .extend(regex.find_iter(&buffer).map(|m| String::from(m.as_str())));
                }
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
