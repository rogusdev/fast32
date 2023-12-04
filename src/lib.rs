mod alphabet;
mod decode_base;
mod decode_bytes;
mod decode_u64;
mod encode_base;
mod encode_bytes;
mod encode_u64;

pub use crate::decode_base::DecodeError;
pub use crate::decode_u64::decode_u64;
pub use crate::encode_u64::encode_u64;
pub use crate::decode_bytes::decode_bytes;
pub use crate::encode_bytes::encode_bytes;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alphabet::{DEC_CROCKFORD_UPPER, ENC_CROCKFORD_UPPER, INVALID_BYTE, INVALID_CHAR};

    const CROCKFORD32: data_encoding::Encoding = data_encoding_macro::new_encoding!{
        symbols: "0123456789ABCDEFGHJKMNPQRSTVWXYZ",
        translate_from: "abcdefghjkmnpqrstvwxyzLIOlio",
        translate_to: "ABCDEFGHJKMNPQRSTVWXYZ110110",
    };

    #[test]
    fn both_u64_0() {
        let n = 0;
        let x = "0";
        let s = encode_u64(n);
        assert_eq!(s, x);
        assert_eq!(decode_u64(s).unwrap(), n);
    }

    #[test]
    fn both_u64_31() {
        let n = 31;
        let x = "Z";
        let s = encode_u64(n);
        assert_eq!(s, x);
        assert_eq!(decode_u64(s).unwrap(), n);
    }

    #[test]
    fn both_u64_32() {
        let n = 32;
        let x = "10";
        let s = encode_u64(n);
        assert_eq!(s, x);
        assert_eq!(decode_u64(s).unwrap(), n);
    }

    #[test]
    fn both_u64_33() {
        let n = 33;
        let x = "11";
        let s = encode_u64(n);
        assert_eq!(s, x);
        assert_eq!(decode_u64(s).unwrap(), n);
    }

    #[test]
    fn both_u64_32_32() {
        let n = 32 * 32;
        let x = "100";
        let s = encode_u64(n);
        assert_eq!(s, x);
        assert_eq!(decode_u64(s).unwrap(), n);
    }

    #[test]
    fn both_u64_32_32_32_32() {
        let n = 32 * 32 * 32 * 32;
        let x = "10000";
        let s = encode_u64(n);
        assert_eq!(s, x);
        assert_eq!(decode_u64(s).unwrap(), n);
    }

    #[test]
    fn both_u64_32_32_32_32_1() {
        let n = (32 * 32 * 32 * 32) - 1;
        let x = "ZZZZ";
        let s = encode_u64(n);
        assert_eq!(s, x);
        assert_eq!(decode_u64(s).unwrap(), n);
    }

    #[test]
    fn both_u64_10239() {
        let n = 10239;
        let x = "9ZZ";
        let s = encode_u64(n);
        assert_eq!(s, x);
        assert_eq!(decode_u64(s).unwrap(), n);
    }

    #[test]
    fn both_bytes_fox() {
        let n = "The quick brown fox jumps over the lazy dog.";
        let x = "AHM6A83HENMP6TS0C9S6YXVE41K6YY10D9TPTW3K41QQCSBJ41T6GS90DHGQMY90CHQPEBG";
        let s = encode_bytes(n.as_bytes());
        assert_eq!(s, x);
    }

    #[test]
    fn both_u64_5111() {
        let n = 5111;
        let x = "4ZQ";
        let s = encode_u64(n);
        assert_eq!(s, x);
        assert_eq!(decode_u64(s).unwrap(), n);
    }

    #[test]
    fn both_u64_1066193093600() {
        let n = 1066193093600;
        let x = "Z0Z0Z0Z0";
        let s = encode_u64(n);
        assert_eq!(s, x);
        assert_eq!(decode_u64(s).unwrap(), n);
    }

    #[test]
    fn both_u64_big() {
        let n = u64::from_be_bytes([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
        //println!("big: {}", n); // 1311768467294899695
        let x = "14D2PF28AQKFF";
        let s = encode_u64(n);
        assert_eq!(s, x);
        assert_eq!(decode_u64(s).unwrap(), n);
    }

    #[test]
    fn both_u64_max() {
        let n = u64::MAX;
        let x = "FZZZZZZZZZZZZ";
        let s = encode_u64(n);
        assert_eq!(s, x);
        assert_eq!(decode_u64(s).unwrap(), n);
    }

    #[test]
    fn decode_u64_bad() {
        let res = decode_u64("1^_^");
        assert_eq!(res.unwrap_err(), DecodeError::InvalidChar { char: '^', index: 1 });
        let res = decode_u64("0123456789abcd");
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

    #[test]
    fn compare_crockford_crate() {
        let rs = [
            (u64::MAX - 100000)..=u64::MAX,
            ((u64::MAX >> 32) - 100000)..=(u64::MAX >> 32),
            0..=100000,
        ];

        for r in rs {
            for n in r {
                let c = crockford::encode(n);
                let e = crate::encode_u64(n);
                assert_eq!(c, e, "mismatch encode for {n}: {c} vs {e}");

                let c = crockford::decode(c).unwrap();
                let e = crate::decode_u64(e).unwrap();
                assert_eq!(c, e, "mismatch decode for {n}: {c} vs {e}");
            }
        }
    }

    #[test]
    fn compare_bytes() {
        {
            // rem 4
            let n = b"The quick brown fox jumps over the lazy dog.";
            let x = "AHM6A83HENMP6TS0C9S6YXVE41K6YY10D9TPTW3K41QQCSBJ41T6GS90DHGQMY90CHQPEBG";

            let s = base32::encode(base32::Alphabet::Crockford, n);
            assert_eq!(s, x);

            let s = encode_bytes(n);
            assert_eq!(s, x);

            let s = CROCKFORD32.encode(n);
            assert_eq!(s, x);
        }

        {
            // rem 3
            let n = b"The quick brown fox jumps over the lazy dog";
            let x = "AHM6A83HENMP6TS0C9S6YXVE41K6YY10D9TPTW3K41QQCSBJ41T6GS90DHGQMY90CHQPE";

            let s = base32::encode(base32::Alphabet::Crockford, n);
            assert_eq!(s, x);

            let s = encode_bytes(n);
            assert_eq!(s, x);

            let s = CROCKFORD32.encode(n);
            assert_eq!(s, x);
        }

        {
            // rem 2
            let n = b"The quick brown fox jumps over the lazy do";
            let x = "AHM6A83HENMP6TS0C9S6YXVE41K6YY10D9TPTW3K41QQCSBJ41T6GS90DHGQMY90CHQG";

            let s = base32::encode(base32::Alphabet::Crockford, n);
            assert_eq!(s, x);

            let s = encode_bytes(n);
            assert_eq!(s, x);

            let s = CROCKFORD32.encode(n);
            assert_eq!(s, x);
        }

        {
            // rem 1
            let n = b"The quick brown fox jumps over the lazy d";
            let x = "AHM6A83HENMP6TS0C9S6YXVE41K6YY10D9TPTW3K41QQCSBJ41T6GS90DHGQMY90CG";

            let s = base32::encode(base32::Alphabet::Crockford, n);
            assert_eq!(s, x);

            let s = encode_bytes(n);
            assert_eq!(s, x);

            let s = CROCKFORD32.encode(n);
            assert_eq!(s, x);
        }

        let rs = [
            (u64::MAX - 100000)..=u64::MAX,
            ((u64::MAX >> 32) - 100000)..=(u64::MAX >> 32),
            0..=100000,
        ];

        for r in rs {
            for n in r {
                let b = &n.to_be_bytes();
                let c = base32::encode(base32::Alphabet::Crockford, b);
                let e = crate::encode_bytes(b);
                assert_eq!(c, e, "mismatch encode for {n}: {c} vs {e}");
                let c = CROCKFORD32.encode(b);
                assert_eq!(c, e, "mismatch encode for {n}: {c} vs {e}");
            }
        }
    }
}
