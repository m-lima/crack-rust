use crate::decrypt;
use crate::error;
use crate::hash;
use crate::secrets;
use crate::Input;

const SALT_ENV: &str = "HASHER_SALT";

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Device {
    CPU,
    GPU,
}

impl Device {
    pub fn variants() -> &'static [&'static str] {
        &["cpu", "gpu"]
    }
}

impl std::fmt::Display for Device {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CPU => write!(fmt, "CPU"),
            Self::GPU => write!(fmt, "GPU"),
        }
    }
}

pub struct Shared<T: Input> {
    input: std::collections::HashSet<T>,
    salt: String,
}

impl<T: Input> Shared<T> {
    fn new(
        input: std::collections::HashSet<T>,
        maybe_salt: Option<String>,
    ) -> Result<Self, error::Error> {
        if input.is_empty() {
            Err(error!("No valid input provided"))
        } else {
            Ok(Self {
                input,
                salt: salt(maybe_salt),
            })
        }
    }
}

pub trait SharedAccessor<T: Input> {
    fn shared(&self) -> &Shared<T>;

    fn input(&self) -> &std::collections::HashSet<T> {
        &self.shared().input
    }

    fn salt(&self) -> &str {
        &self.shared().salt
    }
}

pub struct Encrypt<H: hash::Hash> {
    shared: Shared<String>,
    _phantom: std::marker::PhantomData<H>,
}

impl<H: hash::Hash> Encrypt<H> {
    pub fn new(
        input: std::collections::HashSet<String>,
        maybe_salt: Option<String>,
    ) -> Result<Self, error::Error> {
        Ok(Self {
            shared: Shared::new(input, maybe_salt)?,
            _phantom: std::marker::PhantomData::<H>::default(),
        })
    }
}

impl<H: hash::Hash> SharedAccessor<String> for Encrypt<H> {
    fn shared(&self) -> &Shared<String> {
        &self.shared
    }
}

pub struct Decrypt<H: hash::Hash> {
    shared: Shared<H>,
    device: Device,
    files: std::collections::HashSet<std::path::PathBuf>,
    length: u8,
    number_space: u64,
    prefix: String,
    threads: u8,
    xor: Option<Vec<u8>>,
}

impl<H: hash::Hash> SharedAccessor<H> for Decrypt<H> {
    fn shared(&self) -> &Shared<H> {
        &self.shared
    }
}

impl<H: hash::Hash> Decrypt<H> {
    pub fn device(&self) -> Device {
        self.device
    }

    pub fn files(&self) -> &std::collections::HashSet<std::path::PathBuf> {
        &self.files
    }

    pub fn length(&self) -> u8 {
        self.length
    }

    pub fn number_space(&self) -> u64 {
        self.number_space
    }

    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    pub fn threads(&self) -> u8 {
        self.threads
    }

    pub fn xor(&self) -> &Option<Vec<u8>> {
        &self.xor
    }

    pub fn input_as_eytzinger(&self) -> Vec<H> {
        use eytzinger::{permutation::InplacePermutator, SliceExt};
        let mut data = self
            .shared
            .input
            .iter()
            .map(Clone::clone)
            .collect::<Vec<_>>();
        data.sort_unstable();
        data.as_mut_slice().eytzingerize(&mut InplacePermutator);
        data
    }

    // Allowed because prefix is always less than total_size
    #[allow(clippy::cast_possible_truncation)]
    pub fn prefix_length(&self) -> u8 {
        self.prefix.len() as u8
    }
}

pub struct DecryptBuilder<H: hash::Hash> {
    input: std::collections::HashSet<H>,
    length: u8,
    device: Option<Device>,
    files: Option<std::collections::HashSet<std::path::PathBuf>>,
    prefix: Option<String>,
    salt: Option<String>,
    threads: Option<u8>,
    xor: Option<Vec<u8>>,
}

impl<H: hash::Hash> DecryptBuilder<H> {
    pub fn new(input: std::collections::HashSet<H>, length: u8) -> Self {
        Self {
            input,
            length,
            device: None,
            files: None,
            prefix: None,
            salt: None,
            threads: None,
            xor: None,
        }
    }

    pub fn device(mut self, device: impl Into<Option<Device>>) -> Self {
        self.device = device.into();
        self
    }

    pub fn files(
        mut self,
        files: impl Into<Option<std::collections::HashSet<std::path::PathBuf>>>,
    ) -> Self {
        self.files = files.into();
        self
    }

    pub fn prefix(mut self, prefix: impl Into<Option<String>>) -> Self {
        self.prefix = prefix.into();
        self
    }

    pub fn salt(mut self, salt: impl Into<Option<String>>) -> Self {
        self.salt = salt.into();
        self
    }

    pub fn threads(mut self, threads: impl Into<Option<u8>>) -> Self {
        self.threads = threads.into();
        self
    }

    pub fn xor(mut self, xor: impl Into<Option<Vec<u8>>>) -> Self {
        self.xor = xor.into();
        self
    }

    pub fn build(self) -> Result<Decrypt<H>, error::Error> {
        let prefix_len = self.prefix.as_ref().map_or(0, String::len);
        if prefix_len > usize::from(self.length) {
            bail!("Prefix is too long");
        }

        if self.xor.as_ref().map_or(usize::MAX, Vec::len) < usize::from(self.length) {
            bail!("XOR mask is not long enough");
        }

        // Allowed because the length was checked for overflow
        #[allow(clippy::cast_possible_truncation)]
        let variable_length = self.length - prefix_len as u8;
        let number_space = 10_u64.pow(u32::from(variable_length));
        let threads = threads(self.threads, number_space);
        let device = self.derive_device(number_space, threads);

        Ok(Decrypt {
            shared: Shared::new(self.input, self.salt)?,
            device,
            files: self
                .files
                .unwrap_or_else(|| std::collections::HashSet::with_capacity(0)),
            length: variable_length,
            number_space,
            prefix: self.prefix.unwrap_or_else(String::new),
            threads,
            xor: self.xor,
        })
    }

    fn derive_device(&self, number_space: u64, threads: u8) -> Device {
        if let Some(device) = self.device {
            device
        } else if number_space > u64::from(threads) * decrypt::OPTIMAL_HASHES_PER_THREAD {
            Device::GPU
        } else {
            Device::CPU
        }
    }
}

pub enum Mode<H: hash::Hash> {
    Encrypt(Encrypt<H>),
    Decrypt(Decrypt<H>),
}

impl<H: hash::Hash> Mode<H> {
    pub fn input_len(&self) -> usize {
        match &self {
            Self::Encrypt(mode) => mode.shared.input.len(),
            Self::Decrypt(mode) => mode.shared.input.len(),
        }
    }
}

// Allowed because the count was checked for overflow
#[allow(clippy::cast_possible_truncation)]
fn threads(requested_count: Option<u8>, number_space: u64) -> u8 {
    let threads = std::cmp::min(
        number_space / decrypt::OPTIMAL_HASHES_PER_THREAD + 1,
        requested_count.map_or_else(
            || {
                let cores = num_cpus::get();
                if cores > usize::from(u8::max_value()) {
                    u64::from(u8::max_value())
                } else {
                    cores as u64
                }
            },
            u64::from,
        ),
    );

    // Due to `min`, it will always be less than u8::MAX (255)
    threads as u8
}

// TODO: The env var should belong to CLI
fn salt(maybe_salt: Option<String>) -> String {
    maybe_salt
        .unwrap_or_else(|| std::env::var(SALT_ENV).unwrap_or_else(|_| String::from(secrets::SALT)))
}
