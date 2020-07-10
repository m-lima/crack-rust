pub struct Decrypt {
    pub total_count: usize,
    pub duration: std::time::Duration,
    pub hash_count: u64,
    pub threads: u32,
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
    pub fn new(hash: String, plain: String) -> Self {
        Self { hash, plain }
    }
}
