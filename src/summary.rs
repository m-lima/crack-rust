pub struct Decrypt {
    pub total_count: usize,
    pub cracked_count: usize,
    pub duration: std::time::Duration,
    pub hash_count: u64,
    pub thread_count: u8,
}

pub enum Mode {
    Encrypt(),
    Decrypt(Decrypt),
}
