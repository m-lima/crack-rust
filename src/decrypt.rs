use eytzinger::SliceExt;

use super::hash;
use super::options;
use super::summary;

static OPTIMAL_HASHES_PER_THREAD: u64 = 1024;

#[derive(Clone)]
pub struct Sender<T> {
    data: *const T,
}

unsafe impl<T> Send for Sender<T> {}

// Allowed because the count was checked for overflow
#[allow(clippy::cast_possible_truncation)]
fn get_optimal_thread_count(requested_count: u8, number_space: u64) -> u8 {
    let thread_count = std::cmp::min(
        number_space / OPTIMAL_HASHES_PER_THREAD + 1,
        if requested_count == 0 {
            let cores = num_cpus::get();
            if cores > usize::from(u8::max_value()) {
                panic!("Too many cores.. You have one powerful computer!");
            }
            cores as u64
        } else {
            u64::from(requested_count)
        },
    );

    // Due to `min`, it will always be less than u8::MAX (255)
    thread_count as u8
}

pub fn execute(options: &options::Decrypt) -> summary::Mode {
    let time = std::time::Instant::now();

    let count = std::sync::atomic::AtomicUsize::new(options.shared.input.len());
    let input = {
        use hash::Into;
        let mut data = options
            .shared
            .input
            .iter()
            .map(|v| v.into_hash().unwrap())
            .collect::<Vec<hash::Hash>>();
        data.sort_unstable();
        data.as_mut_slice()
            .eytzingerize(&mut eytzinger::permutation::InplacePermutator);
        data
    };

    let thread_count = get_optimal_thread_count(options.thread_count, options.number_space);
    let thread_space = options.number_space / u64::from(thread_count);
    let mut threads = Vec::<std::thread::JoinHandle<u64>>::with_capacity(thread_count as usize);

    for t in 0..u64::from(thread_count) {
        let count_sender = Sender { data: &count };
        let input_sender = Sender { data: &input };

        let algorithm = options.shared.algorithm.clone();
        let prefix = options.prefix.clone();
        let salted_prefix = format!("{}{}", options.shared.salt, options.prefix);
        let length = options.length as usize;
        let first = t * thread_space;
        let last = std::cmp::min(first + thread_space, options.number_space);

        threads.push(std::thread::spawn(move || unsafe {
            let count = &*count_sender.data;
            let input = &*input_sender.data;

            for n in first..last {
                if n & (OPTIMAL_HASHES_PER_THREAD - 1) == OPTIMAL_HASHES_PER_THREAD - 1
                    && count.load(std::sync::atomic::Ordering::Acquire) == 0
                {
                    return n - first;
                }

                let number = format!("{:01$}", n, length);
                let hash = match &algorithm {
                    options::Algorithm::MD5 => hash::compute::<md5::Md5>(&salted_prefix, &number),
                    options::Algorithm::SHA256 => {
                        hash::compute::<sha2::Sha256>(&salted_prefix, &number)
                    }
                };
                if input.eytzinger_search(&hash).is_some() {
                    count.fetch_sub(1, std::sync::atomic::Ordering::Release);
                    if input.len() == 1 {
                        println!("{}{:02$}", &prefix, n, length);
                        return n - first;
                    }
                    println!("{:x} :: {}{:03$}", &hash, &prefix, n, length);
                }
            }
            last - first
        }));
    }

    let hash_count = threads.into_iter().map(|t| t.join().unwrap()).sum();

    summary::Mode::Decrypt(summary::Decrypt {
        total_count: input.len(),
        cracked_count: input.len() - count.load(std::sync::atomic::Ordering::Relaxed),
        duration: time.elapsed(),
        hash_count,
        thread_count,
    })
}
