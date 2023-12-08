use fast32::{INVALID_BYTE, INVALID_CHAR};

mod base32 {
    use fast32::base32::{decoder_map, decoder_map_simple, Alphabet};

    // only pub for the test below
    pub const ENC_CROCKFORD_LOWER: &'static [u8; 32] = b"0123456789abcdefghjkmnpqrstvwxyz";
    pub const DEC_CROCKFORD_LOWER: [u8; 256] = decoder_map(ENC_CROCKFORD_LOWER,
        b"               0123456789       abcdefgh1jk1mn0pqrst vwxyz      abcdefgh1jk1mn0pqrst vwxyz    "
    );
    pub const CROCKFORD_LOWER: Alphabet = Alphabet::new(ENC_CROCKFORD_LOWER, &DEC_CROCKFORD_LOWER, None);

    const ENC_RFC4648_LOWER: &'static [u8; 32] = b"abcdefghijklmnopqrstuvwxyz234567";
    const DEC_RFC4648_LOWER: [u8; 256] = decoder_map_simple(ENC_RFC4648_LOWER);
    pub const RFC4648_LOWER_NOPAD: Alphabet = Alphabet::new(ENC_RFC4648_LOWER, &DEC_RFC4648_LOWER, None);
}

mod base64 {
    use fast32::base64::{decoder_map, decoder_map_simple, Alphabet};

    const ENC_BASE64_WEIRD: &'static [u8; 64] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_.";
    const DEC_BASE64_WEIRD: [u8; 256] = decoder_map_simple(ENC_BASE64_WEIRD);
    pub const BASE64_WEIRD: Alphabet = Alphabet::new(ENC_BASE64_WEIRD, &DEC_BASE64_WEIRD, None);

    // only pub for the test below
    pub const ENC_BASE64_WEIRD_FLEX: &'static [u8; 64] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_.";
    pub const DEC_BASE64_WEIRD_FLEX: [u8; 256] = decoder_map(
        ENC_BASE64_WEIRD_FLEX,
        b"_    A    _._..0123456789     v ABCDEFGHIJKLMNOPQRSTUVWXYZ    _ abcdefghijklmnopqrstuvwxyz 2  ",
    );
    pub const BASE64_WEIRD_FLEX: Alphabet = Alphabet::new(ENC_BASE64_WEIRD_FLEX, &DEC_BASE64_WEIRD_FLEX, None);
}

use self::base32::{CROCKFORD_LOWER, DEC_CROCKFORD_LOWER, ENC_CROCKFORD_LOWER, RFC4648_LOWER_NOPAD};
use self::base64::{BASE64_WEIRD, BASE64_WEIRD_FLEX, DEC_BASE64_WEIRD_FLEX, ENC_BASE64_WEIRD_FLEX};

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
                ENC_BASE64_WEIRD_FLEX[b as usize] as char
            }
        })
        .iter()
        .collect::<String>();

    assert_eq!(dec_chars, s);
}
