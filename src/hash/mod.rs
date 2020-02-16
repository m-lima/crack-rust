mod h128;
mod h256;

pub trait Hash:
    std::fmt::LowerHex
    + std::fmt::Binary
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + std::ops::ShlAssign<u8>
    + std::ops::BitOrAssign<u8>
    + Default
{
    fn to_string(&self) -> String {
        format!("{:x}", self)
    }

    fn from_array<N: digest::generic_array::ArrayLength<u8>>(
        bytes: digest::generic_array::GenericArray<u8, N>,
    ) -> Self;
}

pub trait AlgorithmConverter<D: digest::Digest> {
    type Hash: Hash + 'static;
    fn digest(salted_prefix: &str, number: &str) -> Self::Hash {
        let mut digest = D::new();
        digest.input(salted_prefix.as_bytes());
        digest.input(number.as_bytes());
        let result = digest.result();
        Self::Hash::from_array(result)
    }

    fn from_string(string: &str) -> Self::Hash {
        let mut hash = Self::Hash::default();
        for (i, c) in string.chars().rev().enumerate() {
            let int = match c as u8 {
                c if c >= 0x30 && c < 0x3a => c - 0x30,       // decimal
                c if c >= 0x41 && c < 0x47 => c - 0x41 + 0xa, // uppercase
                c if c >= 0x61 && c < 0x67 => c - 0x61 + 0xa, // lowercase
                c => {
                    eprintln!("Failed to build hash: invalid character {}", c as char);
                    std::process::exit(-1);
                }
            };
            if i % 2 == 0 {
                hash <<= 8;
                hash |= int;
            } else {
                hash |= int << 4;
            }
        }
        hash
    }
}

pub struct Converter<D: digest::Digest> {
    phantom: std::marker::PhantomData<D>,
}

impl AlgorithmConverter<md5::Md5> for Converter<md5::Md5> {
    type Hash = h128::Hash;
}

impl AlgorithmConverter<sha2::Sha256> for Converter<sha2::Sha256> {
    type Hash = h256::Hash;
}
