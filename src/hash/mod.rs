macro_rules! convert {
    ($($algorithm:ty => $hash:ty as $name:ident),+) => {
        $(pub struct $name;

        impl Converter<$algorithm> for $name {
            type Output = $hash;
            fn digest(salted_prefix: &str, number: &str) -> Self::Output {
                use digest::Digest;
                let mut digest = <$algorithm>::new();
                digest.input(salted_prefix.as_bytes());
                digest.input(number.as_bytes());
                let result = digest.result();
                <$hash>::from_array(result)
            }

            fn from_string(string: &str) -> Self::Output {
                <$hash>::from(string)
            }
        })*

//        #[cfg(test)]
//        mod test {
//        $(
//            #[test]
//            fn string_round_trip() {
//                let string = String::from(
//                    "dd130a849d7b29e5541b05d2f7f86a4acd4f1ec598c1c9438783f56bc4f0ff80",
//                );
//                let hash = Sha256::from_string(&string);
//                assert_eq!(format!("{:x}", hash), string);
//            }
//
//            #[test]
//            fn compute() {
//                //dd130a849d7b29e5541b05d2f7f86a4acd4f1ec598c1c9438783f56bc4f0ff80
//                let hash = Sha256::digest(&String::from("123"), &String::from("abc"));
//
//                use sha2::Digest;
//                let mut expected_hash = sha2::Sha256::new();
//                expected_hash.input("123".as_bytes());
//                expected_hash.input("abc".as_bytes());
//
//                assert_eq!(
//                    format!("{:x}", hash),
//                    format!("{:x}", expected_hash.result())
//                );
//            }
//        )*
    };
}

macro_rules! size_of {
    ($base:ty) => {
        std::mem::size_of::<$base>()
    };
}

macro_rules! create_gpu_array {
    ($name:ident[u128; $size:literal]) => {
        impl super::GpuCompatible for Hash {
            type GpuArray = GpuHash;
            fn to_gpu_array(&self) -> Self::GpuArray {
                unsafe {
                    let mut blocks = MaybeUninit::<[u64; $size * 2]>::uninit().assume_init();
                    for i in 0..$size {
                        let block = transmute::<u128, [u64; 2]>(self.0[i * size_of!($base)]);
                        blocks[i * 2] = blocks[0];
                        blocks[(i + 1) * 2] = blocks[1];
                    }
                    GpuHash(blocks)
                }
            }
        }

        #[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Default, Clone, Copy)]
        pub struct GpuHash([u64; $size * size_of!($base) / size_of!(u64)]);
        unsafe impl ocl::OclPrm for GpuHash {}

        impl std::fmt::Display for GpuHash {
            fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                for block in &self.0 {
                    let bytes = &unsafe { transmute::<u64, [u8; size_of!(u64)]>(*block) };
                    for b in bytes {
                        write!(fmt, "{:02x}", b)?;
                    }
                }
                Ok(())
            }
        }
    };

    ($name:ident[$base:ty; $size:literal]) => {
        impl super::GpuCompatible for Hash {
            type GpuArray = GpuHash;
            fn to_gpu_array(&self) -> Self::GpuArray {
                unsafe {
                    let mut blocks = MaybeUninit::<[$base; $size]>::uninit().assume_init();
                    blocks.copy_from_slice(&self.0);
                    GpuHash(blocks)
                }
            }
        }

        #[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Default, Clone, Copy)]
        pub struct GpuHash([$base; $size]);
        unsafe impl ocl::OclPrm for GpuHash {}

        impl std::fmt::Display for GpuHash {
            fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                for block in &self.0 {
                    let bytes = &unsafe { transmute::<$base, [u8; size_of!($base)]>(*block) };
                    for b in bytes {
                        write!(fmt, "{:02x}", b)?;
                    }
                }
                Ok(())
            }
        }
    };
}

