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
    fn both_bytes_0() {
        let n = &[0x00];
        let x = "00";
        let s = encode_bytes(n);
        assert_eq!(s, x);
        assert_eq!(decode_bytes(s).unwrap(), n);
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
    fn both_bytes_31() {
        let n = &[0x1F];
        let x = "3W";
        let s = encode_bytes(n);
        assert_eq!(s, x);
        assert_eq!(decode_bytes(s).unwrap(), n);
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
    fn both_bytes_32() {
        let n = &[0x20];
        let x = "40";
        let s = encode_bytes(n);
        assert_eq!(s, x);
        assert_eq!(decode_bytes(s).unwrap(), n);
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
    fn both_bytes_33() {
        let n = &[0x21];
        let x = "44";
        let s = encode_bytes(n);
        assert_eq!(s, x);
        assert_eq!(decode_bytes(s).unwrap(), n);
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
    fn both_bytes_32_32() {
        let n = &[0x04, 0x00];
        let x = "0G00";
        let s = encode_bytes(n);
        assert_eq!(s, x);
        assert_eq!(decode_bytes(s).unwrap(), n);
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
    fn both_bytes_32_32_32_32() {
        let n = &[0x10, 0x00, 0x00];
        let x = "20000";
        let s = encode_bytes(n);
        assert_eq!(s, x);
        assert_eq!(decode_bytes(s).unwrap(), n);
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
    fn both_bytes_32_32_32_32_1() {
        let n = &[0x0F, 0xFF, 0xFF];
        let x = "1ZZZY";
        let s = encode_bytes(n);
        assert_eq!(s, x);
        assert_eq!(decode_bytes(s).unwrap(), n);
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
    fn both_bytes_10239() {
        let n = &[0x27, 0xFF];
        let x = "4ZZG";
        let s = encode_bytes(n);
        assert_eq!(s, x);
        assert_eq!(decode_bytes(s).unwrap(), n);
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
    fn both_bytes_5111() {
        let n = &[0x13, 0xF7];
        let x = "2FVG";
        let s = encode_bytes(n);
        assert_eq!(s, x);
        assert_eq!(decode_bytes(s).unwrap(), n);
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
    fn both_bytes_1066193093600() {
        let n = &[0xF8, 0x3E, 0x0F, 0x83, 0xE0];
        // don't how this one matches the u64 encoding -- only one seen so far
        let x = "Z0Z0Z0Z0";
        let s = encode_bytes(n);
        assert_eq!(s, x);
        assert_eq!(decode_bytes(s).unwrap(), n);
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
    fn both_bytes_big() {
        let n = &[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
        let x = "28T5CY4GNF6YY";
        let s = encode_bytes(n);
        assert_eq!(s, x);
        assert_eq!(decode_bytes(s).unwrap(), n);
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
    fn both_bytes_max() {
        let n = &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
        let x = "ZZZZZZZZZZZZY";
        let s = encode_bytes(n);
        assert_eq!(s, x);
        assert_eq!(decode_bytes(s).unwrap(), n);
    }

    #[test]
    fn decode_u64_bad() {
        let res = decode_u64("1^_^");
        assert_eq!(res.unwrap_err(), DecodeError::InvalidChar { char: '^', index: 1 });
        let res = decode_u64("0123456789abcd");
        assert_eq!(res.unwrap_err(), DecodeError::InvalidLength { length: 14 });
        let res = decode_bytes("111");
        assert_eq!(res.unwrap_err(), DecodeError::InvalidLength { length: 3 });
        let res = decode_bytes("11");
        assert_eq!(res.unwrap_err(), DecodeError::InvalidChar { char: '1', index: 1 });
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
    fn both_u64_ones() {
        // first is: Z, F, 7, 3, 1
        // and fill remainder w Z
        let x = [
            "FZZZZZZZZZZZZ",
            "7ZZZZZZZZZZZZ",
            "3ZZZZZZZZZZZZ",
            "1ZZZZZZZZZZZZ",
            "ZZZZZZZZZZZZ",
            "FZZZZZZZZZZZ",
            "7ZZZZZZZZZZZ",
            "3ZZZZZZZZZZZ",
            "1ZZZZZZZZZZZ",
            "ZZZZZZZZZZZ",
            "FZZZZZZZZZZ",
            "7ZZZZZZZZZZ",
            "3ZZZZZZZZZZ",
            "1ZZZZZZZZZZ",
            "ZZZZZZZZZZ",
            "FZZZZZZZZZ",
            "7ZZZZZZZZZ",
            "3ZZZZZZZZZ",
            "1ZZZZZZZZZ",
            "ZZZZZZZZZ",
            "FZZZZZZZZ",
            "7ZZZZZZZZ",
            "3ZZZZZZZZ",
            "1ZZZZZZZZ",
            "ZZZZZZZZ",
            "FZZZZZZZ",
            "7ZZZZZZZ",
            "3ZZZZZZZ",
            "1ZZZZZZZ",
            "ZZZZZZZ",
            "FZZZZZZ",
            "7ZZZZZZ",
            "3ZZZZZZ",
            "1ZZZZZZ",
            "ZZZZZZ",
            "FZZZZZ",
            "7ZZZZZ",
            "3ZZZZZ",
            "1ZZZZZ",
            "ZZZZZ",
            "FZZZZ",
            "7ZZZZ",
            "3ZZZZ",
            "1ZZZZ",
            "ZZZZ",
            "FZZZ",
            "7ZZZ",
            "3ZZZ",
            "1ZZZ",
            "ZZZ",
            "FZZ",
            "7ZZ",
            "3ZZ",
            "1ZZ",
            "ZZ",
            "FZ",
            "7Z",
            "3Z",
            "1Z",
            "Z",
            "F",
            "7",
            "3",
            "1",
        ];
        for i in 0..u64::BITS as usize {
            let n = u64::MAX >> i;
            let s = encode_u64(n);
            assert_eq!(s, x[i]);
            assert_eq!(decode_u64(s).unwrap(), n);
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
            assert_eq!(decode_bytes(s).unwrap(), n);

            let s = CROCKFORD32.encode(n);
            assert_eq!(s, x);
            assert_eq!(CROCKFORD32.decode(s.as_bytes()).unwrap(), n);
        }

        {
            // rem 3
            let n = b"The quick brown fox jumps over the lazy dog";
            let x = "AHM6A83HENMP6TS0C9S6YXVE41K6YY10D9TPTW3K41QQCSBJ41T6GS90DHGQMY90CHQPE";

            let s = base32::encode(base32::Alphabet::Crockford, n);
            assert_eq!(s, x);

            let s = encode_bytes(n);
            assert_eq!(s, x);
            assert_eq!(decode_bytes(s).unwrap(), n);

            let s = CROCKFORD32.encode(n);
            assert_eq!(s, x);
            assert_eq!(CROCKFORD32.decode(s.as_bytes()).unwrap(), n);
        }

        {
            // rem 2
            let n = b"The quick brown fox jumps over the lazy do";
            let x = "AHM6A83HENMP6TS0C9S6YXVE41K6YY10D9TPTW3K41QQCSBJ41T6GS90DHGQMY90CHQG";

            let s = base32::encode(base32::Alphabet::Crockford, n);
            assert_eq!(s, x);

            let s = encode_bytes(n);
            assert_eq!(s, x);
            assert_eq!(decode_bytes(s).unwrap(), n);

            let s = CROCKFORD32.encode(n);
            assert_eq!(s, x);
            assert_eq!(CROCKFORD32.decode(s.as_bytes()).unwrap(), n);
        }

        {
            // rem 1
            let n = b"The quick brown fox jumps over the lazy d";
            let x = "AHM6A83HENMP6TS0C9S6YXVE41K6YY10D9TPTW3K41QQCSBJ41T6GS90DHGQMY90CG";

            let s = base32::encode(base32::Alphabet::Crockford, n);
            assert_eq!(s, x);

            let s = encode_bytes(n);
            assert_eq!(s, x);
            assert_eq!(decode_bytes(s).unwrap(), n);

            let s = CROCKFORD32.encode(n);
            assert_eq!(s, x);
            assert_eq!(CROCKFORD32.decode(s.as_bytes()).unwrap(), n);
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
                let d = crate::decode_bytes(e).unwrap();
                assert_eq!(b.to_vec(), d, "mismatch decode for {n}: {b:?} vs {d:?}");
            }
        }
    }
}
