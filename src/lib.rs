mod alphabet;
mod decode_base;
mod decode_bytes;
mod decode_u64;
mod decode_u128;
mod encode_base;
mod encode_bytes;
mod encode_u64;
mod encode_u128;

pub use crate::decode_base::DecodeError;
pub use crate::decode_u64::decode_u64;
pub use crate::decode_u128::decode_u128;
pub use crate::decode_u64::decode_u64_str;
pub use crate::decode_u128::decode_u128_str;
pub use crate::encode_u64::encode_u64;
pub use crate::encode_u128::encode_u128;
pub use crate::decode_bytes::decode_bytes;
pub use crate::decode_bytes::decode_bytes_str;
pub use crate::encode_bytes::encode_bytes;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alphabet::{DEC_CROCKFORD_UPPER, ENC_CROCKFORD_UPPER, INVALID_BYTE, INVALID_CHAR};

    #[test]
    fn both_u64_0() {
        let n = 0;
        let x = "0";
        let s = encode_u64(n);
        assert_eq!(s, x);
        assert_eq!(decode_u64(s.as_bytes()).unwrap(), n);
    }

    #[test]
    fn both_bytes_0() {
        let n = &[0x00];
        let x = "00";
        let s = encode_bytes(n);
        assert_eq!(s, x);
        assert_eq!(decode_bytes(s.as_bytes()).unwrap(), n);
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
    fn both_bytes_31() {
        let n = &[0x1F];
        let x = "3W";
        let s = encode_bytes(n);
        assert_eq!(s, x);
        assert_eq!(decode_bytes(s.as_bytes()).unwrap(), n);
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
    fn both_bytes_32() {
        let n = &[0x20];
        let x = "40";
        let s = encode_bytes(n);
        assert_eq!(s, x);
        assert_eq!(decode_bytes(s.as_bytes()).unwrap(), n);
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
    fn both_bytes_33() {
        let n = &[0x21];
        let x = "44";
        let s = encode_bytes(n);
        assert_eq!(s, x);
        assert_eq!(decode_bytes(s.as_bytes()).unwrap(), n);
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
    fn both_bytes_32_32() {
        let n = &[0x04, 0x00];
        let x = "0G00";
        let s = encode_bytes(n);
        assert_eq!(s, x);
        assert_eq!(decode_bytes(s.as_bytes()).unwrap(), n);
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
    fn both_bytes_32_32_32_32() {
        let n = &[0x10, 0x00, 0x00];
        let x = "20000";
        let s = encode_bytes(n);
        assert_eq!(s, x);
        assert_eq!(decode_bytes(s.as_bytes()).unwrap(), n);
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
    fn both_bytes_32_32_32_32_1() {
        let n = &[0x0F, 0xFF, 0xFF];
        let x = "1ZZZY";
        let s = encode_bytes(n);
        assert_eq!(s, x);
        assert_eq!(decode_bytes(s.as_bytes()).unwrap(), n);
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
    fn both_bytes_10239() {
        let n = &[0x27, 0xFF];
        let x = "4ZZG";
        let s = encode_bytes(n);
        assert_eq!(s, x);
        assert_eq!(decode_bytes(s.as_bytes()).unwrap(), n);
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
        assert_eq!(decode_u64(s.as_bytes()).unwrap(), n);
    }

    #[test]
    fn both_bytes_5111() {
        let n = &[0x13, 0xF7];
        let x = "2FVG";
        let s = encode_bytes(n);
        assert_eq!(s, x);
        assert_eq!(decode_bytes(s.as_bytes()).unwrap(), n);
    }

    #[test]
    fn both_u64_1066193093600() {
        let n = 1066193093600;
        let x = "Z0Z0Z0Z0";
        let s = encode_u64(n);
        assert_eq!(s, x);
        assert_eq!(decode_u64(s.as_bytes()).unwrap(), n);
    }

    #[test]
    fn both_bytes_1066193093600() {
        let n = &[0xF8, 0x3E, 0x0F, 0x83, 0xE0];
        // same as u64 encoding because this is exactly 5 bytes,
        // which means they both use exactly 5 bits for each char
        let x = "Z0Z0Z0Z0";
        let s = encode_bytes(n);
        assert_eq!(s, x);
        assert_eq!(decode_bytes(s.as_bytes()).unwrap(), n);
    }

    #[test]
    fn both_u64_big() {
        let n = u64::from_be_bytes([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
        //println!("big: {}", n); // 1311768467294899695
        let x = "14D2PF28AQKFF";
        let s = encode_u64(n);
        assert_eq!(s, x);
        assert_eq!(decode_u64(s.as_bytes()).unwrap(), n);
    }

    #[test]
    fn both_bytes_u64_big() {
        let n = &[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
        let x = "28T5CY4GNF6YY";
        let s = encode_bytes(n);
        assert_eq!(s, x);
        assert_eq!(decode_bytes(s.as_bytes()).unwrap(), n);
    }

    #[test]
    fn both_u64_max() {
        let n = u64::MAX;
        let x = "FZZZZZZZZZZZZ";
        let s = encode_u64(n);
        assert_eq!(s, x);
        assert_eq!(decode_u64(s.as_bytes()).unwrap(), n);
    }

    #[test]
    fn both_bytes_max() {
        let n = &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
        let x = "ZZZZZZZZZZZZY";
        let s = encode_bytes(n);
        assert_eq!(s, x);
        assert_eq!(decode_bytes(s.as_bytes()).unwrap(), n);
    }

    #[test]
    fn both_u128_big() {
        let n = u128::from_be_bytes([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
        // println!("big: {}", n); // 24197857200151252728969465429440056815
        let x = "J6HB7H45BSQQH4D2PF28AQKFF";
        let s = encode_u128(n);
        assert_eq!(s, x);
        assert_eq!(decode_u128(s.as_bytes()).unwrap(), n);
    }

    #[test]
    fn both_u128_max() {
        let n = u128::MAX;
        let x = "7ZZZZZZZZZZZZZZZZZZZZZZZZZ";
        let s = encode_u128(n);
        assert_eq!(s, x);
        assert_eq!(decode_u128(s.as_bytes()).unwrap(), n);
    }

    #[test]
    fn both_u128_med() {
        let n = u128::from_be_bytes([0x00, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD]);
        println!("med: {}", n); // 94522879688090830972536974333750221
        let x = "28T5CY4GNF6YY4HMASW91AYD";
        let s = encode_u128(n);
        assert_eq!(s, x);
        assert_eq!(decode_u128(s.as_bytes()).unwrap(), n);
    }

    // cargo test tests::compare_bytes_u128 -- --exact
    #[test]
    fn compare_bytes_u128() {
        let n = u128::from_be_bytes([0x00, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD]);
        let x = "28T5CY4GNF6YY4HMASW91AYD";
        let e = encode_u128(n);
        assert_eq!(e, x);
        let d = decode_u128(e.as_bytes()).unwrap();
        assert_eq!(d, n);
        let eb = encode_bytes(&[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD]);
        assert_eq!(eb, e);
        let db = decode_bytes(eb.as_bytes()).unwrap();
        assert_eq!(db, d.to_be_bytes()[1..]);
        // TODO: find a way to convert a not 5 bit matchup such that both will give the same outcomes
    }

    #[test]
    fn decode_bad() {
        let res = decode_u64(b"1^_^");
        assert_eq!(res.unwrap_err(), DecodeError::InvalidChar { char: '^', index: 1 });
        let res = decode_u64(b"0123456789ABCD");
        assert_eq!(res.unwrap_err(), DecodeError::InvalidLength { length: 14 });
        let res = decode_u128(b"1^_^");
        assert_eq!(res.unwrap_err(), DecodeError::InvalidChar { char: '^', index: 1 });
        let res = decode_u128(b"0123456789ABCD0123456789ABCD");
        assert_eq!(res.unwrap_err(), DecodeError::InvalidLength { length: 28 });
        let res = decode_bytes(b"111");
        assert_eq!(res.unwrap_err(), DecodeError::InvalidLength { length: 3 });
        let res = decode_bytes(b"11");
        assert_eq!(res.unwrap_err(), DecodeError::InvalidChar { char: '1', index: 1 });
    }

    #[test]
    fn both_u64_low() {
        for i in 0..ENC_CROCKFORD_UPPER.len() {
            let s = encode_u64(i as u64);
            // println!("{} vs {} vs {} vs {}", s, i, ENC_CROCKFORD_UPPER[i], ENC_CROCKFORD_UPPER[i] as char);
            assert_eq!(s, (ENC_CROCKFORD_UPPER[i] as char).to_string());
            assert_eq!(i as u64, decode_u64(s.as_bytes()).unwrap());
        }
    }

    #[test]
    fn both_edges_u128() {
        let rs = [
            (u128::MAX - 100000)..=u128::MAX,
            ((1 << 120) - 10000)..=((1 << 120) + 10000),
            ((1 << 115) - 10000)..=((1 << 115) + 10000),
            ((1 << 110) - 10000)..=((1 << 110) + 10000),
            ((1 << 105) - 10000)..=((1 << 105) + 10000),
            ((1 << 100) - 10000)..=((1 << 100) + 10000),
            ((1 << 95) - 10000)..=((1 << 95) + 10000),
            ((1 << 90) - 10000)..=((1 << 90) + 10000),
            ((1 << 85) - 10000)..=((1 << 85) + 10000),
            ((1 << 80) - 10000)..=((1 << 80) + 10000),
            ((1 << 75) - 10000)..=((1 << 75) + 10000),
            ((1 << 70) - 10000)..=((1 << 70) + 10000),
            ((1 << 65) - 10000)..=((1 << 65) + 10000),
            ((1 << 60) - 10000)..=((1 << 60) + 10000),
            ((1 << 55) - 10000)..=((1 << 55) + 10000),
            ((1 << 50) - 10000)..=((1 << 50) + 10000),
            ((1 << 45) - 10000)..=((1 << 45) + 10000),
            ((1 << 40) - 10000)..=((1 << 40) + 10000),
            ((1 << 35) - 10000)..=((1 << 35) + 10000),
            ((1 << 30) - 10000)..=((1 << 30) + 10000),
            ((1 << 25) - 10000)..=((1 << 25) + 10000),
            ((1 << 20) - 10000)..=((1 << 20) + 10000),
            0..=((1 << 16) + 10000),  // first 4 chars
        ];

        for r in rs {
            for n in r {
                let e = crate::encode_u128(n);
                let d = crate::decode_u128(e.as_bytes()).unwrap();
                assert_eq!(n, d, "mismatch decode for {n}: {e} vs {d}");
            }
        }
    }

    #[test]
    fn both_u128_ones() {
        // first is: Z, F, 7, 3, 1
        // and fill remainder w Z
        let x = [
            "7ZZZZZZZZZZZZZZZZZZZZZZZZZ",
            "3ZZZZZZZZZZZZZZZZZZZZZZZZZ",
            "1ZZZZZZZZZZZZZZZZZZZZZZZZZ",
            "ZZZZZZZZZZZZZZZZZZZZZZZZZ",
            "FZZZZZZZZZZZZZZZZZZZZZZZZ",
            "7ZZZZZZZZZZZZZZZZZZZZZZZZ",
            "3ZZZZZZZZZZZZZZZZZZZZZZZZ",
            "1ZZZZZZZZZZZZZZZZZZZZZZZZ",
            "ZZZZZZZZZZZZZZZZZZZZZZZZ",
            "FZZZZZZZZZZZZZZZZZZZZZZZ",
            "7ZZZZZZZZZZZZZZZZZZZZZZZ",
            "3ZZZZZZZZZZZZZZZZZZZZZZZ",
            "1ZZZZZZZZZZZZZZZZZZZZZZZ",
            "ZZZZZZZZZZZZZZZZZZZZZZZ",
            "FZZZZZZZZZZZZZZZZZZZZZZ",
            "7ZZZZZZZZZZZZZZZZZZZZZZ",
            "3ZZZZZZZZZZZZZZZZZZZZZZ",
            "1ZZZZZZZZZZZZZZZZZZZZZZ",
            "ZZZZZZZZZZZZZZZZZZZZZZ",
            "FZZZZZZZZZZZZZZZZZZZZZ",
            "7ZZZZZZZZZZZZZZZZZZZZZ",
            "3ZZZZZZZZZZZZZZZZZZZZZ",
            "1ZZZZZZZZZZZZZZZZZZZZZ",
            "ZZZZZZZZZZZZZZZZZZZZZ",
            "FZZZZZZZZZZZZZZZZZZZZ",
            "7ZZZZZZZZZZZZZZZZZZZZ",
            "3ZZZZZZZZZZZZZZZZZZZZ",
            "1ZZZZZZZZZZZZZZZZZZZZ",
            "ZZZZZZZZZZZZZZZZZZZZ",
            "FZZZZZZZZZZZZZZZZZZZ",
            "7ZZZZZZZZZZZZZZZZZZZ",
            "3ZZZZZZZZZZZZZZZZZZZ",
            "1ZZZZZZZZZZZZZZZZZZZ",
            "ZZZZZZZZZZZZZZZZZZZ",
            "FZZZZZZZZZZZZZZZZZZ",
            "7ZZZZZZZZZZZZZZZZZZ",
            "3ZZZZZZZZZZZZZZZZZZ",
            "1ZZZZZZZZZZZZZZZZZZ",
            "ZZZZZZZZZZZZZZZZZZ",
            "FZZZZZZZZZZZZZZZZZ",
            "7ZZZZZZZZZZZZZZZZZ",
            "3ZZZZZZZZZZZZZZZZZ",
            "1ZZZZZZZZZZZZZZZZZ",
            "ZZZZZZZZZZZZZZZZZ",
            "FZZZZZZZZZZZZZZZZ",
            "7ZZZZZZZZZZZZZZZZ",
            "3ZZZZZZZZZZZZZZZZ",
            "1ZZZZZZZZZZZZZZZZ",
            "ZZZZZZZZZZZZZZZZ",
            "FZZZZZZZZZZZZZZZ",
            "7ZZZZZZZZZZZZZZZ",
            "3ZZZZZZZZZZZZZZZ",
            "1ZZZZZZZZZZZZZZZ",
            "ZZZZZZZZZZZZZZZ",
            "FZZZZZZZZZZZZZZ",
            "7ZZZZZZZZZZZZZZ",
            "3ZZZZZZZZZZZZZZ",
            "1ZZZZZZZZZZZZZZ",
            "ZZZZZZZZZZZZZZ",
            "FZZZZZZZZZZZZZ",
            "7ZZZZZZZZZZZZZ",
            "3ZZZZZZZZZZZZZ",
            "1ZZZZZZZZZZZZZ",
            "ZZZZZZZZZZZZZ",
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
        for i in 0..u128::BITS as usize {
            let n = u128::MAX >> i;
            let s = encode_u128(n);
            assert_eq!(s, x[i], "mismatch ones encode at {i}: {s} vs {}", x[i]);
            let d = decode_u128(s.as_bytes()).unwrap();
            assert_eq!(d, n, "mismatch ones decode at {i}: {d} vs {n}");
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
            assert_eq!(s, x[i], "mismatch ones encode at {i}: {s} vs {}", x[i]);
            let d = decode_u64(s.as_bytes()).unwrap();
            assert_eq!(d, n, "mismatch ones decode at {i}: {d} vs {n}");
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