macro_rules! create_tests {
    ($name:ident[$base:ty; $size:literal]) => {
        #[cfg(test)]
        mod test {
            #[test]
            fn shift() {
                let mut hash = super::Hash::default();
                for i in 0..$size {
                    hash.0[i] = i as $base;
                }
                hash <<= 2;

                let mut expected = super::Hash::default();
                for i in 0..$size {
                    expected.0[i] = i as $base * 4;
                }

                assert_eq!(hash, expected);
            }

            #[test]
            fn shift_overflow() {
                let mut hash = super::Hash::default();
                for i in 0..$size {
                    // Populating with 0b100001
                    hash.0[i] = (1 << (size_of!($base) * 8 - 1)) | 1;
                }
                hash <<= 1;

                let mut expected = super::Hash::default();
                for i in 0..($size - 1) {
                    expected.0[i] = 3;
                }
                expected.0[$size - 1] = 2;

                assert_eq!(hash, expected);
            }

            #[test]
            fn cmp() {
                let mut hash_10 = super::Hash::default();
                hash_10.0[0] = 1;
                let mut hash_20 = super::Hash::default();
                hash_20.0[0] = 2;

                assert_eq!(hash_10.cmp(&hash_10), std::cmp::Ordering::Equal);
                assert_eq!(hash_10.cmp(&hash_20), std::cmp::Ordering::Less);

                if $size > 1 {
                    let mut hash_01 = super::Hash::default();
                    hash_01.0[$size - 1] = 1;
                    let mut hash_02 = super::Hash::default();
                    hash_02.0[$size - 1] = 2;

                    assert_eq!(hash_10.cmp(&hash_02), std::cmp::Ordering::Greater);
                    assert_eq!(hash_10.cmp(&hash_01), std::cmp::Ordering::Greater);
                    assert_eq!(hash_01.cmp(&hash_02), std::cmp::Ordering::Less);
                }
            }

            #[test]
            fn string_round_trip() {
//                let mut random = rand::thread_rng();
//                let mut string = String::new();
//
//                for _ in 0..$size {
//                    let value: $base = random.gen();
//                    string = format!("{}")
//                }

                let string = String::from(&"dd130a849d7b29e5541b05d2f7f86a4acd4f1ec598c1c9438783f56bc4f0ff80dd130a849d7b29e5541b05d2f7f86a4acd4f1ec598c1c9438783f56bc4f0ff80"[0..size_of!($base)*$size*2]);
                let hash = super::Hash::from(&"dd130a849d7b29e5541b05d2f7f86a4acd4f1ec598c1c9438783f56bc4f0ff80dd130a849d7b29e5541b05d2f7f86a4acd4f1ec598c1c9438783f56bc4f0ff80"[0..size_of!($base)*$size*2]);
                println!("{:x} :: {}", hash, string);
                assert_eq!(format!("{:x}", hash), string);
            }

            //            #[test]
            //            fn parse_string() {
            //                let mut random = rand::thread_rng();
            //
            //                let value: $base = random.gen();
            //                let input = String::from(
            //                    "dd130a849d7b29e5541b05d2f7f86a4acd4f1ec598c1c9438783f56bc4f0ff80",
            //                );
            //                let parsed = Sha256::from_string(&input);
            //
            //                let expected = super::Hash {
            //                    hi: 0x80fff0c46bf5838743c9c198c51e4fcd,
            //                    lo: 0x4a6af8f7d2051b54e5297b9d840a13dd,
            //                };
            //
            //                assert_eq!(parsed, expected);
            //            }
            //
            //            #[test]
            //            fn to_string() {
            //                let hash = super::Hash {
            //                    hi: 0x80fff0c46bf5838743c9c198c51e4fcd,
            //                    lo: 0x4a6af8f7d2051b54e5297b9d840a13dd,
            //                };
            //
            //                assert_eq!(
            //                    format!("{:x}", hash),
            //                    "dd130a849d7b29e5541b05d2f7f86a4acd4f1ec598c1c9438783f56bc4f0ff80"
            //                );
            //            }

            //
            //            #[test]
            //            fn round_trip_gpu() {
            //                let expected = super::Hash {
            //                    hi: 0x80fff0c46bf5838743c9c198c51e4fcd,
            //                    lo: 0x4a6af8f7d2051b54e5297b9d840a13dd,
            //                };
            //                {
            //                    use super::super::GpuCompatible;
            //                    let gpu_array = expected.to_gpu_array();
            //
            //                    assert_eq!(gpu_array.to_string(), expected.to_string());
            //                }
            //            }
        }
    };
}

