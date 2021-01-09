macro_rules! byte_size_of {
    ($size:literal) => {
        $size / 8
    };
}

macro_rules! hash {
    ($($name:ident: $size:literal from $algorithm:ty),+) => {
        #[allow(non_camel_case_types)]
        #[derive(Copy, Clone, Eq, PartialEq, Debug)]
        pub enum Algorithm {
            $($name),*
        }

        impl Algorithm {
            pub fn variants() -> &'static [&'static str] {
                &[$(stringify!($name)),*]
            }
        }

        impl std::fmt::Display for Algorithm {
            fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$name => stringify!($name).fmt(fmt),)*
                }
            }
        }

        $(pub mod $name {
            #[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
            pub struct Hash([u8; byte_size_of!($size)]);

            unsafe impl ocl::OclPrm for Hash {}
            impl $crate::Input for Hash {}

            impl $crate::hash::Hash for Hash {
                fn digest(salted_prefix: &str, number: &str) -> Self {
                    use digest::Digest;
                    let mut digest = <$algorithm>::new();
                    digest.update(salted_prefix.as_bytes());
                    digest.update(number.as_bytes());
                    let result = digest.finalize();
                    Self::from_array(result)
                }

                fn from_array<N: digest::generic_array::ArrayLength<u8>>(
                    bytes: digest::generic_array::GenericArray<u8, N>,
                ) -> Self {
                    let mut data = unsafe {
                        std::mem::MaybeUninit::<[u8; byte_size_of!($size)]>::uninit().assume_init()
                    };
                    for i in 0..byte_size_of!($size) {
                        data[i] = bytes[i];
                    }
                    Self(data)
                }

                fn from_str(string: &str) -> Result<Self, $crate::error::Error> {
                    if string.len() != $size >> 2 {
                        bail!("String does not fit into hash: '{}'", &string);
                    }

                    let mut hash = Self::default();
                    for (i, c) in string.chars().enumerate() {
                        let int = match c as u8 {
                            c if (0x30..0x3a).contains(&c) => c - 0x30, // decimal
                            c if (0x41..0x47).contains(&c) => c - 0x41 + 0xa, // uppercase
                            c if (0x61..0x67).contains(&c) => c - 0x61 + 0xa, // lowercase
                            c => {
                                bail!("Failed to build hash: invalid character {}", c as char);
                            }
                        };
                        if i & 1 == 0 {
                            hash.0[i / 2] |= int << 4;
                        } else {
                            hash.0[i / 2] |= int
                        }
                    }
                    Ok(hash)
                }

                fn regex() -> &'static regex::Regex {
                    use lazy_static::lazy_static;
                    lazy_static! {
                        static ref RE: regex::Regex = regex::Regex::new(&format!("\\b[0-9a-fA-F]{{{}}}\\b", $size / 4))
                            .expect(stringify!(Could not build regex for $name));
                    }
                    &RE
                }

                fn name() -> &'static str {
                    stringify!($name)
                }
            }

            impl Default for Hash {
                fn default() -> Self {
                    Self([0; byte_size_of!($size)])
                }
            }

            impl std::cmp::Ord for Hash {
                fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                    for i in (0..byte_size_of!($size)).rev() {
                        match self.0[i].cmp(&other.0[i]) {
                            std::cmp::Ordering::Equal => (),
                            o => return o,
                        }
                    }
                    std::cmp::Ordering::Equal
                }
            }

            impl std::cmp::PartialOrd for Hash {
                fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                    for i in (0..byte_size_of!($size)).rev() {
                        match self.0[i].cmp(&other.0[i]) {
                            std::cmp::Ordering::Equal => (),
                            o => return Some(o),
                        }
                    }
                    Some(std::cmp::Ordering::Equal)
                }
            }

            impl std::fmt::LowerHex for Hash {
                fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    for b in self.0.iter() {
                        write!(fmt, "{:02x}", &b)?;
                    }
                    Ok(())
                }
            }

            impl std::fmt::Binary for Hash {
                fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    for b in self.0.iter() {
                        write!(fmt, "{:08b}", &b)?;
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
            impl std::convert::From<&str> for Hash {
                fn from(string: &str) -> Self {
                    use $crate::hash::Hash;
                    match Self::from_str(string) {
                        Ok(hash) => hash,
                        Err(e) => { panic!("{}", e); },
                    }
                }
            }

            #[cfg(test)]
            mod test {
                #[test]
                fn cmp() {
                    let mut hash_01 = super::Hash::default();
                    hash_01.0[1] = 1;
                    let mut hash_02 = super::Hash::default();
                    hash_02.0[1] = 2;
                    let mut hash_10 = super::Hash::default();
                    hash_10.0[0] = 1;
                    let mut hash_20 = super::Hash::default();
                    hash_20.0[0] = 2;

                    assert_eq!(hash_01.cmp(&hash_01), std::cmp::Ordering::Equal);
                    assert_eq!(hash_01.cmp(&hash_02), std::cmp::Ordering::Less);
                    assert_eq!(hash_01.cmp(&hash_20), std::cmp::Ordering::Greater);
                    assert_eq!(hash_01.cmp(&hash_10), std::cmp::Ordering::Greater);
                    assert_eq!(hash_10.cmp(&hash_20), std::cmp::Ordering::Less);
                }

                #[test]
                fn to_string() {
                    use rand::Rng;
                    let mut random = rand::thread_rng();
                    let mut string = String::new();
                    let mut hash = super::Hash::default();

                    for i in 0..byte_size_of!($size) {
                        let value: u8 = random.gen();
                        hash.0[i] = value;
                        string.push_str(format!("{:02x}", value).as_str());
                    }

                    assert_eq!(format!("{:x}", hash), string);
                }

                #[test]
                fn from_string() {
                    use rand::Rng;
                    let mut random = rand::thread_rng();
                    let mut string = String::new();
                    let mut expected = super::Hash::default();

                    for i in 0..byte_size_of!($size) {
                        let value: u8 = random.gen();
                        expected.0[i] = value;
                        string.push_str(format!("{:02x}", value).as_str());
                    }

                    let hash = super::Hash::from(string.as_str());
                    assert_eq!(expected, hash);
                }

                #[test]
                fn string_round_trip() {
                    use rand::Rng;
                    let mut random = rand::thread_rng();
                    let mut string = String::new();

                    for _ in 0..byte_size_of!($size) {
                        let value: u8 = random.gen();
                        string = format!("{}{:02x}", string, value);
                    }

                    let hash = super::Hash::from(string.as_str());
                    assert_eq!(format!("{:x}", hash), string);
                }

                #[test]
                fn digestion() {
                    use $crate::hash::Hash;
                    use digest::Digest;
                    let hash = super::Hash::digest("123", "abc");

                    let mut expected_hash = <$algorithm>::new();
                    expected_hash.update("123".as_bytes());
                    expected_hash.update("abc".as_bytes());

                    assert_eq!(
                        format!("{:x}", hash),
                        format!("{:x}", expected_hash.finalize())
                    );
                }

                #[test]
                fn regex() {
                    use $crate::hash::Hash;
                    let hash = super::Hash::digest("123", "abc");
                    let regex = super::Hash::regex();

                    assert!(regex.is_match(&hash.to_string()));
                    assert!(regex.is_match(&format!(" {} ", hash)));
                    assert!(regex.is_match(&format!("{{{},", hash)));
                    assert!(!regex.is_match(&format!("{}a", hash)));
                    assert!(!regex.is_match(&format!("a{}", hash)));
                    assert!(!regex.is_match("a"));
                }
            }
        })*
    };
}

pub trait Hash: ocl::OclPrm + std::fmt::LowerHex + std::fmt::Binary + crate::Input {
    fn digest(salted_prefix: &str, number: &str) -> Self;
    fn from_array<N: digest::generic_array::ArrayLength<u8>>(
        bytes: digest::generic_array::GenericArray<u8, N>,
    ) -> Self;
    fn from_str(string: &str) -> Result<Self, crate::error::Error>;
    fn regex() -> &'static regex::Regex;
    fn name() -> &'static str;
}

hash!(md5: 128 from md5::Md5, sha256: 256 from sha2::Sha256);
