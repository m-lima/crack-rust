use super::opencl;

use crate::channel;
use crate::error;
use crate::hash;
use crate::options;
use crate::results;

use crate::options::SharedAccessor;

fn compute_results<'a, H: hash::Hash>(
    environment: &opencl::Environment<'a, H>,
    input: &[H],
    out_buffer: &ocl::Buffer<opencl::Output>,
    options: &options::Decrypt<H>,
) -> Result<Vec<results::Pair>, error::Error> {
    let mut output = vec![opencl::Output::default(); out_buffer.len()];
    out_buffer
        .read(&mut output)
        .enq()
        .map_err(|err| error!(err; "OpenCL: Failed to read output buffer"))?;

    let mut results = Vec::with_capacity(out_buffer.len());

    for (i, plain) in output.iter().enumerate() {
        if plain.is_valid() {
            let mut output = format!("{}{}", &options.prefix(), plain.printable(environment));
            if let Some(xor) = options.xor().as_ref() {
                unsafe {
                    output
                        .as_bytes_mut()
                        .iter_mut()
                        .zip(xor.iter())
                        .for_each(|(b, x)| *b ^= x);
                }
                output = base64::encode(output);
            }
            results.push(results::Pair::new(input[i].to_string(), output));
        }
    }

    // The kernel will output zeros if nothing is found
    // We should hash this in the CPU to make sure it doesn't match anything
    if results.len() < input.len() {
        let salted_prefix = format!("{}{}", &options.salt(), &options.prefix());

        for i in 0..environment.cpu_iterations() {
            use eytzinger::SliceExt;

            let zeros = opencl::Output::new(0, i).printable(environment);
            let hash = H::digest(&salted_prefix, &zeros);

            if input.eytzinger_search(&hash).is_some() {
                let result = format!("{}{}", &options.prefix(), &zeros);
                results.push(results::Pair::new(hash.to_string(), result));
            }

            if results.len() == input.len() {
                break;
            }
        }
    }

    Ok(results)
}

pub fn execute<H: hash::Hash>(
    options: &options::Decrypt<H>,
    channel: &impl channel::Channel,
) -> Result<results::Summary, error::Error> {
    let time = std::time::Instant::now();

    if (options.input().len() as u64) >= (i32::max_value() as u64) {
        bail!("Input count too large. GPU kernel defines are fixed at i32 (2,147,483,647)");
    }

    let input = options.input_as_eytzinger();

    let environment = opencl::setup_for(options)?;
    let program = environment.make_program()?;

    let in_buffer = if environment.memory() < H::bytes() * input.len() as u64 {
        unsafe { ocl::Buffer::builder().use_host_slice(&input) }
    } else {
        ocl::Buffer::builder().copy_host_slice(&input)
    }
    .flags(ocl::MemFlags::READ_ONLY)
    .len(options.input().len())
    .queue(environment.queue().clone())
    .build()
    .map_err(|err| error!(err; "OpenCL: Failed to create input buffer"))?;

    let out_buffer = ocl::Buffer::builder()
        .flags(ocl::MemFlags::WRITE_ONLY)
        .len(options.input().len())
        .queue(environment.queue().clone())
        .build()
        .map_err(|err| error!(err; "OpenCL: Failed to create output buffer"))?;

    channel.progress(0);
    for i in 0..environment.cpu_iterations() {
        let kernel = ocl::Kernel::builder()
            .program(&program)
            .name("crack")
            .queue(environment.queue().clone())
            .global_work_size(environment.range())
            .arg(&in_buffer)
            .arg(&out_buffer)
            .arg(i)
            .build()
            .map_err(|err| error!(err; "OpenCL: Failed to build kernel"))?;

        unsafe {
            kernel
                .enq()
                .map_err(|err| error!(err; "OpenCL: Failed to enqueue kernel"))?;
        }

        // If we enqueue too many, OpenCL will abort
        // Send every 7th iteration
        if i & 0b111 == 0b111 {
            if channel.should_terminate() {
                break;
            }
            // Allowed because it will always be <= 100
            #[allow(clippy::cast_possible_truncation)]
            channel.progress((i * 100 / environment.cpu_iterations()) as u8);
            environment
                .queue()
                .finish()
                .map_err(|err| error!(err; "OpenCL: Failed to wait for queue segment to finish"))?;
        }
    }

    environment
        .queue()
        .finish()
        .map_err(|err| error!(err; "OpenCL: Failed to wait for queue to finish"))?;

    let results = compute_results(&environment, &input, &out_buffer, options)?;

    for result in &results {
        channel.result(&result.hash, &result.plain);
    }

    Ok(results::Summary {
        total_count: input.len(),
        duration: time.elapsed(),
        hash_count: options.number_space(),
        threads: environment.range(),
        results,
    })
}

#[cfg(all(test, not(gpu_tests_disabled)))]
mod test {
    use super::channel;

