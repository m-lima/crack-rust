// TODO: make them private (base only on Digest)
pub mod h128;
pub mod h256;

trait HashInner {}

trait InnerHash: std::ops::ShlAssign<u8> + std::ops::BitOrAssign<u8> + Default {
    fn from<N: digest::generic_array::ArrayLength<u8>>(
        bytes: digest::generic_array::GenericArray<u8, N>,
    ) -> Self;
}

pub trait Hash:
    std::fmt::LowerHex + std::fmt::Binary + PartialEq + Eq + PartialOrd + Ord + Sized
{
    fn to_string(&self) -> String {
        format!("{:x}", self)
    }
}

//pub trait AlgorithmConverter<D: digest::Digest, H: Hash + InnerHash> {
//    fn digest(salted_prefix: &str, number: &str) -> H {
//        let mut digest = D::new();
//        digest.input(salted_prefix.as_bytes());
//        digest.input(number.as_bytes());
//        let result = digest.result();
//        H::from(result)
//    }
//
//    fn from_string(string: &str) -> H {
//        let mut hash = H::default();
//        for (i, c) in string.chars().rev().enumerate() {
//            let int = match c as u8 {
//                c if c >= 0x30 && c < 0x3a => c - 0x30,       // decimal
//                c if c >= 0x41 && c < 0x47 => c - 0x41 + 0xa, // uppercase
//                c if c >= 0x61 && c < 0x67 => c - 0x61 + 0xa, // lowercase
//                c => {
//                    eprintln!("Failed to build hash: invalid character {}", c as char);
//                    std::process::exit(-1);
//                }
//            };
//            if i % 2 == 0 {
//                hash <<= 8;
//                hash |= int;
//            } else {
//                hash |= int << 4;
//            }
//        }
//        hash
//    }
//}
//
//pub struct Converter<D: digest::Digest> {
//    phantom: std::marker::PhantomData<D>,
//}
//
//impl AlgorithmConverter<md5::Md5, h128::Hash> for Converter<md5::Md5> {}
//impl AlgorithmConverter<sha2::Sha256, h256::Hash> for Converter<sha2::Sha256> {}
////impl<D: digest::Digest, H: Hash + InnerHash> AlgorithmConverter<D, H> for Converter<D> {}

//pub struct Converter<D: digest::Digest> {
//    phantom: std::marker::PhantomData<D>,
//}
//
//impl Converter<md5::Md5> {
//    pub fn digest(salted_prefix: &str, number: &str) -> h128::Hash {
//        digest(salted_prefix, number)
//    }
//
//    pub fn from_string(string: &str) -> h128::Hash {
//        from_string(string)
//    }
//}
//
//impl Converter<sha2::Sha256> {
//    pub fn digest(salted_prefix: &str, number: &str) -> h256::Hash {
//        digest(salted_prefix, number)
//    }
//
//    pub fn from_string(string: &str) -> h256::Hash {
//        from_string(string)
//    }
//}
//
//fn digest<H: Hash>(salted_prefix: &str, number: &str) -> H {
//    use digest::Digest;
//    let mut digest = H::Algorithm::new();
//    digest.input(salted_prefix.as_bytes());
//    digest.input(number.as_bytes());
//    let result = digest.result();
//    H::from(result)
//}
//
//fn from_string<H: Hash>(string: &str) -> H {
//    let mut hash = H::default();
//    for (i, c) in string.chars().rev().enumerate() {
//        let int = match c as u8 {
//            c if c >= 0x30 && c < 0x3a => c - 0x30,       // decimal
//            c if c >= 0x41 && c < 0x47 => c - 0x41 + 0xa, // uppercase
//            c if c >= 0x61 && c < 0x67 => c - 0x61 + 0xa, // lowercase
//            c => {
//                eprintln!("Failed to build hash: invalid character {}", c as char);
//                std::process::exit(-1);
//            }
//        };
//        if i % 2 == 0 {
//            hash <<= 8;
//            hash |= int;
//        } else {
//            hash |= int << 4;
//        }
//    }
//    hash
//}
