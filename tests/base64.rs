use fast32::base64::{RFC4648, RFC4648_NOPAD};
use fast32::DecodeError;

#[test]
fn both_u64_0() {
    let n = 0;
    let x = "A";
    let s = RFC4648_NOPAD.encode_u64(n);
    assert_eq!(s, x);
    assert_eq!(RFC4648_NOPAD.decode_u64(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_bytes_0() {
    let n = &[0x00];
    let x = "AA";
    let s = RFC4648_NOPAD.encode(n);
    assert_eq!(s, x);
    assert_eq!(RFC4648_NOPAD.decode(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_u64_31() {
    let n = 31;
    let x = "f";
    let s = RFC4648_NOPAD.encode_u64(n);
    assert_eq!(s, x);
    assert_eq!(RFC4648_NOPAD.decode_u64(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_u64_31_into() {
    let n = 31;
    let x = "f";
    let mut b = Vec::<u8>::with_capacity(2);
    b.push(201);
    RFC4648_NOPAD.encode_u64_into(n, &mut b);
    b.push(202);
    assert_eq!(&b[1..2], x.as_bytes());
    assert_eq!(RFC4648_NOPAD.decode_u64(&b[1..2]).unwrap(), n);
}

#[test]
fn both_bytes_31() {
    let n = &[0x1F];
    let x = "Hw";
    let s = RFC4648_NOPAD.encode(n);
    assert_eq!(s, x);
    assert_eq!(RFC4648_NOPAD.decode(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_u64_63() {
    let n = 63;
    let x = "/";
    let s = RFC4648_NOPAD.encode_u64(n);
    assert_eq!(s, x);
    assert_eq!(RFC4648_NOPAD.decode_u64(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_bytes_63() {
    let n = &[0x3F];
    let x = "Pw";
    let s = RFC4648_NOPAD.encode(n);
    assert_eq!(s, x);
    assert_eq!(RFC4648_NOPAD.decode(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_u64_64() {
    let n = 64;
    let x = "BA";
    let s = RFC4648_NOPAD.encode_u64(n);
    assert_eq!(s, x);
    assert_eq!(RFC4648_NOPAD.decode_u64(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_bytes_64() {
    let n = &[0x40];
    let x = "QA";
    let s = RFC4648_NOPAD.encode(n);
    assert_eq!(s, x);
    assert_eq!(RFC4648_NOPAD.decode(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_u64_65() {
    let n = 65;
    let x = "BB";
    let s = RFC4648_NOPAD.encode_u64(n);
    assert_eq!(s, x);
    assert_eq!(RFC4648_NOPAD.decode_u64(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_bytes_65() {
    let n = &[0x41];
    let x = "QQ";
    let s = RFC4648_NOPAD.encode(n);
    assert_eq!(s, x);
    assert_eq!(RFC4648_NOPAD.decode(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_u64_111() {
    let n = 111;
    let x = "Bv";
    let s = RFC4648_NOPAD.encode_u64(n);
    assert_eq!(s, x);
    assert_eq!(RFC4648_NOPAD.decode_u64(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_u128_111() {
    let n = 111;
    let x = "Bv";
    let s = RFC4648_NOPAD.encode_u128(n);
    assert_eq!(s, x);
    assert_eq!(RFC4648_NOPAD.decode_u128(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_u64_64_64() {
    let n = 64 * 64;
    let x = "BAA";
    let s = RFC4648_NOPAD.encode_u64(n);
    assert_eq!(s, x);
    assert_eq!(RFC4648_NOPAD.decode_u64(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_u64_10238() {
    let n = 10238;
    let x = "Cf+";
    let s = RFC4648_NOPAD.encode_u64(n);
    assert_eq!(s, x);
    assert_eq!(RFC4648_NOPAD.decode_u64(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_bytes_10238() {
    let n = &[0x27, 0xFE];
    let x = "J/4";
    let s = RFC4648_NOPAD.encode(n);
    assert_eq!(s, x);
    assert_eq!(RFC4648_NOPAD.decode(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_bytes_fox() {
    let n = "The quick brown fox jumps over the lazy dog.";
    let x = "VGhlIHF1aWNrIGJyb3duIGZveCBqdW1wcyBvdmVyIHRoZSBsYXp5IGRvZy4";
    let s = RFC4648_NOPAD.encode(n.as_bytes());
    assert_eq!(s, x);
}

#[test]
fn both_u64_5111() {
    let n = 5111;
    let x = "BP3";
    let s = RFC4648_NOPAD.encode_u64(n);
    assert_eq!(s, x);
    assert_eq!(RFC4648_NOPAD.decode_u64(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_bytes_5111() {
    let n = &[0x13, 0xF7];
    let x = "E/c";
    let s = RFC4648_NOPAD.encode(n);
    assert_eq!(s, x);
    assert_eq!(RFC4648_NOPAD.decode(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_u64_1066193093600() {
    let n = 1066193093600;
    let x = "Pg+D4Pg";
    let s = RFC4648_NOPAD.encode_u64(n);
    assert_eq!(s, x);
    assert_eq!(RFC4648_NOPAD.decode_u64(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_bytes_1066193093600() {
    let n = &[0xF8, 0x3E, 0x0F, 0x83, 0xE0];
    let x = "+D4Pg+A";
    let s = RFC4648_NOPAD.encode(n);
    assert_eq!(s, x);
    assert_eq!(RFC4648_NOPAD.decode(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_u64_big() {
    let n = u64::from_be_bytes([0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]);
    let x = "BI0VniQq83v";
    let s = RFC4648_NOPAD.encode_u64(n);
    assert_eq!(s, x);
    assert_eq!(RFC4648_NOPAD.decode_u64(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_bytes_u64_big() {
    let n = &[0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
    let x = "EjRWeJCrze8";
    let s = RFC4648_NOPAD.encode(n);
    assert_eq!(s, x);
    assert_eq!(RFC4648_NOPAD.decode(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_u64_max() {
    let n = u64::MAX;
    let x = "P//////////";
    let s = RFC4648_NOPAD.encode_u64(n);
    assert_eq!(s, x);
    assert_eq!(RFC4648_NOPAD.decode_u64(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_bytes_max() {
    let n = &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    let x = "//////////8";
    let s = RFC4648_NOPAD.encode(n);
    assert_eq!(s, x);
    assert_eq!(RFC4648_NOPAD.decode(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_u128_big() {
    let n = u128::from_be_bytes([
        0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD,
        0xEF,
    ]);
    let x = "SNFZ4kKvN7xI0VniQq83v";
    let s = RFC4648_NOPAD.encode_u128(n);
    assert_eq!(s, x);
    assert_eq!(RFC4648_NOPAD.decode_u128(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_u128_max() {
    let n = u128::MAX;
    let x = "D/////////////////////";
    let s = RFC4648_NOPAD.encode_u128(n);
    assert_eq!(s, x);
    assert_eq!(RFC4648_NOPAD.decode_u128(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_u128_med() {
    let n = u128::from_be_bytes([
        0x00, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB,
        0xCD,
    ]);
    let x = "EjRWeJCrze8SNFZ4kKvN";
    let s = RFC4648_NOPAD.encode_u128(n);
    assert_eq!(s, x);
    assert_eq!(RFC4648_NOPAD.decode_u128(s.as_bytes()).unwrap(), n);
}

#[test]
fn both_u128_med_into() {
    let n = u128::from_be_bytes([
        0x00, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB,
        0xCD,
    ]);
    let x = "EjRWeJCrze8SNFZ4kKvN";
    let mut b = Vec::<u8>::with_capacity(21);
    b.push(201);
    RFC4648_NOPAD.encode_u128_into(n, &mut b);
    b.push(202);
    assert_eq!(&b[1..21], x.as_bytes());
    assert_eq!(RFC4648_NOPAD.decode_u128(&b[1..21]).unwrap(), n);
}

#[test]
fn both_bytes_into() {
    let n = &[
        0x00, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB,
        0xCD,
    ];
    let x = "ABI0VniQq83vEjRWeJCrzQ";
    assert_eq!(RFC4648_NOPAD.encode(n), x);

    let mut b = Vec::<u8>::with_capacity(22);
    RFC4648_NOPAD.encode_into(n, &mut b);
    assert_eq!(&b[0..22], x.as_bytes());
    assert_eq!(RFC4648_NOPAD.decode(&b[0..22]).unwrap(), n);

    let mut b = Vec::<u8>::with_capacity(23);
    b.push(201);
    RFC4648_NOPAD.encode_into(n, &mut b);
    b.push(202);
    assert_eq!(String::from_utf8(b[1..23].to_vec()).unwrap(), x);
    assert_eq!(&b[1..23], x.as_bytes());
    assert_eq!(RFC4648_NOPAD.decode(&b[1..23]).unwrap(), n);

    let mut b = Vec::<u8>::with_capacity(17);
    b.push(201);
    RFC4648_NOPAD.decode_into(x.as_bytes(), &mut b).unwrap();
    b.push(202);
    assert_eq!(&b[1..17], n);
}

#[test]
#[should_panic(expected = "Missing capacity for encoding")]
fn panic_bytes_into() {
    let n = &[0x00, 0x12];
    let mut b = Vec::<u8>::with_capacity(3);
    RFC4648_NOPAD.encode_into(n, &mut b);
    assert_eq!(&b, b"ABI");
    let mut b = Vec::<u8>::with_capacity(2);
    RFC4648_NOPAD.encode_into(n, &mut b);
}

#[test]
#[should_panic(expected = "Missing capacity for padding")]
fn panic_pad() {
    let n = &[0x00, 0x12];
    let mut b = Vec::<u8>::with_capacity(4);
    RFC4648.encode_into(n, &mut b);
    assert_eq!(&b, b"ABI=");
    let mut b = Vec::<u8>::with_capacity(3);
    RFC4648.encode_into(n, &mut b);
}

#[test]
#[should_panic(expected = "Missing capacity for encoding")]
fn panic_encode_u64_into() {
    let n = 111;
    let mut b = Vec::<u8>::with_capacity(2);
    RFC4648_NOPAD.encode_u64_into(n, &mut b);
    assert_eq!(&b, b"Bv");
    let mut b = Vec::<u8>::with_capacity(1);
    RFC4648_NOPAD.encode_u64_into(n, &mut b);
}

#[test]
#[should_panic(expected = "Missing capacity for encoding")]
fn panic_encode_u128_into() {
    let n = 111;
    let mut b = Vec::<u8>::with_capacity(2);
    RFC4648_NOPAD.encode_u128_into(n, &mut b);
    assert_eq!(&b, b"Bv");
    let mut b = Vec::<u8>::with_capacity(1);
    RFC4648_NOPAD.encode_u128_into(n, &mut b);
}

// cargo test tests::compare_bytes_u128 -- --exact
#[test]
fn compare_bytes_u128() {
    let n = u128::from_be_bytes([
        0x00, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB,
        0xCD,
    ]);
    let x = "EjRWeJCrze8SNFZ4kKvN";
    let e = RFC4648_NOPAD.encode_u128(n);
    assert_eq!(e, x);
    let d = RFC4648_NOPAD.decode_u128(e.as_bytes()).unwrap();
    assert_eq!(d, n);
    let eb = RFC4648_NOPAD.encode(&[
        0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD,
    ]);
    assert_eq!(eb, e);
    let db = RFC4648_NOPAD.decode(eb.as_bytes()).unwrap();
    assert_eq!(db, d.to_be_bytes()[1..]);
    // TODO: find a way to convert a not 12 bit matchup such that both will give the same outcomes
}

#[test]
fn both_u64_low() {
    let enc = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    for i in 0..enc.len() {
        let s = RFC4648_NOPAD.encode_u64(i as u64);
        // println!("{} vs {} vs {} vs {}", s, i, enc[i], enc[i] as char);
        assert_eq!(s, (enc[i] as char).to_string());
        assert_eq!(i as u64, RFC4648_NOPAD.decode_u64(s.as_bytes()).unwrap());
    }
}

#[test]
fn decode_bad() {
    let res = RFC4648_NOPAD.decode_u64(b"1^_^");
    assert_eq!(
        res.unwrap_err(),
        DecodeError::InvalidChar {
            char: '^',
            index: 1
        }
    );
    let res = RFC4648_NOPAD.decode_u64(b"0123456789AB");
    assert_eq!(res.unwrap_err(), DecodeError::InvalidLength { length: 12 });
    let res = RFC4648_NOPAD.decode_u128(b"1^_^");
    assert_eq!(
        res.unwrap_err(),
        DecodeError::InvalidChar {
            char: '^',
            index: 1
        }
    );
    let res = RFC4648_NOPAD.decode_u128(b"0123456789ABCD012345678");
    assert_eq!(res.unwrap_err(), DecodeError::InvalidLength { length: 23 });
    let res = RFC4648_NOPAD.decode(b"1");
    assert_eq!(res.unwrap_err(), DecodeError::InvalidLength { length: 1 });
    let res = RFC4648_NOPAD.decode(b"11");
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
    assert_eq!(RFC4648_NOPAD.encode_str(""), "");
    assert_eq!(RFC4648_NOPAD.encode_str("f"), "Zg");
    assert_eq!(RFC4648_NOPAD.encode_str("fo"), "Zm8");
    assert_eq!(RFC4648_NOPAD.encode_str("foo"), "Zm9v");
    assert_eq!(RFC4648_NOPAD.encode_str("foob"), "Zm9vYg");
    assert_eq!(RFC4648_NOPAD.encode_str("fooba"), "Zm9vYmE");
    assert_eq!(RFC4648_NOPAD.encode_str("foobar"), "Zm9vYmFy");

    assert_eq!(RFC4648_NOPAD.decode_str(""), Ok(b"".to_vec()));
    assert_eq!(RFC4648_NOPAD.decode_str("Zg"), Ok(b"f".to_vec()));
    assert_eq!(RFC4648_NOPAD.decode_str("Zm8"), Ok(b"fo".to_vec()));
    assert_eq!(RFC4648_NOPAD.decode_str("Zm9v"), Ok(b"foo".to_vec()));
    assert_eq!(RFC4648_NOPAD.decode_str("Zm9vYg"), Ok(b"foob".to_vec()));
    assert_eq!(RFC4648_NOPAD.decode_str("Zm9vYmE"), Ok(b"fooba".to_vec()));
    assert_eq!(RFC4648_NOPAD.decode_str("Zm9vYmFy"), Ok(b"foobar".to_vec()));

    assert_eq!(RFC4648.encode_str(""), "");
    assert_eq!(RFC4648.encode_str("f"), "Zg==");
    assert_eq!(RFC4648.encode_str("fo"), "Zm8=");
    assert_eq!(RFC4648.encode_str("foo"), "Zm9v");
    assert_eq!(RFC4648.encode_str("foob"), "Zm9vYg==");
    assert_eq!(RFC4648.encode_str("fooba"), "Zm9vYmE=");
    assert_eq!(RFC4648.encode_str("foobar"), "Zm9vYmFy");

    assert_eq!(RFC4648.decode_str(""), Ok(b"".to_vec()));
    assert_eq!(RFC4648.decode_str("Zg=="), Ok(b"f".to_vec()));
    assert_eq!(RFC4648.decode_str("Zm8="), Ok(b"fo".to_vec()));
    assert_eq!(RFC4648.decode_str("Zm9v"), Ok(b"foo".to_vec()));
    assert_eq!(RFC4648.decode_str("Zm9vYg=="), Ok(b"foob".to_vec()));
    assert_eq!(RFC4648.decode_str("Zm9vYmE="), Ok(b"fooba".to_vec()));
    assert_eq!(RFC4648.decode_str("Zm9vYmFy"), Ok(b"foobar".to_vec()));

    assert_eq!(RFC4648.encode(b""), "");
    assert_eq!(RFC4648.encode(b"f"), "Zg==");
    assert_eq!(RFC4648.encode(b"fo"), "Zm8=");
    assert_eq!(RFC4648.encode(b"foo"), "Zm9v");
    assert_eq!(RFC4648.encode(b"foob"), "Zm9vYg==");
    assert_eq!(RFC4648.encode(b"fooba"), "Zm9vYmE=");
    assert_eq!(RFC4648.encode(b"foobar"), "Zm9vYmFy");

    assert_eq!(RFC4648.decode(b""), Ok(b"".to_vec()));
    assert_eq!(RFC4648.decode(b"Zg=="), Ok(b"f".to_vec()));
    assert_eq!(RFC4648.decode(b"Zm8="), Ok(b"fo".to_vec()));
    assert_eq!(RFC4648.decode(b"Zm9v"), Ok(b"foo".to_vec()));
    assert_eq!(RFC4648.decode(b"Zm9vYg=="), Ok(b"foob".to_vec()));
    assert_eq!(RFC4648.decode(b"Zm9vYmE="), Ok(b"fooba".to_vec()));
    assert_eq!(RFC4648.decode(b"Zm9vYmFy"), Ok(b"foobar".to_vec()));
}

#[test]
fn both_edges_u128() {
    let rs = [
        (u128::MAX - 100000)..=u128::MAX,
        ((1 << 126) - 10000)..=((1 << 126) + 10000),
        ((1 << 122) - 10000)..=((1 << 122) + 10000),
        ((1 << 114) - 10000)..=((1 << 114) + 10000),
        ((1 << 108) - 10000)..=((1 << 108) + 10000),
        ((1 << 102) - 10000)..=((1 << 102) + 10000),
        ((1 << 96) - 10000)..=((1 << 96) + 10000),
        ((1 << 90) - 10000)..=((1 << 90) + 10000),
        ((1 << 84) - 10000)..=((1 << 84) + 10000),
        ((1 << 78) - 10000)..=((1 << 78) + 10000),
        ((1 << 72) - 10000)..=((1 << 72) + 10000),
        ((1 << 66) - 10000)..=((1 << 66) + 10000),
        ((1 << 60) - 10000)..=((1 << 60) + 10000),
        ((1 << 54) - 10000)..=((1 << 54) + 10000),
        ((1 << 48) - 10000)..=((1 << 48) + 10000),
        ((1 << 42) - 10000)..=((1 << 42) + 10000),
        ((1 << 36) - 10000)..=((1 << 36) + 10000),
        ((1 << 30) - 10000)..=((1 << 30) + 10000),
        ((1 << 24) - 10000)..=((1 << 24) + 10000),
        ((1 << 18) - 10000)..=((1 << 18) + 10000),
        0..=((1 << 16) + 10000), // first 4 chars
    ];

    for r in rs {
        for n in r {
            let e = crate::RFC4648_NOPAD.encode_u128(n);
            let d = crate::RFC4648_NOPAD.decode_u128(e.as_bytes()).unwrap();
            assert_eq!(n, d, "mismatch decode for {n}: {e} vs {d}");
        }
    }
}

#[test]
fn both_u128_ones() {
    // first is: Z, F, 7, 3, 1
    // and fill remainder w Z
    let x = [
        "D/////////////////////",
        "B/////////////////////",
        "/////////////////////",
        "f////////////////////",
        "P////////////////////",
        "H////////////////////",
        "D////////////////////",
        "B////////////////////",
        "////////////////////",
        "f///////////////////",
        "P///////////////////",
        "H///////////////////",
        "D///////////////////",
        "B///////////////////",
        "///////////////////",
        "f//////////////////",
        "P//////////////////",
        "H//////////////////",
        "D//////////////////",
        "B//////////////////",
        "//////////////////",
        "f/////////////////",
        "P/////////////////",
        "H/////////////////",
        "D/////////////////",
        "B/////////////////",
        "/////////////////",
        "f////////////////",
        "P////////////////",
        "H////////////////",
        "D////////////////",
        "B////////////////",
        "////////////////",
        "f///////////////",
        "P///////////////",
        "H///////////////",
        "D///////////////",
        "B///////////////",
        "///////////////",
        "f//////////////",
        "P//////////////",
        "H//////////////",
        "D//////////////",
        "B//////////////",
        "//////////////",
        "f/////////////",
        "P/////////////",
        "H/////////////",
        "D/////////////",
        "B/////////////",
        "/////////////",
        "f////////////",
        "P////////////",
        "H////////////",
        "D////////////",
        "B////////////",
        "////////////",
        "f///////////",
        "P///////////",
        "H///////////",
        "D///////////",
        "B///////////",
        "///////////",
        "f//////////",
        "P//////////",
        "H//////////",
        "D//////////",
        "B//////////",
        "//////////",
        "f/////////",
        "P/////////",
        "H/////////",
        "D/////////",
        "B/////////",
        "/////////",
        "f////////",
        "P////////",
        "H////////",
        "D////////",
        "B////////",
        "////////",
        "f///////",
        "P///////",
        "H///////",
        "D///////",
        "B///////",
        "///////",
        "f//////",
        "P//////",
        "H//////",
        "D//////",
        "B//////",
        "//////",
        "f/////",
        "P/////",
        "H/////",
        "D/////",
        "B/////",
        "/////",
        "f////",
        "P////",
        "H////",
        "D////",
        "B////",
        "////",
        "f///",
        "P///",
        "H///",
        "D///",
        "B///",
        "///",
        "f//",
        "P//",
        "H//",
        "D//",
        "B//",
        "//",
        "f/",
        "P/",
        "H/",
        "D/",
        "B/",
        "/",
        "f",
        "P",
        "H",
        "D",
        "B",
    ];
    for i in 0..u128::BITS as usize {
        let n = u128::MAX >> i;
        let s = RFC4648_NOPAD.encode_u128(n);
        assert_eq!(s, x[i], "mismatch ones encode at {i}: {s} vs {}", x[i]);
        let d = RFC4648_NOPAD.decode_u128(s.as_bytes()).unwrap();
        assert_eq!(d, n, "mismatch ones decode at {i}: {d} vs {n}");
    }
}

#[test]
fn both_u64_ones() {
    // first is: Z, F, 7, 3, 1
    // and fill remainder w Z
    let x = [
        "P//////////",
        "H//////////",
        "D//////////",
        "B//////////",
        "//////////",
        "f/////////",
        "P/////////",
        "H/////////",
        "D/////////",
        "B/////////",
        "/////////",
        "f////////",
        "P////////",
        "H////////",
        "D////////",
        "B////////",
        "////////",
        "f///////",
        "P///////",
        "H///////",
        "D///////",
        "B///////",
        "///////",
        "f//////",
        "P//////",
        "H//////",
        "D//////",
        "B//////",
        "//////",
        "f/////",
        "P/////",
        "H/////",
        "D/////",
        "B/////",
        "/////",
        "f////",
        "P////",
        "H////",
        "D////",
        "B////",
        "////",
        "f///",
        "P///",
        "H///",
        "D///",
        "B///",
        "///",
        "f//",
        "P//",
        "H//",
        "D//",
        "B//",
        "//",
        "f/",
        "P/",
        "H/",
        "D/",
        "B/",
        "/",
        "f",
        "P",
        "H",
        "D",
        "B",
    ];
    for i in 0..u64::BITS as usize {
        let n = u64::MAX >> i;
        let s = RFC4648_NOPAD.encode_u64(n);
        assert_eq!(s, x[i], "mismatch ones encode at {i}: {s} vs {}", x[i]);
        let d = RFC4648_NOPAD.decode_u64(s.as_bytes()).unwrap();
        assert_eq!(d, n, "mismatch ones decode at {i}: {d} vs {n}");
    }
}
