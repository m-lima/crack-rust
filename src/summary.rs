use super::hash;

pub struct Decrypt {
    pub total_count: usize,
    pub cracked_count: usize,
    pub duration: std::time::Duration,
    pub hash_count: u64,
    pub thread_count: u8,
    pub results: Vec<Decrypted>,
}

pub enum Mode {
    Encrypt(),
    Decrypt(Decrypt),
}

#[derive(Debug, PartialEq)]
pub struct Decrypted {
    pub hash: String,
    pub plain: String,
}

impl Decrypted {
    pub fn new<H: hash::Hash>(hash: H, plain: String) -> Self {
        Self {
            hash: hash.to_string(),
            plain,
        }
    }
}
