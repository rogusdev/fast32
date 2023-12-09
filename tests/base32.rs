use fast32::base32::{CROCKFORD, RFC4648, RFC4648_HEX, RFC4648_HEX_NOPAD, RFC4648_NOPAD};
use fast32::DecodeError;

#[test]
fn both_u64_0() {
    let n = 0;
    let x = "0";
    let s = CROCKFORD.encode_u64(n);
    assert_eq!(s, x);
    assert_eq!(CROCKFORD.decode_u64(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_u64_0_rfc4648() {
    let n = 0;
    let x = "A";
    let s = RFC4648_NOPAD.encode_u64(n);
    assert_eq!(s, x);
    assert_eq!(RFC4648_NOPAD.decode_u64(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_bytes_0() {
    let n = &[0x00];
    let x = "00";
    let s = CROCKFORD.encode_bytes(n);
    assert_eq!(s, x);
    assert_eq!(CROCKFORD.decode_bytes(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_u64_31() {
    let n = 31;
    let x = "Z";
    let s = CROCKFORD.encode_u64(n);
    assert_eq!(s, x);
    assert_eq!(CROCKFORD.decode_u64(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_u64_31_into() {
    let n = 31;
    let x = "Z";
    let mut b = Vec::<u8>::with_capacity(2);
    b.push(201);
    CROCKFORD.encode_u64_into(n, &mut b);
    b.push(202);
    assert_eq!(&b[1..2], x.as_bytes());
    assert_eq!(CROCKFORD.decode_u64(&b[1..2]).unwrap(), n);
}

#[test]
fn both_bytes_31() {
    let n = &[0x1F];
    let x = "3W";
    let s = CROCKFORD.encode_bytes(n);
    assert_eq!(s, x);
    assert_eq!(CROCKFORD.decode_bytes(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_bytes_31_rfc4648() {
    let n = &[0x1F];
    let x = "D4";
    let s = RFC4648_NOPAD.encode_bytes(n);
    assert_eq!(s, x);
    assert_eq!(RFC4648_NOPAD.decode_bytes(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_u64_32() {
    let n = 32;
    let x = "10";
    let s = CROCKFORD.encode_u64(n);
    assert_eq!(s, x);
    assert_eq!(CROCKFORD.decode_u64(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_bytes_32() {
    let n = &[0x20];
    let x = "40";
    let s = CROCKFORD.encode_bytes(n);
    assert_eq!(s, x);
    assert_eq!(CROCKFORD.decode_bytes(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_bytes_32_rfc4648() {
    let n = &[0x20];
    let x = "EA";
    let s = RFC4648_NOPAD.encode_bytes(n);
    assert_eq!(s, x);
    assert_eq!(RFC4648_NOPAD.decode_bytes(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_u64_33() {
    let n = 33;
    let x = "11";
    let s = CROCKFORD.encode_u64(n);
    assert_eq!(s, x);
    assert_eq!(CROCKFORD.decode_u64(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_bytes_33() {
    let n = &[0x21];
    let x = "44";
    let s = CROCKFORD.encode_bytes(n);
    assert_eq!(s, x);
    assert_eq!(CROCKFORD.decode_bytes(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_u64_111() {
    let n = 111;
    let x = "3F";
    let s = CROCKFORD.encode_u64(n);
    assert_eq!(s, x);
    assert_eq!(CROCKFORD.decode_u64(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_u128_111() {
    let n = 111;
    let x = "3F";
    let s = CROCKFORD.encode_u128(n);
    assert_eq!(s, x);
    assert_eq!(CROCKFORD.decode_u128(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_u64_32_32() {
    let n = 32 * 32;
    let x = "100";
    let s = CROCKFORD.encode_u64(n);
    assert_eq!(s, x);
    assert_eq!(CROCKFORD.decode_u64(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_bytes_32_32() {
    let n = &[0x04, 0x00];
    let x = "0G00";
    let s = CROCKFORD.encode_bytes(n);
    assert_eq!(s, x);
    assert_eq!(CROCKFORD.decode_bytes(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_u64_32_32_32_32() {
    let n = 32 * 32 * 32 * 32;
    let x = "10000";
    let s = CROCKFORD.encode_u64(n);
    assert_eq!(s, x);
    assert_eq!(CROCKFORD.decode_u64(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_bytes_32_32_32_32() {
    let n = &[0x10, 0x00, 0x00];
    let x = "20000";
    let s = CROCKFORD.encode_bytes(n);
    assert_eq!(s, x);
    assert_eq!(CROCKFORD.decode_bytes(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_u64_32_32_32_32_1() {
    let n = (32 * 32 * 32 * 32) - 1;
    let x = "ZZZZ";
    let s = CROCKFORD.encode_u64(n);
    assert_eq!(s, x);
    assert_eq!(CROCKFORD.decode_u64(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_bytes_32_32_32_32_1() {
    let n = &[0x0F, 0xFF, 0xFF];
    let x = "1ZZZY";
    let s = CROCKFORD.encode_bytes(n);
    assert_eq!(s, x);
    assert_eq!(CROCKFORD.decode_bytes(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_u64_10238() {
    let n = 10238;
    let x = "9ZY";
    let s = CROCKFORD.encode_u64(n);
    assert_eq!(s, x);
    assert_eq!(CROCKFORD.decode_u64(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_bytes_10238() {
    let n = &[0x27, 0xFE];
    let x = "4ZZ0";
    let s = CROCKFORD.encode_bytes(n);
    assert_eq!(s, x);
    assert_eq!(CROCKFORD.decode_bytes(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_bytes_fox() {
    let n = "The quick brown fox jumps over the lazy dog.";
    let x = "AHM6A83HENMP6TS0C9S6YXVE41K6YY10D9TPTW3K41QQCSBJ41T6GS90DHGQMY90CHQPEBG";
    let s = CROCKFORD.encode_bytes(n.as_bytes());
    assert_eq!(s, x);
}

#[test]
fn both_u64_5111() {
    let n = 5111;
    let x = "4ZQ";
    let s = CROCKFORD.encode_u64(n);
    assert_eq!(s, x);
    assert_eq!(CROCKFORD.decode_u64(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_bytes_5111() {
    let n = &[0x13, 0xF7];
    let x = "2FVG";
    let s = CROCKFORD.encode_bytes(n);
    assert_eq!(s, x);
    assert_eq!(CROCKFORD.decode_bytes(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_u64_1066193093600() {
    let n = 1066193093600;
    let x = "Z0Z0Z0Z0";
    let s = CROCKFORD.encode_u64(n);
    assert_eq!(s, x);
    assert_eq!(CROCKFORD.decode_u64(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_bytes_1066193093600() {
    let n = &[0xF8, 0x3E, 0x0F, 0x83, 0xE0];
    // same as u64 encoding because this is exactly 5 bytes,
    // which means they both use exactly 5 bits for each char
    let x = "Z0Z0Z0Z0";
    let s = CROCKFORD.encode_bytes(n);
    assert_eq!(s, x);
    assert_eq!(CROCKFORD.decode_bytes(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_u64_big() {
    let n = u64::from_be_bytes([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    //println!("big: {}", n); // 1311768467294899695
    let x = "14D2PF28AQKFF";
    let s = CROCKFORD.encode_u64(n);
    assert_eq!(s, x);
    assert_eq!(CROCKFORD.decode_u64(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_bytes_u64_big() {
    let n = &[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    let x = "28T5CY4GNF6YY";
    let s = CROCKFORD.encode_bytes(n);
    assert_eq!(s, x);
    assert_eq!(CROCKFORD.decode_bytes(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_u64_max() {
    let n = u64::MAX;
    let x = "FZZZZZZZZZZZZ";
    let s = CROCKFORD.encode_u64(n);
    assert_eq!(s, x);
    assert_eq!(CROCKFORD.decode_u64(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_bytes_max() {
    let n = &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    let x = "ZZZZZZZZZZZZY";
    let s = CROCKFORD.encode_bytes(n);
    assert_eq!(s, x);
    assert_eq!(CROCKFORD.decode_bytes(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_u128_big() {
    let n = u128::from_be_bytes([
        0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD,
        0xEF,
    ]);
    // println!("big: {}", n); // 24197857200151252728969465429440056815
    let x = "J6HB7H45BSQQH4D2PF28AQKFF";
    let s = CROCKFORD.encode_u128(n);
    assert_eq!(s, x);
    assert_eq!(CROCKFORD.decode_u128(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_u128_max() {
    let n = u128::MAX;
    let x = "7ZZZZZZZZZZZZZZZZZZZZZZZZZ";
    let s = CROCKFORD.encode_u128(n);
    assert_eq!(s, x);
    assert_eq!(CROCKFORD.decode_u128(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_u128_med() {
    let n = u128::from_be_bytes([
        0x00, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB,
        0xCD,
    ]);
    // println!("med: {}", n); // 94522879688090830972536974333750221
    let x = "28T5CY4GNF6YY4HMASW91AYD";
    let s = CROCKFORD.encode_u128(n);
    assert_eq!(s, x);
    assert_eq!(CROCKFORD.decode_u128(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_u128_med_into() {
    let n = u128::from_be_bytes([
        0x00, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB,
        0xCD,
    ]);
    let x = "28T5CY4GNF6YY4HMASW91AYD";
    let mut b = Vec::<u8>::with_capacity(25);
    b.push(201);
    CROCKFORD.encode_u128_into(n, &mut b);
    b.push(202);
    assert_eq!(&b[1..25], x.as_bytes());
    assert_eq!(CROCKFORD.decode_u128(&b[1..25]).unwrap(), n);
}

#[test]
fn both_bytes_into() {
    let n = &[
        0x00, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB,
        0xCD,
    ];
    let x = "00938NKRJ2NWVVRJ6HB7H45BSM";
    let s = CROCKFORD.encode_bytes(n);
    assert_eq!(s, x);

    let mut b = Vec::<u8>::with_capacity(26);
    CROCKFORD.encode_bytes_into(n, &mut b);
    assert_eq!(&b[0..26], x.as_bytes());
    assert_eq!(CROCKFORD.decode_bytes(&b[0..26]).unwrap(), n);

    let mut b = Vec::<u8>::with_capacity(27);
    b.push(201);
    CROCKFORD.encode_bytes_into(n, &mut b);
    b.push(202);
    assert_eq!(String::from_utf8(b[1..27].to_vec()).unwrap(), x);
    assert_eq!(&b[1..27], x.as_bytes());
    assert_eq!(CROCKFORD.decode_bytes(&b[1..27]).unwrap(), n);

    let mut b2 = Vec::<u8>::with_capacity(17);
    b2.push(201);
    CROCKFORD.decode_bytes_into(&b[1..27], &mut b2).unwrap();
    b2.push(202);
    assert_eq!(&b2[1..17], n);
}

#[test]
#[should_panic(expected = "Missing capacity for encoding")]
fn panic_bytes_into() {
    let n = &[0x00, 0x12];
    let mut b = Vec::<u8>::with_capacity(5);
    CROCKFORD.encode_bytes_into(n, &mut b);
    assert_eq!(&b, b"0090");
    let mut b = Vec::<u8>::with_capacity(3);
    CROCKFORD.encode_bytes_into(n, &mut b);
}

#[test]
#[should_panic(expected = "Missing capacity for padding")]
fn panic_pad() {
    let n = &[0x00, 0x12];
    let mut b = Vec::<u8>::with_capacity(8);
    RFC4648.encode_bytes_into(n, &mut b);
    assert_eq!(&b, b"AAJA====");
    let mut b = Vec::<u8>::with_capacity(7);
    RFC4648.encode_bytes_into(n, &mut b);
}

#[test]
#[should_panic(expected = "Missing capacity for encoding")]
fn panic_encode_u64_into() {
    let n = 111;
    let mut b = Vec::<u8>::with_capacity(2);
    CROCKFORD.encode_u64_into(n, &mut b);
    assert_eq!(&b, b"3F");
    let mut b = Vec::<u8>::with_capacity(1);
    CROCKFORD.encode_u64_into(n, &mut b);
}

#[test]
#[should_panic(expected = "Missing capacity for encoding")]
fn panic_encode_u128_into() {
    let n = 111;
    let mut b = Vec::<u8>::with_capacity(2);
    CROCKFORD.encode_u128_into(n, &mut b);
    assert_eq!(&b, b"3F");
    let mut b = Vec::<u8>::with_capacity(1);
    CROCKFORD.encode_u128_into(n, &mut b);
}

// cargo test tests::compare_bytes_u128 -- --exact
#[test]
fn compare_bytes_u128() {
    let n = u128::from_be_bytes([
        0x00, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB,
        0xCD,
    ]);
    let x = "28T5CY4GNF6YY4HMASW91AYD";
    let e = CROCKFORD.encode_u128(n);
    assert_eq!(e, x);
    let d = CROCKFORD.decode_u128(e.as_bytes()).unwrap();
    assert_eq!(d, n);
    let eb = CROCKFORD.encode_bytes(&[
        0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD,
    ]);
    assert_eq!(eb, e);
    let db = CROCKFORD.decode_bytes(eb.as_bytes()).unwrap();
    assert_eq!(db, d.to_be_bytes()[1..]);
    // TODO: find a way to convert a not 5 bit matchup such that both will give the same outcomes
}

#[test]
fn both_u64_low() {
    let enc = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";
    for i in 0..enc.len() {
        let s = CROCKFORD.encode_u64(i as u64);
        // println!("{} vs {} vs {} vs {}", s, i, enc[i], enc[i] as char);
        assert_eq!(s, (enc[i] as char).to_string());
        assert_eq!(i as u64, CROCKFORD.decode_u64(s.as_bytes()).unwrap());
    }
}

#[test]
fn decode_bad() {
    let res = CROCKFORD.decode_u64(b"1^_^");
    assert_eq!(
        res.unwrap_err(),
        DecodeError::InvalidChar {
            char: '^',
            index: 1
        }
    );
    let res = CROCKFORD.decode_u64(b"0123456789ABCD");
    assert_eq!(res.unwrap_err(), DecodeError::InvalidLength { length: 14 });
    let res = CROCKFORD.decode_u128(b"1^_^");
    assert_eq!(
        res.unwrap_err(),
        DecodeError::InvalidChar {
            char: '^',
            index: 1
        }
    );
    let res = CROCKFORD.decode_u128(b"0123456789ABCD0123456789ABC");
    assert_eq!(res.unwrap_err(), DecodeError::InvalidLength { length: 27 });
    let res = CROCKFORD.decode_bytes(b"111");
    assert_eq!(res.unwrap_err(), DecodeError::InvalidLength { length: 3 });
    let res = CROCKFORD.decode_bytes(b"11");
    assert_eq!(
        res.unwrap_err(),
        DecodeError::InvalidChar {
            char: '1',
            index: 1
        }
    );
}

#[rustfmt::skip]
#[test]
fn rfc4648() {
    // https://datatracker.ietf.org/doc/html/rfc4648#section-10
    assert_eq!(RFC4648_NOPAD.encode_bytes_str(""), "");
    assert_eq!(RFC4648_NOPAD.encode_bytes_str("f"), "MY");
    assert_eq!(RFC4648_NOPAD.encode_bytes_str("fo"), "MZXQ");
    assert_eq!(RFC4648_NOPAD.encode_bytes_str("foo"), "MZXW6");
    assert_eq!(RFC4648_NOPAD.encode_bytes_str("foob"), "MZXW6YQ");
    assert_eq!(RFC4648_NOPAD.encode_bytes_str("fooba"), "MZXW6YTB");
    assert_eq!(RFC4648_NOPAD.encode_bytes_str("foobar"), "MZXW6YTBOI");

    assert_eq!(RFC4648_NOPAD.decode_bytes_str(""), Ok(b"".to_vec()));
    assert_eq!(RFC4648_NOPAD.decode_bytes_str("MY"), Ok(b"f".to_vec()));
    assert_eq!(RFC4648_NOPAD.decode_bytes_str("MZXQ"), Ok(b"fo".to_vec()));
    assert_eq!(RFC4648_NOPAD.decode_bytes_str("MZXW6"), Ok(b"foo".to_vec()));
    assert_eq!(RFC4648_NOPAD.decode_bytes_str("MZXW6YQ"), Ok(b"foob".to_vec()));
    assert_eq!(RFC4648_NOPAD.decode_bytes_str("MZXW6YTB"), Ok(b"fooba".to_vec()));
    assert_eq!(RFC4648_NOPAD.decode_bytes_str("MZXW6YTBOI"), Ok(b"foobar".to_vec()));

    assert_eq!(RFC4648_HEX_NOPAD.encode_bytes_str(""), "");
    assert_eq!(RFC4648_HEX_NOPAD.encode_bytes_str("f"), "CO");
    assert_eq!(RFC4648_HEX_NOPAD.encode_bytes_str("fo"), "CPNG");
    assert_eq!(RFC4648_HEX_NOPAD.encode_bytes_str("foo"), "CPNMU");
    assert_eq!(RFC4648_HEX_NOPAD.encode_bytes_str("foob"), "CPNMUOG");
    assert_eq!(RFC4648_HEX_NOPAD.encode_bytes_str("fooba"), "CPNMUOJ1");
    assert_eq!(RFC4648_HEX_NOPAD.encode_bytes_str("foobar"), "CPNMUOJ1E8");

    assert_eq!(RFC4648_HEX_NOPAD.decode_bytes_str(""), Ok(b"".to_vec()));
    assert_eq!(RFC4648_HEX_NOPAD.decode_bytes_str("CO"), Ok(b"f".to_vec()));
    assert_eq!(RFC4648_HEX_NOPAD.decode_bytes_str("CPNG"), Ok(b"fo".to_vec()));
    assert_eq!(RFC4648_HEX_NOPAD.decode_bytes_str("CPNMU"), Ok(b"foo".to_vec()));
    assert_eq!(RFC4648_HEX_NOPAD.decode_bytes_str("CPNMUOG"), Ok(b"foob".to_vec()));
    assert_eq!(RFC4648_HEX_NOPAD.decode_bytes_str("CPNMUOJ1"), Ok(b"fooba".to_vec()));
    assert_eq!(RFC4648_HEX_NOPAD.decode_bytes_str("CPNMUOJ1E8"), Ok(b"foobar".to_vec()));

    assert_eq!(RFC4648.encode_bytes_str(""), "");
    assert_eq!(RFC4648.encode_bytes_str("f"), "MY======");
    assert_eq!(RFC4648.encode_bytes_str("fo"), "MZXQ====");
    assert_eq!(RFC4648.encode_bytes_str("foo"), "MZXW6===");
    assert_eq!(RFC4648.encode_bytes_str("foob"), "MZXW6YQ=");
    assert_eq!(RFC4648.encode_bytes_str("fooba"), "MZXW6YTB");
    assert_eq!(RFC4648.encode_bytes_str("foobar"), "MZXW6YTBOI======");

    assert_eq!(RFC4648.decode_bytes_str(""), Ok(b"".to_vec()));
    assert_eq!(RFC4648.decode_bytes_str("MY======"), Ok(b"f".to_vec()));
    assert_eq!(RFC4648.decode_bytes_str("MZXQ===="), Ok(b"fo".to_vec()));
    assert_eq!(RFC4648.decode_bytes_str("MZXW6==="), Ok(b"foo".to_vec()));
    assert_eq!(RFC4648.decode_bytes_str("MZXW6YQ="), Ok(b"foob".to_vec()));
    assert_eq!(RFC4648.decode_bytes_str("MZXW6YTB"), Ok(b"fooba".to_vec()));
    assert_eq!(RFC4648.decode_bytes_str("MZXW6YTBOI======"), Ok(b"foobar".to_vec()));

    assert_eq!(RFC4648.encode_bytes(b""), "");
    assert_eq!(RFC4648.encode_bytes(b"f"), "MY======");
    assert_eq!(RFC4648.encode_bytes(b"fo"), "MZXQ====");
    assert_eq!(RFC4648.encode_bytes(b"foo"), "MZXW6===");
    assert_eq!(RFC4648.encode_bytes(b"foob"), "MZXW6YQ=");
    assert_eq!(RFC4648.encode_bytes(b"fooba"), "MZXW6YTB");
    assert_eq!(RFC4648.encode_bytes(b"foobar"), "MZXW6YTBOI======");

    assert_eq!(RFC4648.decode_bytes(b""), Ok(b"".to_vec()));
    assert_eq!(RFC4648.decode_bytes(b"MY======"), Ok(b"f".to_vec()));
    assert_eq!(RFC4648.decode_bytes(b"MZXQ===="), Ok(b"fo".to_vec()));
    assert_eq!(RFC4648.decode_bytes(b"MZXW6==="), Ok(b"foo".to_vec()));
    assert_eq!(RFC4648.decode_bytes(b"MZXW6YQ="), Ok(b"foob".to_vec()));
    assert_eq!(RFC4648.decode_bytes(b"MZXW6YTB"), Ok(b"fooba".to_vec()));
    assert_eq!(RFC4648.decode_bytes(b"MZXW6YTBOI======"), Ok(b"foobar".to_vec()));

    assert_eq!(RFC4648_HEX.encode_bytes_str(""), "");
    assert_eq!(RFC4648_HEX.encode_bytes_str("f"), "CO======");
    assert_eq!(RFC4648_HEX.encode_bytes_str("fo"), "CPNG====");
    assert_eq!(RFC4648_HEX.encode_bytes_str("foo"), "CPNMU===");
    assert_eq!(RFC4648_HEX.encode_bytes_str("foob"), "CPNMUOG=");
    assert_eq!(RFC4648_HEX.encode_bytes_str("fooba"), "CPNMUOJ1");
    assert_eq!(RFC4648_HEX.encode_bytes_str("foobar"), "CPNMUOJ1E8======");

    assert_eq!(RFC4648_HEX.decode_bytes_str(""), Ok(b"".to_vec()));
    assert_eq!(RFC4648_HEX.decode_bytes_str("CO======"), Ok(b"f".to_vec()));
    assert_eq!(RFC4648_HEX.decode_bytes_str("CPNG===="), Ok(b"fo".to_vec()));
    assert_eq!(RFC4648_HEX.decode_bytes_str("CPNMU==="), Ok(b"foo".to_vec()));
    assert_eq!(RFC4648_HEX.decode_bytes_str("CPNMUOG="), Ok(b"foob".to_vec()));
    assert_eq!(RFC4648_HEX.decode_bytes_str("CPNMUOJ1"), Ok(b"fooba".to_vec()));
    assert_eq!(RFC4648_HEX.decode_bytes_str("CPNMUOJ1E8======"), Ok(b"foobar".to_vec()));

    assert_eq!(CROCKFORD.encode_bytes_str(""), "");
    assert_eq!(CROCKFORD.encode_bytes_str("f"), "CR");
    assert_eq!(CROCKFORD.encode_bytes_str("fo"), "CSQG");
    assert_eq!(CROCKFORD.encode_bytes_str("foo"), "CSQPY");
    assert_eq!(CROCKFORD.encode_bytes_str("foob"), "CSQPYRG");
    assert_eq!(CROCKFORD.encode_bytes_str("fooba"), "CSQPYRK1");
    assert_eq!(CROCKFORD.encode_bytes_str("foobar"), "CSQPYRK1E8");

    assert_eq!(CROCKFORD.decode_bytes_str(""), Ok(b"".to_vec()));
    assert_eq!(CROCKFORD.decode_bytes_str("CR"), Ok(b"f".to_vec()));
    assert_eq!(CROCKFORD.decode_bytes_str("CSQG"), Ok(b"fo".to_vec()));
    assert_eq!(CROCKFORD.decode_bytes_str("CSQPY"), Ok(b"foo".to_vec()));
    assert_eq!(CROCKFORD.decode_bytes_str("CSQPYRG"), Ok(b"foob".to_vec()));
    assert_eq!(CROCKFORD.decode_bytes_str("CSQPYRK1"), Ok(b"fooba".to_vec()));
    assert_eq!(CROCKFORD.decode_bytes_str("CSQPYRK1E8"), Ok(b"foobar".to_vec()));
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
        0..=((1 << 16) + 10000), // first 4 chars
    ];

    for r in rs {
        for n in r {
            let e = crate::CROCKFORD.encode_u128(n);
            let d = crate::CROCKFORD.decode_u128(e.as_bytes()).unwrap();
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
        let s = CROCKFORD.encode_u128(n);
        assert_eq!(s, x[i], "mismatch ones encode at {i}: {s} vs {}", x[i]);
        let d = CROCKFORD.decode_u128(s.as_bytes()).unwrap();
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
        let s = CROCKFORD.encode_u64(n);
        assert_eq!(s, x[i], "mismatch ones encode at {i}: {s} vs {}", x[i]);
        let d = CROCKFORD.decode_u64(s.as_bytes()).unwrap();
        assert_eq!(d, n, "mismatch ones decode at {i}: {d} vs {n}");
    }
}
