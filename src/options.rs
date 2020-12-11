use crate::cli::print;
use crate::hash;
use crate::Input;

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
    printer: print::Printer,
}

pub trait SharedAccessor<T: Input> {
    fn shared(&self) -> &Shared<T>;

    fn input(&self) -> &std::collections::HashSet<T> {
        &self.shared().input
    }

    fn salt(&self) -> &str {
        &self.shared().salt
    }

    fn printer(&self) -> print::Printer {
        self.shared().printer
    }
}

pub struct Encrypt<H: hash::Hash> {
    shared: Shared<String>,
    _phantom: std::marker::PhantomData<H>,
}

impl<H: hash::Hash> Encrypt<H> {
    pub fn new(
        input: std::collections::HashSet<String>,
        salt: String,
        printer: print::Printer,
    ) -> Self {
        Self {
            shared: Shared {
                input,
                salt,
                printer,
            },
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
        salt: String,
        printer: print::Printer,
        files: std::collections::HashSet<std::path::PathBuf>,
        length: u8,
        threads: u8,
        number_space: u64,
        prefix: String,
        device: Device,
    ) -> Self {
        Self {
            shared: Shared {
                input,
                salt,
                printer,
            },
            files,
            length,
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

    pub fn printer(&self) -> print::Printer {
        match &self {
            Self::Encrypt(mode) => mode.printer(),
            Self::Decrypt(mode) => mode.printer(),
        }
    }
}
