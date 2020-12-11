use crate::hash;

pub struct Summary {
    pub total_count: usize,
    pub duration: std::time::Duration,
    pub hash_count: u64,
    pub threads: u32,
    pub results: Vec<hash::Pair>,
}
