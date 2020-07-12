use clap::Clap;

use crate::hash;
// use crate::print;
use crate::Input;

mod args;

pub static OPTIMAL_HASHES_PER_THREAD: u64 = 1024 * 16;

pub fn parse() -> Mode {
    let mode: Mode = args::RawMode::parse().into();

    // if !atty::is(atty::Stream::Stdin) {
    //     print::loading_start(mode.verboseness(), "stdin");
    //     print::loading_done(
    //         mode.verboseness(),
    //         mode.insert_input_from_stream(std::io::stdin().lock()),
    //     );
    // }

    // if let Mode::Decrypt(ref mut decrypt) = mode {
    //     decrypt
    //         .files
    //         .iter()
    //         .inspect(|file| {
    //             print::loading_start(decrypt.shared.verboseness, &file.display().to_string());
    //         })
    //         .filter_map(|file| match std::fs::File::open(file) {
    //             Ok(f) => Some(f),
    //             Err(e) => {
    //                 print::loading_done(
    //                     decrypt.shared.verboseness,
    //                     error!(e; "Could not open '{}'", file.display()),
    //                 );
    //                 None
    //             }
    //         })
    //         .collect::<Vec<_>>()
    //         .iter()
    //         .for_each(|file| {
    //             let reader = std::io::BufReader::new(file);
    //             print::loading_done(mode.verboseness(), mode.insert_input_from_stream(reader));
    //         });
    // }

    // if mode.input().is_empty() {
    //     panic!("No valid input provided");
    // }

    mode
}

#[derive(Copy, Clone)]
pub enum Verboseness {
    None = 0,
    Low = 1,
    High = 2,
}

#[derive(Clap, PartialEq, Debug, Copy, Clone)]
pub enum Device {
    CPU,
    GPU,
}

impl Device {
    fn variants() -> &'static [&'static str] {
        &["cpu", "gpu"]
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

pub struct Shared<T: Input> {
    input: std::collections::HashSet<T>,
    salt: String,
    verboseness: Verboseness,
}

pub trait SharedAccessor<T: Input> {
    fn shared(&self) -> &Shared<T>;

    fn input(&self) -> &std::collections::HashSet<T> {
        &self.shared().input
    }

    fn salt(&self) -> &str {
        &self.shared().salt
    }

    fn verboseness(&self) -> Verboseness {
        self.shared().verboseness
    }
}

pub struct Encrypt<H: hash::Hash> {
    shared: Shared<String>,
    _phantom: std::marker::PhantomData<H>,
}

impl<H: hash::Hash> Encrypt<H> {
    fn new(shared: Shared<String>) -> Self {
        Self {
            shared,
            _phantom: std::marker::PhantomData::<H>::default(),
        }
    }
}

impl<H: hash::Hash> SharedAccessor<String> for Encrypt<H> {
    fn shared(&self) -> &Shared<String> {
        &self.shared
    }
}

pub struct Decrypt<H: hash::Hash> {
    shared: Shared<H>,
    files: std::collections::HashSet<std::path::PathBuf>,
    length: u8,
    threads: u8,
    number_space: u64,
    prefix: String,
    device: Device,
}

impl<H: hash::Hash> SharedAccessor<H> for Decrypt<H> {
    fn shared(&self) -> &Shared<H> {
        &self.shared
    }
}

impl<H: hash::Hash> Decrypt<H> {
    // Allowed because it is only for tests
    #[cfg(test)]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        input: std::collections::HashSet<H>,
        files: std::collections::HashSet<std::path::PathBuf>,
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

    pub fn input_as_eytzinger(&self) -> Vec<H> {
        use eytzinger::{permutation::InplacePermutator, SliceExt};
        let mut data = self
            .shared
            .input
            .iter()
            .map(Clone::clone)
            .collect::<Vec<_>>();
        data.sort_unstable();
        data.as_mut_slice().eytzingerize(&mut InplacePermutator);
        data
    }

    // Allowed because prefix is always less than total_size
    #[allow(clippy::cast_possible_truncation)]
    pub fn prefix_length(&self) -> u8 {
        self.prefix.len() as u8
    }
}

pub enum Mode {
    Encrypt(Encrypt<hash::sha256::Hash>),
    EncryptMd5(Encrypt<hash::md5::Hash>),
    Decrypt(Decrypt<hash::sha256::Hash>),
    DecryptMd5(Decrypt<hash::md5::Hash>),
}

// impl Mode {
//     fn insert_input_from_stream(&mut self, mut stream: impl std::io::BufRead) -> Result<(), Error> {
//         let mut buffer = String::new();
//         match self {
//             Self::Encrypt(ref mut mode) => {
//                 if let Ok(bytes) = stream.read_to_string(&mut buffer) {
//                     if bytes > 0 {
//                         mode.shared.input.insert(buffer);
//                     }
//                 }
//                 Ok(())
//             }
//             Self::EncryptMd5(ref mut mode) => {
//                 if let Ok(bytes) = stream.read_to_string(&mut buffer) {
//                     if bytes > 0 {
//                         mode.shared.input.insert(buffer);
//                     }
//                 }
//                 Ok(())
//             }
//             Self::Decrypt(ref mut mode) | Self::DecryptMd5(ref mut mode) => {
//                 let regex = mode.algorithm().regex();
//
//                 loop {
//                     buffer.clear();
//                     match stream.read_line(&mut buffer) {
//                         Ok(bytes) => {
//                             if bytes == 0 {
//                                 break;
//                             }
//
//                             mode.shared
//                                 .input
//                                 .extend(regex.find_iter(&buffer).map(|m| String::from(m.as_str())));
//                         }
//                         Err(e) => {
//                             return error!(e; "Error reading");
//                         }
//                     }
//                 }
//                 Ok(())
//             }
//         }
//     }
// }

impl Mode {
    pub fn verboseness(&self) -> Verboseness {
        match &self {
            Self::Encrypt(mode) => mode.shared.verboseness,
            Self::EncryptMd5(mode) => mode.shared.verboseness,
            Self::Decrypt(mode) => mode.shared.verboseness,
            Self::DecryptMd5(mode) => mode.shared.verboseness,
        }
    }

    pub fn input_len(&self) -> usize {
        match &self {
            Self::Encrypt(mode) => mode.shared.input.len(),
            Self::EncryptMd5(mode) => mode.shared.input.len(),
            Self::Decrypt(mode) => mode.shared.input.len(),
            Self::DecryptMd5(mode) => mode.shared.input.len(),
        }
    }
}

// impl<T> SharedAccessor<T> for Mode {
//     fn shared(&self) -> &Shared<T> {
//         match &self {
//             Self::Encrypt(mode) => mode.shared.verboseness,
//             Self::EncryptMd5(mode) => mode.shared.verboseness,
//             Self::Decrypt(mode) => mode.shared.verboseness,
//             Self::DecryptMd5(mode) => mode.shared.verboseness,
//         }
//     }
// }
