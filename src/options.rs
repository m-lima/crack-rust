use clap::arg_enum;

use crate::hash;

clap::arg_enum! {
    #[derive(PartialEq, Debug, Clone)]
    pub enum Algorithm {
        MD5 = 32,
        SHA256 = 64,
    }
}

clap::arg_enum! {
    #[derive(PartialEq, Debug, Clone)]
    pub enum Device {
        CPU,
        GPU,
    }
}

pub struct Shared {
    pub input: Vec<String>,
    pub algorithm: Algorithm,
    pub salt: String,
}

pub struct Encrypt {
    pub shared: Shared,
}

pub struct Decrypt {
    pub shared: Shared,
    pub length: u8,
    pub thread_count: u8,
    pub number_space: u64,
    pub prefix: String,
    pub device: Device,
}

pub enum Mode {
    Encrypt(Encrypt),
    Decrypt(Decrypt),
}

impl Decrypt {
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
