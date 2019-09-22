extern crate md5;
extern crate sha2;

use super::options;

#[derive(Clone)]
pub struct Input {
    data: std::rc::Rc<std::collections::HashSet<String>>,
}

unsafe impl Send for Input {}

fn hash(algorithm: &options::Algorithm, number: &String) -> String {
    match algorithm {
        options::Algorithm::MD5 => {
            use md5::Digest;
            format!("{:x}", md5::Md5::digest(number.as_bytes()))
        }
        options::Algorithm::SHA256 => {
            use sha2::Digest;
            format!("{:x}", sha2::Sha256::digest(number.as_bytes()))
        }
    }
}

pub fn execute(options: options::Decrypt) {
    let count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(
        options.shared.input.len(),
    ));
    let input = Input {
        data: std::rc::Rc::new(options.shared.input.into_iter().collect()),
    };

    let thread_space = options.number_space / options.thread_count as u64;
    let mut threads =
        Vec::<std::thread::JoinHandle<()>>::with_capacity(options.thread_count as usize);

    for t in 0..options.thread_count {
        let count = count.clone();
        let input = input.clone();

        let algorithm = options.shared.algorithm.clone();
        let prefix = options.prefix.clone();
        let salted_prefix = format!("{}{}", options.shared.salt, options.prefix);
        let length = options.length as usize;
        let this_thread_space = if t < options.thread_count - 1 {
            thread_space
        } else {
            options.number_space - t as u64 * thread_space
        };

        threads.push(std::thread::spawn(move || {
            for n in (t as u64 * this_thread_space)..((t + 1) as u64 * this_thread_space) {
                if (n & 1023) == 1023 {
                    if count.load(std::sync::atomic::Ordering::Acquire) == 0 {
                        return;
                    }
                }

                let number = format!("{}{:02$}", salted_prefix, n, length);
                let hash = hash(&algorithm, &number);
                if input.data.contains(&hash) {
                    count.fetch_sub(1, std::sync::atomic::Ordering::Release);
                    if input.data.len() == 1 {
                        println!("{}{:02$}", prefix, n, length);
                        return;
                    }
                    println!("{} :: {}{:03$}", hash, prefix, n, length);
                }
            }
        }));
    }

    for t in threads {
        t.join().unwrap();
    }
}
