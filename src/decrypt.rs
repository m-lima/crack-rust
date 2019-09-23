extern crate md5;
extern crate num_cpus;
extern crate sha2;

use super::hash;
use super::options;
use super::summary;
use eytzinger::SliceExt;

static OPTIMAL_HASHES_PER_THREAD: u64 = 1024;

#[derive(Clone)]
pub struct Input {
    data: std::rc::Rc<Vec<hash::Hash>>,
}

unsafe impl Send for Input {}

fn get_optimal_thread_count(requested_count: u8, number_space: u64) -> u8 {
    std::cmp::min(
        number_space / OPTIMAL_HASHES_PER_THREAD + 1,
        if requested_count == 0 {
            num_cpus::get() as u8
        } else {
            requested_count
        } as u64,
    ) as u8
}

pub fn execute(options: options::Decrypt) -> summary::Variant {
    let time = std::time::Instant::now();

    let count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(
        options.shared.input.len(),
    ));
    let input = {
        use hash::IntoHash;
        let mut data = options
            .shared
            .input
            .into_iter()
            .map(|v| v.into_hash().unwrap())
            .collect::<Vec<hash::Hash>>();
        data.as_mut_slice()
            .eytzingerize(&mut eytzinger::permutation::InplacePermutator);
        Input {
            data: std::rc::Rc::new(data),
        }
    };

    let thread_count = get_optimal_thread_count(options.thread_count, options.number_space);
    let thread_space = options.number_space / thread_count as u64;
    let mut threads = Vec::<std::thread::JoinHandle<(u64)>>::with_capacity(thread_count as usize);

    for t in 0..thread_count {
        let count = count.clone();
        let input = input.clone();

        let algorithm = options.shared.algorithm.clone();
        let prefix = options.prefix.clone();
        let salted_prefix = format!("{}{}", options.shared.salt, options.prefix);
        let length = options.length as usize;
        let this_thread_space = if t < thread_count - 1 {
            thread_space
        } else {
            options.number_space - t as u64 * thread_space
        };

        threads.push(std::thread::spawn(move || {
            for n in (t as u64 * this_thread_space)..((t + 1) as u64 * this_thread_space) {
                if (n & OPTIMAL_HASHES_PER_THREAD - 1) == OPTIMAL_HASHES_PER_THREAD - 1 {
                    if count.load(std::sync::atomic::Ordering::Acquire) == 0 {
                        return n - (t as u64 * this_thread_space);
                    }
                }

                let number = format!("{:01$}", n, length);
                let hash = match &algorithm {
                    options::Algorithm::MD5 => hash::compute::<md5::Md5>(&salted_prefix, &number),
                    options::Algorithm::SHA256 => {
                        hash::compute::<sha2::Sha256>(&salted_prefix, &number)
                    }
                };
                if input.data.eytzinger_search(&hash).is_some() {
                    count.fetch_sub(1, std::sync::atomic::Ordering::Release);
                    if input.data.len() == 1 {
                        println!("{}{:02$}", &prefix, n, length);
                        return n - (t as u64 * this_thread_space);
                    }
                    println!("{:x} :: {}{:03$}", &hash, &prefix, n, length);
                }
            }
            this_thread_space
        }));
    }

    let hash_count = threads
        .into_iter()
        .map(|t| t.join().unwrap())
        .fold(0u64, |p, c| p + c);

    summary::Variant::Decrypt(summary::Decrypt {
        total_count: input.data.len(),
        cracked_count: input.data.len() - count.load(std::sync::atomic::Ordering::Relaxed),
        duration: time.elapsed(),
        hash_count,
        thread_count,
    })
}
