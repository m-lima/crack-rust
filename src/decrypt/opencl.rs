use crate::error;
use crate::hash;
use crate::options;

use crate::options::SharedAccessor;

const MAX_GPU_LENGTH: u8 = 7;
const BASE64: &str = include_str!("../../cl/base64.cl");
const PREPARE: &str = include_str!("../../cl/prepare.cl");

pub(super) fn setup_for<H: hash::Hash>(
    options: &options::Decrypt<H>,
) -> Result<Environment<'_, H>, error::Error> {
    Ok(Environment {
        options,
        configuration: Configuration::new()?,
        kernel_parameters: KernelParameters::from(options),
    })
}

fn calculate_base64_len(length: usize) -> usize {
    if length % 3 > 0 {
        (length / 3 + 1) << 2
    } else {
        (length / 3) << 2
    }
}

pub(super) struct Environment<'a, H: hash::Hash> {
    options: &'a options::Decrypt<H>, // The environment is locked to the options. It must not change
    configuration: Configuration,
    kernel_parameters: KernelParameters,
}

impl<'a, H: hash::Hash> Environment<'a, H> {
    // Allowed because of previous check for options.shared.input.len() <= i32.max_value()
    // Allowed because salted prefix is limited in size
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    pub(super) fn make_program(&self) -> Result<ocl::Program, error::Error> {
        let salted_prefix = format!("{}{}", self.options.salt(), self.options.prefix());
        let end = i32::from(salted_prefix.len() as u8 + self.options.length());

        let mut builder = ocl::Program::builder();
        builder.source(PREPARE);

        if let Some(xor) = self.options.xor().as_ref() {
            let length = usize::from(self.options.length() + self.options.prefix_length());
            let source = source::template::<H>().with_prefix_and_xor(
                &salted_prefix,
                self.options.salt().len(),
                length,
                xor,
            );

            builder
                .source(BASE64)
                .source(source.to_string())
                .cmplr_def("CONST_BASE64_BEGIN", self.options.salt().len() as i32)
                .cmplr_def(
                    "CONST_LENGTH",
                    (calculate_base64_len(length) + self.options.salt().len()) as i32,
                );
        } else {
            let source = source::template::<H>().with_prefix(&salted_prefix);
            builder
                .source(source.to_string())
                .cmplr_def("CONST_LENGTH", end);
        }

        builder
            .devices(self.configuration.device)
            .cmplr_def("CONST_BEGIN", salted_prefix.len() as i32)
            .cmplr_def("CONST_END", end)
            .cmplr_def("CONST_TARGET_COUNT", self.options.input().len() as i32)
            .cmplr_def(
                self.kernel_parameters.cpu_length_definition(),
                i32::from(self.kernel_parameters.length_on_cpu_iterations),
            )
            .build(&self.configuration.context)
            .map_err(|err| error!(err; "OpenCL: Failed to build program"))
    }

    pub(super) fn queue(&self) -> &ocl::Queue {
        &self.configuration.queue
    }

    pub(super) fn cpu_iterations(&self) -> u32 {
        self.kernel_parameters.cpu_iterations
    }

    pub(super) fn range(&self) -> u32 {
        self.kernel_parameters.range
    }

    pub(super) fn memory(&self) -> u64 {
        self.configuration.memory
    }
}

struct Configuration {
    device: ocl::Device,
    context: ocl::Context,
    queue: ocl::Queue,
    memory: u64,
}

impl Configuration {
    fn new() -> Result<Self, error::Error> {
        let (platform, device) = Self::first_gpu()?;
        let context = ocl::Context::builder()
            .platform(platform)
            .devices(device)
            .build()
            .map_err(|err| error!(err; "OpenCL: Failed to create context"))?;

        let queue = ocl::Queue::new(&context, device, None)
            .map_err(|err| error!(err; "OpenCL: Failed to create queue"))?;

        let memory = match device.info(ocl::core::DeviceInfo::LocalMemSize) {
            Ok(ocl::core::DeviceInfoResult::LocalMemSize(memory)) => memory,
            Ok(_) => bail!("OpenCL: Failed query local memory size"),
            Err(err) => bail!(err; "OpenCL: Failed query local memory size"),
        };

        Ok(Self {
            device,
            context,
            queue,
            memory,
        })
    }

    fn first_gpu() -> Result<(ocl::Platform, ocl::Device), error::Error> {
        let mut devices = Vec::new();
        for platform in ocl::Platform::list() {
            if let Ok(all_devices) = ocl::Device::list_all(&platform) {
                for device in all_devices {
                    devices.push((platform, device));
                }
            }
        }

        // Prefer GPU
        devices.sort_by(|&(_, ref a), &(_, ref b)| {
            use ocl::core::{DeviceInfo, DeviceInfoResult};

            if let (Ok(DeviceInfoResult::Type(a_type)), Ok(DeviceInfoResult::Type(b_type))) =
                (a.info(DeviceInfo::Type), b.info(DeviceInfo::Type))
            {
                let cmp = b_type.cmp(&a_type);
                if std::cmp::Ordering::Equal == cmp {
                    if let (
                        Ok(DeviceInfoResult::GlobalMemSize(a_mem)),
                        Ok(DeviceInfoResult::GlobalMemSize(b_mem)),
                    ) = (
                        a.info(DeviceInfo::GlobalMemSize),
                        b.info(DeviceInfo::GlobalMemSize),
                    ) {
                        b_mem.cmp(&a_mem)
                    } else {
                        cmp
                    }
                } else {
                    cmp
                }
            } else {
                std::cmp::Ordering::Equal
            }
        });

        match devices.first() {
            Some(pair) => Ok(*pair),
            None => Err(error!("OpenCL: Failed to find any OpenCL devices")),
        }
    }
}