    #[derive(Copy, Clone)]
    struct Channel;

    impl channel::Channel for Channel {
        fn progress(&self, _: u8) {}
        fn result(&self, _: &str, _: &str) {}
        fn should_terminate(&self) -> bool {
            false
        }
    }

    mod sha256 {
        use super::super::{execute, hash, options, results};
        use super::Channel;

        #[test]
        fn test_decryption() {
            let salt = String::from("abc");
            let prefix = String::from("1");

            let mut expected = vec![
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
            expected.sort();

            let options = options::DecryptBuilder::<hash::sha256::Hash>::new(
                expected
                    .iter()
                    .map(|v| <hash::sha256::Hash as std::convert::From<&str>>::from(&v.hash))
                    .collect(),
                3,
            )
            .device(options::Device::Gpu)
            .prefix(prefix)
            .salt(salt)
            .build()
            .unwrap();

            let mut results = execute(&options, &Channel).unwrap().results;
            results.sort();

            assert_eq!(results, expected);
        }

        #[test]
        fn test_xor_decryption() {
            let salt = String::from("abc");
            let prefix = String::from("1");
            let xor = vec![3, 4, 5, 6];

            let mut expected = vec![
                results::Pair {
                    hash: String::from(
                        "f3b90305e926c8d7ad0c4a1750532341875df1aeecde3c508bfbe4be1969180c",
                    ),
                    plain: String::from("MjY2"),
                },
                results::Pair {
                    hash: String::from(
                        "836bfc1d576b5a04e1688cd4603f42a67dda7e31c2e7adb5142eb4c4e898a66d",
                    ),
                    plain: String::from("MjEw"),
                },
                results::Pair {
                    hash: String::from(
                        "8823993be0da4a4f07aa33dd3ebfe1a33b36f01d5d11d64e93235119e8b3468f",
                    ),
                    plain: String::from("Mj08"),
                },
            ];
            expected.sort();

            let options = options::DecryptBuilder::<hash::sha256::Hash>::new(
                expected
                    .iter()
                    .map(|v| <hash::sha256::Hash as std::convert::From<&str>>::from(&v.hash))
                    .collect(),
                3,
            )
            .device(options::Device::Gpu)
            .prefix(prefix)
            .salt(salt)
            .xor(xor)
            .build()
            .unwrap();

            let mut results = execute(&options, &Channel).unwrap().results;
            results.sort();

            assert_eq!(results, expected);
        }
    }

    mod md5 {
        use super::super::{execute, hash, options, results};
        use super::Channel;

        #[test]
        fn test_decryption() {
            let salt = String::from("abc");
            let prefix = String::from("1");

            let mut expected = vec![
                results::Pair {
                    hash: String::from("e99a18c428cb38d5f260853678922e03"),
                    plain: prefix.clone() + "23",
                },
                results::Pair {
                    hash: String::from("6b14d696623c7b26c275da041719ce53"),
                    plain: prefix.clone() + "55",
                },
                results::Pair {
                    hash: String::from("361ac235e1e08be7325a8ced898e6ff4"),
                    plain: prefix.clone() + "99",
                },
            ];
            expected.sort();

            let options = options::DecryptBuilder::<hash::md5::Hash>::new(
                expected
                    .iter()
                    .map(|v| <hash::md5::Hash as std::convert::From<&str>>::from(&v.hash))
                    .collect(),
                3,
            )
            .device(options::Device::Gpu)
            .prefix(prefix)
            .salt(salt)
            .build()
            .unwrap();

            let mut results = execute(&options, &Channel).unwrap().results;
            results.sort();

            assert_eq!(results, expected);
        }

        #[test]
        fn test_xor_decryption() {
            let salt = String::from("abc");
            let prefix = String::from("1");
            let xor = vec![3, 4, 5, 6];

            let mut expected = vec![
                results::Pair {
                    hash: String::from("7900c0f65c087c03458293d7bb172ed1"),
                    plain: String::from("MjY2"),
                },
                results::Pair {
                    hash: String::from("7c1b8268077c6a9439fb82434dd5a5af"),
                    plain: String::from("MjEw"),
                },
                results::Pair {
                    hash: String::from("dd9eac6ed5ce1d8c5a645b4642ca1cd8"),
                    plain: String::from("Mj08"),
                },
            ];
            expected.sort();

            let options = options::DecryptBuilder::<hash::md5::Hash>::new(
                expected
                    .iter()
                    .map(|v| <hash::md5::Hash as std::convert::From<&str>>::from(&v.hash))
                    .collect(),
                3,
            )
            .device(options::Device::Gpu)
            .prefix(prefix)
            .salt(salt)
            .xor(xor)
            .build()
            .unwrap();

            let mut results = execute(&options, &Channel).unwrap().results;
            results.sort();

            assert_eq!(results, expected);
        }
    }
}
