pub struct Summary {
    pub total_count: usize,
    pub duration: std::time::Duration,
    pub hash_count: u64,
    pub threads: u32,
    pub results: Vec<Pair>,
}

#[derive(Debug, PartialEq)]
pub struct Pair {
    pub hash: String,
    pub plain: String,
}

impl Pair {
    pub fn new(hash: String, plain: String) -> Self {
        Self { hash, plain }
    }
}
