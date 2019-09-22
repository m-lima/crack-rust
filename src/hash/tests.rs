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
