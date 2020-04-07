use clap::arg_enum;

mod args;

use crate::hash;

pub static OPTIMAL_HASHES_PER_THREAD: u64 = 1024 * 16;

pub fn parse() -> Mode {
    args::parse()
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

impl SharedAccessor for Mode {
    fn shared(&self) -> &Shared {
        match &self {
            Self::Encrypt(mode) => mode.shared(),
            Self::Decrypt(mode) => mode.shared(),
        }
    }
}
