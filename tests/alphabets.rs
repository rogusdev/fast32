use fast32::INVALID_BYTE;

mod base32 {
    use fast32::make_base32_alpha;

    // only pub for the test below
    pub const ENC_CROCKFORD_LOWER: &'static [u8; 32] = b"0123456789abcdefghjkmnpqrstvwxyz";
    // also see this defined in base32/alphabet.rs and available in the crate at fast32::base32::CROCKFORD_LOWER
    make_base32_alpha!(
        CROCKFORD_LOWER,
        DEC_CROCKFORD_LOWER,
        ENC_CROCKFORD_LOWER,
        b"iloABCDEFGHIJKLMNOPQRSTVWXYZ",
        b"110abcdefgh1jk1mn0pqrstvwxyz"
    );

    make_base32_alpha!(
        RFC4648_LOWER_NOPAD,
        DEC_RFC4648_LOWER,
        b"abcdefghijklmnopqrstuvwxyz234567"
    );
}

mod base64 {
    use fast32::make_base64_alpha;

    // only pub for the test below
    pub const ENC_BASE64_WEIRD: &'static [u8; 64] =
        b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_.";
    make_base64_alpha!(BASE64_WEIRD, DEC_BASE64_WEIRD, ENC_BASE64_WEIRD);

    make_base64_alpha!(
        BASE64_WEIRD_FLEX,
        DEC_BASE64_WEIRD_FLEX,
        ENC_BASE64_WEIRD,
        b"!&+,-/|?",
        b"_A_._.2v"
    );
}

use self::base32::{
    CROCKFORD_LOWER, DEC_CROCKFORD_LOWER, ENC_CROCKFORD_LOWER, RFC4648_LOWER_NOPAD,
};
use self::base64::{BASE64_WEIRD, BASE64_WEIRD_FLEX, DEC_BASE64_WEIRD_FLEX, ENC_BASE64_WEIRD};

#[test]
fn crockford_10238() {
    let n = 10238;
    let x = "9zy";
    let s = CROCKFORD_LOWER.encode_u64(n);
    assert_eq!(s, x);
    assert_eq!(CROCKFORD_LOWER.decode_u64(s.as_bytes()).unwrap(), n);
}

#[test]
fn crockford_1055_confirm() {
    let n = 1055;
    let x = "10z";
    let s = CROCKFORD_LOWER.encode_u64(n);
    assert_eq!(s, x);
    assert_eq!(CROCKFORD_LOWER.decode_u64(s.as_bytes()).unwrap(), n);
    let s = "LoZ";
    assert_eq!(CROCKFORD_LOWER.decode_u64(s.as_bytes()).unwrap(), n);
}

#[test]
fn rfc4648_32_10238() {
    let n = 10238;
    let x = "j76";
    let s = RFC4648_LOWER_NOPAD.encode_u64(n);
    assert_eq!(s, x);
    assert_eq!(RFC4648_LOWER_NOPAD.decode_u64(s.as_bytes()).unwrap(), n);
}

#[test]
fn base64_weird_64_10238() {
    let n = 10238;
    let x = "2v_";
    let s = BASE64_WEIRD.encode_u64(n);
    assert_eq!(s, x);
    assert_eq!(BASE64_WEIRD.decode_u64(s.as_bytes()).unwrap(), n);
}

#[test]
fn base64_weird_flex_confirm() {
    let n = 10238;
    let x = "2v_";
    let s = BASE64_WEIRD_FLEX.encode_u64(n);
    assert_eq!(s, x);
    assert_eq!(BASE64_WEIRD_FLEX.decode_u64(s.as_bytes()).unwrap(), n);
    let s = "|?!";
    assert_eq!(BASE64_WEIRD_FLEX.decode_u64(s.as_bytes()).unwrap(), n);
}

// adapted from src/base32/alphabet test
#[test]
fn confirm_crockford_lower_decode() {
    const INVALID_CHAR: char = ' ';
    let mut s = String::new();
    for i in 0..=255u8 {
        let c = i as char;
        #[rustfmt::skip]
        let c = match c {
            '0' | 'o' | 'O' => '0',
            '1' | 'i' | 'I' | 'l' | 'L' => '1',
            '2' ..= '9' => c,
            'u' | 'U' => INVALID_CHAR,
            'A' ..= 'Z' => c.to_ascii_lowercase(),
            'a' ..= 'z' => c,
            _ => INVALID_CHAR,
        };
        s.push(c);
    }

    let dec_chars = DEC_CROCKFORD_LOWER
        .map(|b| {
            if b == INVALID_BYTE {
                INVALID_CHAR
            } else {
                ENC_CROCKFORD_LOWER[b as usize] as char
            }
        })
        .iter()
        .collect::<String>();

    assert_eq!(dec_chars, s);
}

#[test]
fn confirm_b64_weird_flex_decode() {
    const INVALID_CHAR: char = ' ';
    let mut s = String::new();
    for i in 0..=255u8 {
        let c = i as char;
        #[rustfmt::skip]
        let c = match c {
            '!' | '+' | '-' | '_' => '_',
            ',' | '/' | '.' => '.',
            '&' => 'A',
            '|' => '2',
            '?' => 'v',
            '0' ..= '9' => c,
            'A' ..= 'Z' => c,
            'a' ..= 'z' => c,
            _ => INVALID_CHAR,
        };
        s.push(c);
    }

    let dec_chars = DEC_BASE64_WEIRD_FLEX
        .map(|b| {
            if b == INVALID_BYTE {
                INVALID_CHAR
            } else {
                ENC_BASE64_WEIRD[b as usize] as char
            }
        })
        .iter()
        .collect::<String>();

    assert_eq!(dec_chars, s);
}
