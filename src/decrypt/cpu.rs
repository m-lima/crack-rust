use crate::hash;
use crate::options;
use crate::summary;

static OPTIMAL_HASHES_PER_THREAD: u64 = 1024;

#[derive(Clone)]
pub struct Sender<T> {
    data: *const T,
}

impl<T> std::ops::Deref for Sender<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.data }
    }
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
                eprintln!("Too many cores.. You have one powerful computer!");
                std::process::exit(-1);
            }
            cores as u64
        } else {
            u64::from(requested_count)
        },
    );

    // Due to `min`, it will always be less than u8::MAX (255)
    thread_count as u8
}

split_by_algorithm!(execute_typed);

fn execute_typed<D: digest::Digest, C: hash::Converter<D>>(
    options: &options::Decrypt,
) -> summary::Mode {
    let time = std::time::Instant::now();

    let count = std::sync::atomic::AtomicUsize::new(options.shared.input.len());
    let input = options.input_as_eytzinger::<_, C>();

    let thread_count = get_optimal_thread_count(options.thread_count, options.number_space);
    let thread_space = options.number_space / u64::from(thread_count);
    let mut threads = Vec::<_>::with_capacity(thread_count as usize);

    for t in 0..u64::from(thread_count) {
        let count_sender = Sender { data: &count };
        let input_sender = Sender { data: &input };

        let prefix = options.prefix.clone();
        let salted_prefix = format!("{}{}", options.shared.salt, options.prefix);
        let length = options.length as usize;
        let first = t * thread_space;
        let last = std::cmp::min(first + thread_space, options.number_space);

        threads.push(std::thread::spawn(move || {
            let count = &count_sender;
            let input = &input_sender;
            let mut decrypted = Vec::new();

            for n in first..last {
                use eytzinger::SliceExt;

                if n & (OPTIMAL_HASHES_PER_THREAD - 1) == OPTIMAL_HASHES_PER_THREAD - 1
                    && count.load(std::sync::atomic::Ordering::Acquire) == 0
                {
                    return (n - first, decrypted);
                }

                let number = format!("{:01$}", n, length);
                let hash = C::digest(&salted_prefix, &number);
                if input.eytzinger_search(&hash).is_some() {
                    count.fetch_sub(1, std::sync::atomic::Ordering::Release);
                    let result = format!("{}{:02$}", &prefix, n, length);
                    decrypted.push(summary::Decrypted::new(hash.to_string(), result.clone()));

                    if input.len() == 1 {
                        #[cfg(not(test))]
                        println!("{}{:02$}", &prefix, n, length);
                        return (n - first, decrypted);
                    }
                    #[cfg(not(test))]
                    println!("{:x} :: {}", &hash, &result);
                }
            }
            (last - first, decrypted)
        }));
    }

    let count_sender = Sender { data: &count };
    if let Err(err) = ctrlc::set_handler(move || {
        let count = &count_sender;
        count.store(0, std::sync::atomic::Ordering::Relaxed);
    }) {
        eprintln!("Failed to capture SIGINT: {}", err);
        eprintln!("CTRL + C will not interrupt the threads");
    }

    let (hash_count, results) = threads
        .into_iter()
        .map(|t| t.join().expect("Failed to join threads"))
        .fold((0, Vec::new()), |acc, curr| {
            (acc.0 + curr.0, {
                let mut v = acc.1;
                v.extend(curr.1);
                v
            })
        });

    summary::Mode::Decrypt(summary::Decrypt {
        total_count: input.len(),
        duration: time.elapsed(),
        hash_count,
        thread_count: u32::from(thread_count),
        results,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decryption() {
        let salt = String::from("abc");
        let prefix = String::from("1");

        let expected = vec![
            summary::Decrypted {
                hash: String::from(
                    "6ca13d52ca70c883e0f0bb101e425a89e8624de51db2d2392593af6a84118090",
                ),
                plain: prefix.clone() + "23",
            },
            summary::Decrypted {
                hash: String::from(
                    "97193f3095a7fc166ae10276c083735b41a36abdaac6a33e62d15b7eafa22a67",
                ),
                plain: prefix.clone() + "55",
            },
            summary::Decrypted {
                hash: String::from(
                    "237dd1639d476eda038aff4b83283e3c657a9f38b50c2d7177336d344fe8992e",
                ),
                plain: prefix.clone() + "99",
            },
        ];

        let options = options::Decrypt {
            shared: options::Shared {
                input: expected.iter().map(|v| v.hash.to_string()).collect(),
                algorithm: options::Algorithm::SHA256,
                salt,
            },
            length: 2u8,
            thread_count: 4,
            number_space: 100,
            prefix,
            device: options::Device::CPU,
        };

        if let summary::Mode::Decrypt(decrypt) = execute(&options) {
            assert_eq!(decrypt.results, expected);
        }
    }
}
