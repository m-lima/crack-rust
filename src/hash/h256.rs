#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Hash {
    hi: u128,
    lo: u128,
}

impl super::GpuCompatible for Hash {
    type GpuArray = GpuHash;
    fn to_gpu_array(&self) -> Self::GpuArray {
        GpuHash(unsafe {
            let lo = std::mem::transmute::<u128, [u64; 2]>(self.lo);
            let hi = std::mem::transmute::<u128, [u64; 2]>(self.hi);
            [lo[0], lo[1], hi[0], hi[1]]
        })
    }
}

impl super::Hash for Hash {
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
            hi: unsafe {
                std::mem::transmute::<[u8; 16], u128>(
                    bytes[16..32].try_into().expect("Failed lo transmutation"),
                )
            },
        }
    }
}

impl Default for Hash {
    fn default() -> Self {
        Self { hi: 0, lo: 0 }
    }
}

impl std::ops::ShlAssign<u8> for Hash {
    fn shl_assign(&mut self, rhs: u8) {
        self.hi <<= rhs;
        self.hi |= self.lo >> (128 - rhs);
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

impl std::fmt::Display for Hash {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self, fmt)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Default, Clone, Copy)]
pub struct GpuHash([u64; 4]);
unsafe impl ocl::OclPrm for GpuHash {}

impl std::fmt::Display for GpuHash {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for block in &self.0 {
            let bytes = &unsafe { std::mem::transmute::<u64, [u8; 8]>(*block) };
            for b in bytes {
                write!(fmt, "{:02x}", b)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::super::AlgorithmConverter;
    use super::super::Converter;
    use super::Hash;

    #[test]
    fn shift() {
        let mut hash = Hash { lo: 1, hi: 8 };
        hash <<= 4;
        assert_eq!(hash, Hash { hi: 8 * 16, lo: 16 });
    }

    #[test]
    fn shift_overflow() {
        let mut hash = Hash {
            lo: (1 << 127) | 1,
            hi: 8,
        };
        hash <<= 4;
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
        let parsed = Converter::<sha2::Sha256>::from_string(&input);

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
        let hash = Converter::<sha2::Sha256>::from_string(&string);
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
        let hash = Converter::<sha2::Sha256>::digest(&String::from("123"), &String::from("abc"));

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
        let hash_10 = Hash { hi: 1, lo: 0 };
        let hash_20 = Hash { hi: 2, lo: 0 };
        let hash_01 = Hash { hi: 0, lo: 1 };
        let hash_02 = Hash { hi: 0, lo: 2 };

        assert_eq!(hash_10.cmp(&hash_10), std::cmp::Ordering::Equal);
        assert_eq!(hash_10.cmp(&hash_20), std::cmp::Ordering::Less);
        assert_eq!(hash_10.cmp(&hash_02), std::cmp::Ordering::Greater);
        assert_eq!(hash_10.cmp(&hash_01), std::cmp::Ordering::Greater);
        assert_eq!(hash_01.cmp(&hash_02), std::cmp::Ordering::Less);
    }

    #[test]
    fn round_trip_gpu() {
        let expected = Hash {
            hi: 0x80fff0c46bf5838743c9c198c51e4fcd,
            lo: 0x4a6af8f7d2051b54e5297b9d840a13dd,
        };
        {
            use super::super::GpuCompatible;
            let gpu_array = expected.to_gpu_array();

            assert_eq!(gpu_array.to_string(), expected.to_string());
        }
    }
}