struct KernelParameters {
    cpu_iterations: u32,
    range: u32,
    length_on_gpu_kernel: u8,
    length_on_cpu_iterations: u8,
}

impl KernelParameters {
    fn from<H: hash::Hash>(options: &options::Decrypt<H>) -> Self {
        let length_on_cpu_iterations = if MAX_GPU_LENGTH > options.length() {
            0
        } else {
            options.length() - MAX_GPU_LENGTH
        };
        let cpu_iterations = 10_u32.pow(u32::from(length_on_cpu_iterations));

        // Allowed because min(MAX_GPU_RANGE, ...) will always fit in u32
        #[allow(clippy::cast_possible_truncation)]
        let range = std::cmp::min(
            10_u64.pow(u32::from(MAX_GPU_LENGTH)),
            options.number_space(),
        ) as u32;

        let length_on_gpu_kernel = options.length() - length_on_cpu_iterations;

        Self {
            cpu_iterations,
            range,
            length_on_gpu_kernel,
            length_on_cpu_iterations,
        }
    }

    fn cpu_length_definition(&self) -> &'static str {
        if self.cpu_iterations > 1 {
            "CONST_LENGTH_ON_CPU"
        } else {
            "INVALID_VALUE"
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub(super) struct Output {
    data: [u32; 2],
}

unsafe impl ocl::OclPrm for Output {}

impl Output {
    pub(super) fn new(from_kernel: u32, from_iterations: u32) -> Self {
        Self {
            data: [from_kernel, from_iterations],
        }
    }

    pub(super) fn is_valid(self) -> bool {
        self.data[0] > 0
    }

    pub(super) fn printable<H: hash::Hash>(self, environment: &Environment<'_, H>) -> String {
        if environment.cpu_iterations() > 1 {
            format!(
                "{:02$}{:03$}",
                self.data[1],
                self.data[0],
                usize::from(environment.kernel_parameters.length_on_cpu_iterations),
                usize::from(environment.kernel_parameters.length_on_gpu_kernel)
            )
        } else {
            format!(
                "{:01$}",
                self.data[0],
                usize::from(environment.kernel_parameters.length_on_gpu_kernel)
            )
        }
    }
}

mod source {
    use crate::hash;

    const MD5: &str = include_str!("../../cl/md5.cl");
    const SHA256: &str = include_str!("../../cl/sha256.cl");

    pub(super) struct SourceTemplate(&'static str);
    pub(super) struct Source(String);

    pub(super) fn template<H: hash::Hash>() -> SourceTemplate {
        match H::algorithm() {
            hash::Algorithm::md5 => SourceTemplate(MD5),
            hash::Algorithm::sha256 => SourceTemplate(SHA256),
        }
    }

    impl SourceTemplate {
        pub(super) fn with_prefix(&self, salted_prefix: &str) -> Source {
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

        pub(super) fn with_prefix_and_xor(
            &self,
            salted_prefix: &str,
            salt_len: usize,
            length: usize,
            xor: &[u8],
        ) -> Source {
            let mut prefix_code = String::new();
            for (i, c) in salted_prefix.chars().enumerate() {
                prefix_code.push_str(format!("value.bytes[{}] = \'{}\';", i, c).as_str());
            }

            let mut xor_code = String::new();
            for (i, c) in xor.iter().take(length).enumerate() {
                xor_code.push_str(format!("value.bytes[{}] ^= {};", i + salt_len, c).as_str());
            }

            let mut output = String::new();
            for line in self.0.lines() {
                if line.ends_with("// %%PREFIX%%") {
                    output.push_str(prefix_code.as_str());
                } else if line.ends_with("// %%XOR%%") {
                    output.push_str(xor_code.as_str());
                } else {
                    output.push_str(line);
                };
                output.push('\n');
            }

            Source(output)
        }
    }

    impl Source {
        pub(super) fn to_string(&self) -> &String {
            &self.0
        }
    }

    #[cfg(test)]
    mod test {
        use super::SourceTemplate;

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

        #[test]
        fn test_code_injection() {
            let src = r#"
One line
Another line
// %%PREFIX%%
// %%PREFIX%% 
   // %%XOR%%
Final line"#;

            let expected = r#"
One line
Another line
value.bytes[0] = '0';value.bytes[1] = '1';value.bytes[2] = '2';
// %%PREFIX%% 
value.bytes[1] ^= 0;value.bytes[2] ^= 1;
Final line
"#;

            let output = SourceTemplate(src).with_prefix_and_xor("012", 1, 2, &[0, 1, 2, 3]);
            assert_eq!(output.to_string(), expected);
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn base64_length() {
        use super::calculate_base64_len;

        assert_eq!(calculate_base64_len(0), 0);
        assert_eq!(calculate_base64_len(1), 4);
        assert_eq!(calculate_base64_len(2), 4);
        assert_eq!(calculate_base64_len(3), 4);
        assert_eq!(calculate_base64_len(4), 8);
        assert_eq!(calculate_base64_len(5), 8);
        assert_eq!(calculate_base64_len(6), 8);
        assert_eq!(calculate_base64_len(7), 12);
        assert_eq!(calculate_base64_len(8), 12);
        assert_eq!(calculate_base64_len(9), 12);
        assert_eq!(calculate_base64_len(10), 16);
    }
}
