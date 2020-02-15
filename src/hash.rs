#[derive(Debug)]
pub enum Error {
    ParseError,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Copy, Clone)]
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn shift() {
        let mut hash = Hash { lo: 1, hi: 8 };
        hash = hash << 4;
        assert_eq!(hash, Hash { hi: 8 * 16, lo: 16 });
    }

    #[test]
    fn shift_overflow() {
        let mut hash = Hash {
            lo: (1 << 127) | 1,
            hi: 8,
        };
        hash = hash << 4;
        assert_eq!(
            hash,
            Hash {
                hi: 128 + 8,
                lo: 16
            }
        );
    }

    #[test]
    fn parse_string() {
        let input =
            String::from("dd130a849d7b29e5541b05d2f7f86a4acd4f1ec598c1c9438783f56bc4f0ff80");
        let parsed = input.into_hash().unwrap();

        let expected = Hash {
            hi: 0x80fff0c46bf5838743c9c198c51e4fcd,
            lo: 0x4a6af8f7d2051b54e5297b9d840a13dd,
        };

        assert_eq!(parsed, expected);
    }

    #[test]
    fn to_string() {
        let hash = Hash {
            hi: 0x80fff0c46bf5838743c9c198c51e4fcd,
            lo: 0x4a6af8f7d2051b54e5297b9d840a13dd,
        };

        assert_eq!(
            format!("{:x}", hash),
            "dd130a849d7b29e5541b05d2f7f86a4acd4f1ec598c1c9438783f56bc4f0ff80"
        );
    }

    #[test]
    fn string_round_trip() {
        let string =
            String::from("dd130a849d7b29e5541b05d2f7f86a4acd4f1ec598c1c9438783f56bc4f0ff80");
        let hash = string.into_hash().unwrap();
        assert_eq!(format!("{:x}", hash), string);
    }

    #[test]
    fn transmute() {
        let value = 1048576u128;
        let array = unsafe { std::mem::transmute::<_, [u8; 16]>(value) };
        for c in 0..16 {
            print!("{:02x}", array[16 - 1 - c]);
        }
        println!("\n{:032x}", value);
    }

    #[test]
    fn compute() {
        //dd130a849d7b29e5541b05d2f7f86a4acd4f1ec598c1c9438783f56bc4f0ff80
        let hash = super::compute::<sha2::Sha256>(&String::from("123"), &String::from("abc"));

        use sha2::Digest;
        let mut expected_hash = sha2::Sha256::new();
        expected_hash.input("123".as_bytes());
        expected_hash.input("abc".as_bytes());

        assert_eq!(
            format!("{:x}", hash),
            format!("{:x}", expected_hash.result())
        );
    }

    #[test]
    fn cmp() {
        let hash_10 = super::Hash { hi: 1, lo: 0 };
        let hash_20 = super::Hash { hi: 2, lo: 0 };
        let hash_01 = super::Hash { hi: 0, lo: 1 };
        let hash_02 = super::Hash { hi: 0, lo: 2 };

        assert_eq!(hash_10.cmp(&hash_10), std::cmp::Ordering::Equal);
        assert_eq!(hash_10.cmp(&hash_20), std::cmp::Ordering::Less);
        assert_eq!(hash_10.cmp(&hash_02), std::cmp::Ordering::Greater);
        assert_eq!(hash_10.cmp(&hash_01), std::cmp::Ordering::Greater);
        assert_eq!(hash_01.cmp(&hash_02), std::cmp::Ordering::Less);
    }
}
