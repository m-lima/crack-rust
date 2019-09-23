#[cfg(test)]
mod tests;

#[derive(Debug)]
pub enum HashError {
    ParseError,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Hash {
    pub hi: u128,
    pub lo: u128,
}

impl Hash {
    pub fn default() -> Hash {
        Hash { hi: 0, lo: 0 }
    }
}

impl std::hash::Hash for Hash {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u128(self.hi);
        state.write_u128(self.lo);
    }
}

impl Ord for Hash {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        let o = match self.hi.cmp(&rhs.hi) {
            std::cmp::Ordering::Equal => self.lo.cmp(&rhs.lo),
            o => o,
        };
        o
    }
}

impl PartialOrd for Hash {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}

impl std::ops::Shl<u8> for Hash {
    type Output = Self;

    fn shl(self, rhs: u8) -> Hash {
        Hash {
            lo: self.lo << rhs,
            hi: (self.hi << rhs) | (self.lo >> (128 - rhs)),
        }
    }
}

impl std::ops::Add<u8> for Hash {
    type Output = Self;

    fn add(self, rhs: u8) -> Hash {
        let lo = std::num::Wrapping(self.lo) + std::num::Wrapping(rhs as u128);
        Hash {
            lo: lo.0,
            hi: self.hi + if self.lo > lo.0 { 1 } else { 0 },
        }
    }
}

impl std::ops::Add<u64> for Hash {
    type Output = Self;

    fn add(self, rhs: u64) -> Hash {
        let lo = std::num::Wrapping(self.lo) + std::num::Wrapping(rhs as u128);
        Hash {
            lo: lo.0,
            hi: self.hi + if self.lo > lo.0 { 1 } else { 0 },
        }
    }
}

impl std::ops::BitAnd<u8> for Hash {
    type Output = Self;

    fn bitand(self, rhs: u8) -> Hash {
        Hash {
            lo: self.lo & rhs as u128,
            hi: self.hi,
        }
    }
}

impl std::ops::BitAnd<u64> for Hash {
    type Output = Self;

    fn bitand(self, rhs: u64) -> Hash {
        Hash {
            lo: self.lo & rhs as u128,
            hi: self.hi,
        }
    }
}

impl std::ops::BitOr<u8> for Hash {
    type Output = Self;

    fn bitor(self, rhs: u8) -> Hash {
        Hash {
            lo: self.lo | rhs as u128,
            hi: self.hi,
        }
    }
}

impl std::ops::BitOr<u64> for Hash {
    type Output = Self;

    fn bitor(self, rhs: u64) -> Hash {
        Hash {
            lo: self.lo | rhs as u128,
            hi: self.hi,
        }
    }
}

impl std::fmt::LowerHex for Hash {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
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
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
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

pub trait IntoHash {
    fn into_hash(&self) -> Result<Hash, HashError>;
}

impl IntoHash for String {
    fn into_hash(&self) -> Result<Hash, HashError> {
        let mut hash = Hash::default();
        for (i, c) in self.chars().rev().enumerate() {
            let int = match c as u8 {
                c if c >= 0x30 && c < 0x3a => c - 0x30,       // decimal
                c if c >= 0x41 && c < 0x47 => c - 0x41 + 0xa, // uppercase
                c if c >= 0x61 && c < 0x67 => c - 0x61 + 0xa, // lowercase
                _ => return Err(HashError::ParseError),
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

pub fn compute<D: digest::Digest>(salted_prefix: &String, number: &String) -> Hash {
    let mut digest = D::new();
    digest.input(salted_prefix.as_bytes());
    digest.input(number.as_bytes());
    let result = digest.result();
    Hash {
        lo: unsafe {
            std::mem::transmute::<[u8; 16], u128>({
                // let mut value = [0u8; 16];
                // let mut value = std::mem::MaybeUninit::<[u8; 16]>::uninit();
                let mut value = std::mem::uninitialized::<[u8; 16]>();
                value.copy_from_slice(&result[00..16]);
                value
            })
        },
        hi: unsafe {
            std::mem::transmute::<[u8; 16], u128>({
                let mut value = std::mem::uninitialized::<[u8; 16]>();
                value.copy_from_slice(&result[16..32]);
                value
            })
        },
    }
}
