#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Copy, Clone)]
pub struct Hash {
    lo: u128,
}

impl super::Builder for Hash {
    fn from_array<N: digest::generic_array::ArrayLength<u8>>(
        bytes: digest::generic_array::GenericArray<u8, N>,
    ) -> Self {
        use std::convert::TryInto;
        Self {
            lo: unsafe {
                std::mem::transmute::<[u8; 16], u128>(
                    bytes[00..16].try_into().expect("Failed lo transmutation"),
                )
            },
        }
    }
}

impl super::Hash for Hash {}

impl Default for Hash {
    fn default() -> Self {
        Self { lo: 0 }
    }
}

impl std::ops::ShlAssign<u8> for Hash {
    fn shl_assign(&mut self, rhs: u8) {
        self.lo <<= rhs;
    }
}

impl std::ops::BitOrAssign<u8> for Hash {
    // Allowed because of hot path and certainty of no loss
    #[allow(clippy::cast_lossless)]
    fn bitor_assign(&mut self, rhs: u8) {
        self.lo |= rhs as u128;
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
        Ok(())
    }
}

impl std::fmt::Display for Hash {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self, fmt)
    }
}

#[cfg(test)]
mod test {
    use super::super::AlgorithmConverter;
    use super::super::Converter;
    use super::Hash;

    #[test]
    fn shift() {
        let mut hash = Hash { lo: 1 };
        hash <<= 4;
        assert_eq!(hash, Hash { lo: 16 });
    }

    #[test]
    fn shift_overflow() {
        let mut hash = Hash { lo: (1 << 127) | 1 };
        hash <<= 4;
        assert_eq!(hash, Hash { lo: 16 });
    }

    #[test]
    fn parse_string() {
        let input =
            String::from("dd130a849d7b29e5541b05d2f7f86a4acd4f1ec598c1c9438783f56bc4f0ff80");
        let parsed = Converter::<md5::Md5>::from_string(&input);

        let expected = Hash {
            lo: 0x4a6af8f7d2051b54e5297b9d840a13dd,
        };

        assert_eq!(parsed, expected);
    }

    #[test]
    fn to_string() {
        let hash = Hash {
            lo: 0x4a6af8f7d2051b54e5297b9d840a13dd,
        };

        assert_eq!(format!("{:x}", hash), "dd130a849d7b29e5541b05d2f7f86a4a");
    }

    #[test]
    fn string_round_trip() {
        let string = String::from("dd130a849d7b29e5541b05d2f7f86a4a");
        let hash = Converter::<md5::Md5>::from_string(&string);
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
        //a906449d5769fa7361d7ecc6aa3f6d28
        let hash = Converter::<md5::Md5>::digest(&String::from("123"), &String::from("abc"));

        use sha2::Digest;
        let mut expected_hash = md5::Md5::new();
        expected_hash.input("123".as_bytes());
        expected_hash.input("abc".as_bytes());

        assert_eq!(
            format!("{:x}", hash),
            format!("{:x}", expected_hash.result())
        );
    }

    #[test]
    fn cmp() {
        let hash_0 = Hash { lo: 0 };
        let hash_0b = Hash { lo: 0 };
        let hash_1 = Hash { lo: 1 };
        let hash_2 = Hash { lo: 2 };

        assert_eq!(hash_0.cmp(&hash_0b), std::cmp::Ordering::Equal);
        assert_eq!(hash_1.cmp(&hash_2), std::cmp::Ordering::Less);
        assert_eq!(hash_1.cmp(&hash_0), std::cmp::Ordering::Greater);
    }
}
