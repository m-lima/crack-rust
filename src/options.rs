extern crate clap;

use clap::arg_enum;

clap::arg_enum! {
    #[derive(PartialEq, Debug, Clone)]
    pub enum Algorithm {
        MD5 = 32,
        SHA256 = 64,
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
}

pub enum Variant {
    Encrypt(Encrypt),
    Decrypt(Decrypt),
}
