use crate::channel;
use crate::error;
use crate::hash;
use crate::options;
use crate::results;

use crate::options::SharedAccessor;

pub static OPTIMAL_HASHES_PER_THREAD: u64 = 1024 * 16;

#[derive(Clone, Copy)]
pub struct Sender<T>(*const T);

impl<T> std::ops::Deref for Sender<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

unsafe impl<T> Send for Sender<T> {}

pub fn execute<H: hash::Hash>(
    options: &options::Decrypt<H>,
    channel: impl channel::Channel,
) -> Result<results::Summary, error::Error> {
    let time = std::time::Instant::now();

    let count = std::sync::atomic::AtomicUsize::new(options.input().len());
    let input = options.input_as_eytzinger();

    let thread_count = options.threads();
    let thread_space = options.number_space() / u64::from(thread_count);
    let mut threads = Vec::<_>::with_capacity(thread_count as usize);

    channel.progress(0);
    for t in 0..u64::from(thread_count) {
        let count_sender = Sender(&count);
        let input_sender = Sender(&input);

        let prefix = String::from(options.prefix());
        let salted_prefix = format!("{}{}", options.salt(), options.prefix());
        let length = options.length() as usize;
        let first = t * thread_space;
        let last = std::cmp::min(first + thread_space, options.number_space());

        threads.push(std::thread::spawn(move || {
            let count = &count_sender;
            let input = &input_sender;
            let mut decrypted = Vec::new();

            for n in first..last {
                use eytzinger::SliceExt;

                if n & (OPTIMAL_HASHES_PER_THREAD - 1) == OPTIMAL_HASHES_PER_THREAD - 1 {
                    if t == 0 {
                        // Allowed because of division; value will stay in bound
                        // `n` is less than `last`
                        #[allow(clippy::cast_possible_truncation)]
                        channel.progress((n * 100 / last) as u8);
                    }
                    if channel.should_terminate() {
                        return (n - first, decrypted);
                    }
                }

                let number = format!("{:01$}", n, length);
                let hash = H::digest(&salted_prefix, &number);
                if input.eytzinger_search(&hash).is_some() {
                    count.fetch_sub(1, std::sync::atomic::Ordering::Release);
                    let result = format!("{}{:02$}", &prefix, n, length);
                    decrypted.push(results::Pair::new(hash.to_string(), result.clone()));

                    channel.result(&format!("{:x}", hash), &result);
                    if input.len() == 1 {
                        return (n - first, decrypted);
                    }
                }
            }
            (last - first, decrypted)
        }));
    }

    let (hash_count, results) = threads
        .into_iter()
        .map(|t| t.join().map_err(error::on_join))
        .fold(Ok((0, Vec::new())), |acc, curr| {
            if let Ok(mut acc) = acc {
                curr.map(|(count, results)| {
                    (acc.0 + count, {
                        acc.1.extend(results);
                        acc.1
                    })
                })
            } else {
                acc
            }
        })?;

    Ok(results::Summary {
        total_count: input.len(),
        duration: time.elapsed(),
        hash_count,
        threads: u32::from(thread_count),
        results,
    })
}

#[cfg(test)]
mod test {
    use super::{channel, execute, hash, options, results};

    #[derive(Copy, Clone)]
    struct Channel;

    impl channel::Channel for Channel {
        fn progress(&self, _: u8) {}
        fn result(&self, _: &str, _: &str) {}
        fn should_terminate(&self) -> bool {
            false
        }
    }

    #[test]
    fn test_decryption() {
        let salt = String::from("abc");
        let prefix = String::from("1");

        let expected = vec![
            results::Pair {
                hash: String::from(
                    "6ca13d52ca70c883e0f0bb101e425a89e8624de51db2d2392593af6a84118090",
                ),
                plain: prefix.clone() + "23",
            },
            results::Pair {
                hash: String::from(
                    "97193f3095a7fc166ae10276c083735b41a36abdaac6a33e62d15b7eafa22a67",
                ),
                plain: prefix.clone() + "55",
            },
            results::Pair {
                hash: String::from(
                    "237dd1639d476eda038aff4b83283e3c657a9f38b50c2d7177336d344fe8992e",
                ),
                plain: prefix.clone() + "99",
            },
        ];

        let options = options::Decrypt::<hash::sha256::Hash>::new(
            expected
                .iter()
                .map(|v| <hash::sha256::Hash as std::convert::From<&str>>::from(&v.hash))
                .collect(),
            std::collections::HashSet::new(),
            Some(salt),
            3,
            prefix,
            Some(4),
            Some(options::Device::CPU),
        )
        .unwrap();

        assert_eq!(execute(&options, Channel).unwrap().results, expected);
    }
}
