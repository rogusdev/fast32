mod decode_base;
mod decode_u64;
mod encode_u64;

pub use crate::decode_base::DecodeError;
pub use crate::decode_u64::decode_u64;
pub use crate::encode_u64::encode_u64;

const INVALID_CHAR: char = '.';
const INVALID_BYTE: u8 = u8::MAX;

pub const ENC_CROCKFORD_UPPER: &'static [u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decode_base::DEC_CROCKFORD_UPPER;
    use crate::ENC_CROCKFORD_UPPER;

    #[test]
    fn both_u64_0() {
        let n = 0;
        let x = "0";
        let s = encode_u64(n);
        assert_eq!(s, x);
        assert_eq!(decode_u64(s.as_bytes()).unwrap(), n);
    }

    #[test]
    fn both_u64_31() {
        let n = 31;
        let x = "Z";
        let s = encode_u64(n);
        assert_eq!(s, x);
        assert_eq!(decode_u64(s.as_bytes()).unwrap(), n);
    }

    #[test]
    fn both_u64_32() {
        let n = 32;
        let x = "10";
        let s = encode_u64(n);
        assert_eq!(s, x);
        assert_eq!(decode_u64(s.as_bytes()).unwrap(), n);
    }

    #[test]
    fn both_u64_33() {
        let n = 33;
        let x = "11";
        let s = encode_u64(n);
        assert_eq!(s, x);
        assert_eq!(decode_u64(s.as_bytes()).unwrap(), n);
    }

    #[test]
    fn both_u64_32_32() {
        let n = 32 * 32;
        let x = "100";
        let s = encode_u64(n);
        assert_eq!(s, x);
        assert_eq!(decode_u64(s.as_bytes()).unwrap(), n);
    }

    #[test]
    fn both_u64_32_32_32_32() {
        let n = 32 * 32 * 32 * 32;
        let x = "10000";
        let s = encode_u64(n);
        assert_eq!(s, x);
        assert_eq!(decode_u64(s.as_bytes()).unwrap(), n);
    }

    #[test]
    fn both_u64_32_32_32_32_1() {
        let n = (32 * 32 * 32 * 32) - 1;
        let x = "ZZZZ";
        let s = encode_u64(n);
        assert_eq!(s, x);
        assert_eq!(decode_u64(s.as_bytes()).unwrap(), n);
    }

    #[test]
    fn both_u64_10239() {
        let n = 10239;
        let x = "9ZZ";
        let s = encode_u64(n);
        assert_eq!(s, x);
        assert_eq!(decode_u64(s.as_bytes()).unwrap(), n);
    }

    #[test]
    fn decode_u64_bad() {
        let res = decode_u64(b"^_^");
        assert_eq!(res.unwrap_err(), DecodeError::InvalidChar { char: '^' });
        let res = decode_u64(b"0123456789abcd");
        assert_eq!(res.unwrap_err(), DecodeError::InvalidLength { length: 14 });
    }

    #[test]
    fn both_u64_low() {
        for i in 0..ENC_CROCKFORD_UPPER.len() {
            let s = encode_u64(i as u64);
            // println!("{} vs {} vs {} vs {}", s, i, ENC_CROCKFORD_UPPER[i], ENC_CROCKFORD_UPPER[i] as char);
            assert_eq!(s, (ENC_CROCKFORD_UPPER[i] as char).to_string());
        }
    }

    #[test]
    fn gen_crockford_upper_decode() {
        let mut s = String::new();
        for i in 0..=255u8 {
            let c = i as char;
            #[rustfmt::skip]
            let c = match c {
                '0' | 'o' | 'O' => '0',
                '1' | 'i' | 'I' | 'l' | 'L' => '1',
                '2' ..= '9' => c,
                'u' | 'U' => INVALID_CHAR,
                'A' ..= 'Z' => c,
                'a' ..= 'z' => c.to_ascii_uppercase(),
                _ => INVALID_CHAR,
            };
            s.push(c);
        }

        let dec_chars = DEC_CROCKFORD_UPPER
            .map(|b| {
                if b == INVALID_BYTE {
                    INVALID_CHAR
                } else {
                    ENC_CROCKFORD_UPPER[b as usize] as char
                }
            })
            .iter()
            .collect::<String>();

        assert_eq!(dec_chars, s);
    }
}

// "The quick brown fox jumps over the lazy dog."
// "AHM6A83HENMP6TS0C9S6YXVE41K6YY10D9TPTW3K41QQCSBJ41T6GS90DHGQMY90CHQPEBG"
