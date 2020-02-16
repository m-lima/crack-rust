// TODO: make them private (base only on Digest)
pub mod h128;
pub mod h256;

trait HashInner {}

pub trait Hash:
    std::fmt::LowerHex
    + std::fmt::Binary
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + std::hash::Hash
    + Copy
    + Clone
    + Default
    + std::ops::ShlAssign<u8>
    + std::ops::BitOrAssign<u8>
{
    type Algorithm: digest::Digest;

    fn from<N: digest::generic_array::ArrayLength<u8>>(
        bytes: digest::generic_array::GenericArray<u8, N>,
    ) -> Self;

    fn to_string(&self) -> String {
        format!("{:x}", self)
    }
}

pub fn compute<H: Hash>(salted_prefix: &str, number: &str) -> H {
    use digest::Digest;
    let mut digest = H::Algorithm::new();
    digest.input(salted_prefix.as_bytes());
    digest.input(number.as_bytes());
    let result = digest.result();
    H::from(result)
}

pub fn hash<H: Hash>(string: &str) -> H {
    let mut hash = H::default();
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
