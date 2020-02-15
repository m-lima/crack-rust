#[cfg(test)]
mod tests;

#[derive(Debug)]
pub enum Error {
    ParseError,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Hash {
    hi: u128,
    lo: u128,
}

impl Default for Hash {
    fn default() -> Self {
        Self { hi: 0, lo: 0 }
    }
}

impl std::ops::Shl<u8> for Hash {
    type Output = Self;

    fn shl(self, rhs: u8) -> Self {
        Self {
            lo: self.lo << rhs,
            hi: (self.hi << rhs) | (self.lo >> (128 - rhs)),
        }
    }
}

impl std::ops::BitOr<u8> for Hash {
    type Output = Self;

    // Allowed because of hot path and certainty of no loss
    #[allow(clippy::cast_lossless)]
    fn bitor(self, rhs: u8) -> Self {
        Self {
            lo: self.lo | rhs as u128,
            hi: self.hi,
        }
    }
}

impl std::fmt::LowerHex for Hash {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        {
            let bytes = &unsafe { std::mem::transmute::<u128, [u8; 16]>(self.lo) };
            for b in bytes {
                write!(fmt, "{:02x}", b)?;
            }
        }
        {
            let bytes = &unsafe { std::mem::transmute::<u128, [u8; 16]>(self.hi) };
            for b in bytes {
                write!(fmt, "{:02x}", b)?;
            }
        }
        Ok(())
    }
}

impl std::fmt::Binary for Hash {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        {
            let bytes = &unsafe { std::mem::transmute::<u128, [u8; 16]>(self.lo) };
            for b in bytes {
                write!(fmt, "{:08b}", b)?;
            }
        }
        {
            let bytes = &unsafe { std::mem::transmute::<u128, [u8; 16]>(self.hi) };
            for b in bytes {
                write!(fmt, "{:08b}", b)?;
            }
        }
        Ok(())
    }
}

pub trait Into {
    fn into_hash(&self) -> Result<Hash, Error>;
}

impl Into for String {
    fn into_hash(&self) -> Result<Hash, Error> {
        let mut hash = Hash::default();
        for (i, c) in self.chars().rev().enumerate() {
            let int = match c as u8 {
                c if c >= 0x30 && c < 0x3a => c - 0x30,       // decimal
                c if c >= 0x41 && c < 0x47 => c - 0x41 + 0xa, // uppercase
                c if c >= 0x61 && c < 0x67 => c - 0x61 + 0xa, // lowercase
                _ => return Err(Error::ParseError),
            };
            if i % 2 == 0 {
                hash = hash << 8;
                hash = hash | int;
            } else {
                hash = hash | (int << 4);
            }
        }
        Ok(hash)
    }
}

pub fn compute<D: digest::Digest>(salted_prefix: &str, number: &str) -> Hash {
    use std::convert::TryInto;
    let mut digest = D::new();
    digest.input(salted_prefix.as_bytes());
    digest.input(number.as_bytes());
    let result = digest.result();
    Hash {
        lo: unsafe { std::mem::transmute::<[u8; 16], u128>(result[00..16].try_into().unwrap()) },
        hi: unsafe { std::mem::transmute::<[u8; 16], u128>(result[16..32].try_into().unwrap()) },
    }
}
