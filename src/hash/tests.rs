use super::*;

#[test]
fn add() {
    let mut hash = Hash {
        lo: u128::max_value() - 1,
        hi: 16,
    };
    hash = hash + 1u64;
    assert_eq!(
        hash,
        Hash {
            hi: 16,
            lo: u128::max_value(),
        }
    );
}

#[test]
fn add_overflow() {
    let mut hash = Hash {
        lo: u128::max_value(),
        hi: 16,
    };
    hash = hash + 1u8;
    assert_eq!(hash, Hash { hi: 17, lo: 0 });
}

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
    let input = String::from("dd130a849d7b29e5541b05d2f7f86a4acd4f1ec598c1c9438783f56bc4f0ff80");
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
    let string = String::from("dd130a849d7b29e5541b05d2f7f86a4acd4f1ec598c1c9438783f56bc4f0ff80");
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
