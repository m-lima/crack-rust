use crate::error;
use crate::hash;
use crate::options;

use crate::options::SharedAccessor;

static MAX_GPU_LENGTH: u8 = 7;

pub(super) fn setup_for<H: hash::Hash>(
    options: &options::Decrypt<H>,
) -> Result<Environment<'_, H>, error::Error> {
    Ok(Environment {
        options,
        configuration: Configuration::new()?,
        kernel_parameters: KernelParameters::from(options),
    })
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
        let salted_prefix = format!("{}{}", &self.options.salt(), &self.options.prefix());
        let source = source::template::<H>()?.with_prefix(&salted_prefix);

        ocl::Program::builder()
            .source(source.to_string())
            .devices(self.configuration.device)
            .cmplr_def("CONST_BEGIN", salted_prefix.len() as i32)
            .cmplr_def(
                "CONST_END",
                i32::from(salted_prefix.len() as u8 + self.options.length()),
            )
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
}

struct Configuration {
    device: ocl::Device,
    context: ocl::Context,
    queue: ocl::Queue,
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

        Ok(Self {
            device,
            context,
            queue,
        })
    }

    fn first_gpu() -> Result<(ocl::Platform, ocl::Device), error::Error> {
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

        match out.first() {
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
            length_on_cpu_iterations,
            range,
            length_on_gpu_kernel,
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
    use crate::error;
    use crate::hash;

    static MD5: &str = include_str!("../../cl/md5.cl");
    static SHA256: &str = include_str!("../../cl/sha256.cl");

    pub(super) struct SourceTemplate(&'static str);
    pub(super) struct Source(String);

    pub(super) fn template<H: hash::Hash>() -> Result<SourceTemplate, error::Error> {
        // TODO: Update with type specialization when available
        if std::any::TypeId::of::<H>() == std::any::TypeId::of::<hash::md5::Hash>() {
            Ok(SourceTemplate(MD5))
        } else if std::any::TypeId::of::<H>() == std::any::TypeId::of::<hash::sha256::Hash>() {
            Ok(SourceTemplate(SHA256))
        } else {
            Err(error!("Algorithm not supported for GPU execution"))
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
    }
}
