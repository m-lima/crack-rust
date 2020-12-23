use crate::decrypt;
use crate::hash;
use crate::secrets;
use crate::Input;

static SALT_ENV: &str = "HASHER_SALT";

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
    fn new(input: std::collections::HashSet<T>, maybe_salt: Option<String>) -> Self {
        Self {
            input,
            salt: salt(maybe_salt),
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
    pub fn new(input: std::collections::HashSet<String>, maybe_salt: Option<String>) -> Self {
        Self {
            shared: Shared::new(input, maybe_salt),
            _phantom: std::marker::PhantomData::<H>::default(),
        }
    }
}

impl<H: hash::Hash> SharedAccessor<String> for Encrypt<H> {
    fn shared(&self) -> &Shared<String> {
        &self.shared
    }
}

pub struct Decrypt<H: hash::Hash> {
    shared: Shared<H>,
    files: std::collections::HashSet<std::path::PathBuf>,
    length: u8,
    threads: u8,
    number_space: u64,
    prefix: String,
    device: Device,
}

impl<H: hash::Hash> SharedAccessor<H> for Decrypt<H> {
    fn shared(&self) -> &Shared<H> {
        &self.shared
    }
}

impl<H: hash::Hash> Decrypt<H> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        input: std::collections::HashSet<H>,
        files: std::collections::HashSet<std::path::PathBuf>,
        maybe_salt: Option<String>,
        length: u8,
        prefix: String,
        maybe_threads: Option<u8>,
        maybe_device: Option<Device>,
    ) -> Self {
        // Allowed because the length was checked for overflow
        #[allow(clippy::cast_possible_truncation)]
        let variable_length = length - prefix.len() as u8;
        let number_space = 10_u64.pow(u32::from(variable_length));
        let threads = threads(maybe_threads, number_space);
        let device = device(maybe_device, number_space, threads);

        Self {
            shared: Shared::new(input, maybe_salt),
            files,
            length: variable_length,
            threads,
            number_space,
            prefix,
            device,
        }
    }

    pub fn files(&self) -> &std::collections::HashSet<std::path::PathBuf> {
        &self.files
    }

    pub fn length(&self) -> u8 {
        self.length
    }

    pub fn threads(&self) -> u8 {
        self.threads
    }

    pub fn number_space(&self) -> u64 {
        self.number_space
    }

    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    pub fn device(&self) -> Device {
        self.device
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
                    panic!("Too many cores.. You have one powerful computer!");
                }
                cores as u64
            },
            u64::from,
        ),
    );

    // Due to `min`, it will always be less than u8::MAX (255)
    threads as u8
}

fn salt(maybe_salt: Option<String>) -> String {
    maybe_salt
        .unwrap_or_else(|| std::env::var(SALT_ENV).unwrap_or_else(|_| String::from(secrets::SALT)))
}

fn device(maybe_device: Option<Device>, number_space: u64, threads: u8) -> Device {
    if let Some(device) = maybe_device {
        device
    } else if number_space > u64::from(threads) * decrypt::OPTIMAL_HASHES_PER_THREAD {
        Device::GPU
    } else {
        Device::CPU
    }
}
