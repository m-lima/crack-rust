#[derive(Debug)]
enum HashError {
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

pub fn compute<D: digest::Digest>(salted_prefix: &String, number: &String) -> Hash {
    let mut digest = D::new();
    digest.input(salted_prefix);
    digest.input(number);
    digest
        .result()
        .into_iter()
        .fold(Hash::default(), |p, c| (p << 1) + c as u64)
}

impl Ord for Hash {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        match self.hi.cmp(&rhs.lo) {
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => self.lo.cmp(&rhs.lo),
        }
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
        write!(fmt, "{:032x}{:032x}", self.hi, self.lo)
    }
}

impl std::fmt::Binary for Hash {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{:0128b}{:0128b}", self.hi, self.lo)
    }
}

trait IntoHash {
    fn into_number(&self) -> Result<Hash, HashError>;
}

impl IntoHash for String {
  fn into_number(&self) -> Result<Hash, HashError> {
      let mut hash = Hash::default();
      for c in self.chars() {
          let int = match c as u8 {
              c if c >= 0x30 && c < 0x3a  => c - 0x30, // decimal
              c if c >= 0x41 && c < 0x47  => c - 0x41 + 0xa, // uppercase
              c if c >= 0x61 && c < 0x67  => c - 0x61 + 0xa, // lowercase
              _ => return Err(HashError::ParseError),
          };
          hash = (hash << 4) | int;
      }
      Ok(hash)
  }
}

#[test]
fn add() {
    let mut hash = Hash{
        lo: std::u128::MAX - 1,
        hi: 16,
    };
    hash = hash + 1u64;
    assert_eq!(hash, Hash{ hi: 16, lo: std::u128::MAX });
}

#[test]
fn add_overflow() {
    let mut hash = Hash{
        lo: std::u128::MAX,
        hi: 16,
    };
    hash = hash + 1u8;
    assert_eq!(hash, Hash{ hi: 17, lo: 0 });
}

#[test]
fn shift() {
    let mut hash = Hash{
        lo: 1,
        hi: 8,
    };
    hash = hash << 4;
    assert_eq!(hash, Hash{ hi: 8 * 16, lo: 16 });
}

#[test]
fn shift_overflow() {
    let mut hash = Hash{
        lo: (1 << 127) | 1,
        hi: 8,
    };
    hash = hash << 4;
    assert_eq!(hash, Hash{ hi: 128 + 8, lo: 16 });
}

#[test]
fn parse_string() {
    let input = String::from("123Af");
    let parsed = input.into_number().unwrap();
    assert_eq!(parsed, Hash{ hi: 0, lo: 74671 });
}