macro_rules! create_hash {
    ($name:ident[$base:ty; $size:literal]) => {
        mod $name {
            use std::mem::{transmute, MaybeUninit};

            #[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
            pub struct Hash([$base; $size]);

            // Allowed because clippy cannot interpret this macro
            #[allow(clippy::derive_hash_xor_eq)]
            impl $crate::hash::Hash for Hash {
                fn from_array<N: digest::generic_array::ArrayLength<u8>>(
                    bytes: digest::generic_array::GenericArray<u8, N>,
                ) -> Self {
                    unsafe {
                        use std::convert::TryInto;
                        let mut blocks = MaybeUninit::<[$base; $size]>::uninit().assume_init();
                        for i in 0..$size {
                            blocks[i] = transmute::<[u8; size_of!($base)], $base>(
                                bytes[i * size_of!($base)..(i + 1) * size_of!($base)]
                                    .try_into()
                                    .expect("Failed transmutation"),
                            );
                        }
                        Self(blocks)
                    }
                }
            }

            impl Default for Hash {
                fn default() -> Self {
                    Self([0; $size])
                }
            }

            // Allowed because clippy cannot interpret this macro
            #[allow(clippy::suspicious_op_assign_impl)]
            impl std::ops::ShlAssign<u8> for Hash {
                fn shl_assign(&mut self, rhs: u8) {
                    for i in 0..($size - 1) {
                        self.0[i] <<= rhs;
                        self.0[i] |= self.0[i + 1] >> ((size_of!($base) * 8) as u8 - rhs);
                    }
                    self.0[$size - 1] <<= rhs;
                }
            }

            // Allowed because clippy cannot interpret this macro
            #[allow(clippy::suspicious_op_assign_impl)]
            impl std::ops::BitOrAssign<u8> for Hash {
                // Allowed because of hot path and certainty of no loss
                #[allow(clippy::cast_lossless)]
                fn bitor_assign(&mut self, rhs: u8) {
                    self.0[$size - 1] |= rhs as u128;
                }
            }

            impl std::fmt::LowerHex for Hash {
                fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    for block in &self.0 {
                        let bytes = &unsafe { transmute::<$base, [u8; size_of!($base)]>(*block) };
                        for b in bytes {
                            write!(fmt, "{:02x}", b)?;
                        }
                    }
                    Ok(())
                }
            }

            impl std::fmt::Binary for Hash {
                fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    for block in &self.0 {
                        let bytes = &unsafe { transmute::<$base, [u8; size_of!($base)]>(*block) };
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

            impl std::convert::From<&str> for Hash {
                fn from(string: &str) -> Self {
                    if (string.len() >> 1)  != size_of!($base) * $size {
                        eprintln!("String does not fit into hash: {}", &string);
                        std::process::exit(-1);
                    }

                    let mut hash = Self::default();
                    for (i, c) in string.chars().rev().enumerate() {
                        let int = match c as u8 {
                            c if c >= 0x30 && c < 0x3a => c - 0x30, // decimal
                            c if c >= 0x41 && c < 0x47 => c - 0x41 + 0xa, // uppercase
                            c if c >= 0x61 && c < 0x67 => c - 0x61 + 0xa, // lowercase
                            c => {
                                eprintln!("Failed to build hash: invalid character {}", c as char);
                                std::process::exit(-1);
                            }
                        };
                        if i & 1 == 0 {
                            hash <<= 8;
                            hash |= int;
                        } else {
                            hash |= int << 4;
                        }
                    }
                    hash
                }
            }

            create_gpu_array!($name[$base; $size]);
            create_tests!($name[$base; $size]);
        }
    };
}

create_hash!(h128[u128; 1]);
create_hash!(h256[u128; 2]);

pub trait GpuCompatible {
    type GpuArray: ocl::OclPrm;
    fn to_gpu_array(&self) -> Self::GpuArray;
}

pub trait Hash:
    GpuCompatible + std::fmt::LowerHex + std::fmt::Binary + ToString + PartialEq + Eq + PartialOrd + Ord
{
    fn from_array<N: digest::generic_array::ArrayLength<u8>>(
        bytes: digest::generic_array::GenericArray<u8, N>,
    ) -> Self;
}

pub trait Converter<D: digest::Digest> {
    type Output: Hash + 'static;
    fn digest(salted_prefix: &str, number: &str) -> Self::Output;
    fn from_string(string: &str) -> Self::Output;
}

convert!(md5::Md5 => h128::Hash as Md5, sha2::Sha256 => h256::Hash as Sha256);
