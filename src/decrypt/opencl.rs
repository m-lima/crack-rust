use crate::options;

pub mod program {
    static MD5: &str = include!(concat!(env!("OUT_DIR"), "/cl/md5.rs"));
    static SHA256: &str = include!(concat!(env!("OUT_DIR"), "/cl/sha256.rs"));

    pub struct SourceTemplate<'a>(&'a str);
    pub struct Source(String);

    pub fn from<'a>(algorithm: &super::options::Algorithm) -> SourceTemplate<'a> {
        SourceTemplate(match algorithm {
            super::options::Algorithm::MD5 => MD5,
            super::options::Algorithm::SHA256 => SHA256,
        })
    }

    impl<'a> SourceTemplate<'a> {
        pub fn with_prefix(&self, salted_prefix: &str) -> Source {
            let mut injected_code = String::new();
            for (i, c) in salted_prefix.chars().enumerate() {
                injected_code.push_str(format!("value.bytes[{}] = \'{}\';", i, c).as_str());
            }

            let mut output = String::new();
            for line in self.0.lines() {
                if line.ends_with("// %%PREFIX%%") {
                    output.push_str(injected_code.as_str());
                } else {
                    output.push_str(line);
                };
                output.push('\n');
            }

            Source(output)
        }
    }

    impl Source {
        pub fn to_string(&self) -> &String {
            &self.0
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_prefix_injection() {
            let src = r#"
One line
Another line
// %%PREFIX%%
// %%PREFIX%% 
Final line"#;

            let expected = r#"
One line
Another line
value.bytes[0] = '0';value.bytes[1] = '1';value.bytes[2] = '2';
// %%PREFIX%% 
Final line
"#;

            let output = SourceTemplate(src).with_prefix("012");
            assert_eq!(output.to_string(), expected);
        }
    }
}

pub(super) struct Configuration {
    device: ocl::Device,
    context: ocl::Context,
    queue: ocl::Queue,
}

impl Configuration {
    pub fn device(&self) -> ocl::Device {
        self.device
    }

    pub fn context(&self) -> &ocl::Context {
        &self.context
    }

    pub fn queue(&self) -> &ocl::Queue {
        &self.queue
    }
}

fn first_gpu() -> (ocl::Platform, ocl::Device) {
    let mut out = Vec::new();
    for platform in ocl::Platform::list() {
        if let Ok(all_devices) = ocl::Device::list_all(&platform) {
            for device in all_devices {
                out.push((platform, device));
            }
        }
    }

    // Prefer GPU
    out.sort_by(|&(_, ref a), &(_, ref b)| {
        let a_type = a.info(ocl::core::DeviceInfo::Type);
        let b_type = b.info(ocl::core::DeviceInfo::Type);
        if let (
            Ok(ocl::core::DeviceInfoResult::Type(a_type)),
            Ok(ocl::core::DeviceInfoResult::Type(b_type)),
        ) = (a_type, b_type)
        {
            b_type.cmp(&a_type)
        } else {
            (0).cmp(&0)
        }
    });

    if out.first().is_none() {
        eprintln!("OpenCL: Failed to find any OpenCL devices");
        std::process::exit(-1);
    }
    *out.first().unwrap()
}

pub(super) fn setup() -> Configuration {
    let (platform, device) = first_gpu();
    let context = ocl::Context::builder()
        .platform(platform)
        .devices(device)
        .build()
        .unwrap_or_else(|err| {
            eprintln!("OpenCL: Failed to create context: {}", err);
            std::process::exit(-1);
        });
    let queue = ocl::Queue::new(&context, device, None).unwrap();

    Configuration {
        device,
        context,
        queue,
    }
}
