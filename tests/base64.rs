use fast32::base64::*;

#[ignore]
#[test]
fn rfc4648() {
    // https://datatracker.ietf.org/doc/html/rfc4648#section-10
    assert_eq!(RFC4648_NOPAD.encode_bytes_str(""), "");
    assert_eq!(RFC4648_NOPAD.encode_bytes_str("f"), "Zg");
    assert_eq!(RFC4648_NOPAD.encode_bytes_str("fo"), "Zm8");
    assert_eq!(RFC4648_NOPAD.encode_bytes_str("foo"), "Zm9v");
    assert_eq!(RFC4648_NOPAD.encode_bytes_str("foob"), "Zm9vYg");
    assert_eq!(RFC4648_NOPAD.encode_bytes_str("fooba"), "Zm9vYmE");
    assert_eq!(RFC4648_NOPAD.encode_bytes_str("foobar"), "Zm9vYmFy");

    assert_eq!(RFC4648_NOPAD.decode_bytes_str(""), Ok(b"".to_vec()));
    assert_eq!(RFC4648_NOPAD.decode_bytes_str("Zg"), Ok(b"f".to_vec()));
    assert_eq!(RFC4648_NOPAD.decode_bytes_str("Zm8"), Ok(b"fo".to_vec()));
    assert_eq!(RFC4648_NOPAD.decode_bytes_str("Zm9v"), Ok(b"foo".to_vec()));
    assert_eq!(RFC4648_NOPAD.decode_bytes_str("Zm9vYg"), Ok(b"foob".to_vec()));
    assert_eq!(RFC4648_NOPAD.decode_bytes_str("Zm9vYmE"), Ok(b"fooba".to_vec()));
    assert_eq!(RFC4648_NOPAD.decode_bytes_str("Zm9vYmFy"), Ok(b"foobar".to_vec()));

    assert_eq!(RFC4648.encode_bytes_str(""), "");
    assert_eq!(RFC4648.encode_bytes_str("f"), "Zg==");
    assert_eq!(RFC4648.encode_bytes_str("fo"), "Zm8=");
    assert_eq!(RFC4648.encode_bytes_str("foo"), "Zm9v");
    assert_eq!(RFC4648.encode_bytes_str("foob"), "Zm9vYg==");
    assert_eq!(RFC4648.encode_bytes_str("fooba"), "Zm9vYmE=");
    assert_eq!(RFC4648.encode_bytes_str("foobar"), "Zm9vYmFy");

    assert_eq!(RFC4648.decode_bytes_str(""), Ok(b"".to_vec()));
    assert_eq!(RFC4648.decode_bytes_str("Zg=="), Ok(b"f".to_vec()));
    assert_eq!(RFC4648.decode_bytes_str("Zm8="), Ok(b"fo".to_vec()));
    assert_eq!(RFC4648.decode_bytes_str("Zm9v"), Ok(b"foo".to_vec()));
    assert_eq!(RFC4648.decode_bytes_str("Zm9vYg=="), Ok(b"foob".to_vec()));
    assert_eq!(RFC4648.decode_bytes_str("Zm9vYmE="), Ok(b"fooba".to_vec()));
    assert_eq!(RFC4648.decode_bytes_str("Zm9vYmFy"), Ok(b"foobar".to_vec()));

    assert_eq!(RFC4648.encode_bytes(b""), "");
    assert_eq!(RFC4648.encode_bytes(b"f"), "Zg==");
    assert_eq!(RFC4648.encode_bytes(b"fo"), "Zm8=");
    assert_eq!(RFC4648.encode_bytes(b"foo"), "Zm9v");
    assert_eq!(RFC4648.encode_bytes(b"foob"), "Zm9vYg==");
    assert_eq!(RFC4648.encode_bytes(b"fooba"), "Zm9vYmE=");
    assert_eq!(RFC4648.encode_bytes(b"foobar"), "Zm9vYmFy");

    assert_eq!(RFC4648.decode_bytes(b""), Ok(b"".to_vec()));
    assert_eq!(RFC4648.decode_bytes(b"Zg=="), Ok(b"f".to_vec()));
    assert_eq!(RFC4648.decode_bytes(b"Zm8="), Ok(b"fo".to_vec()));
    assert_eq!(RFC4648.decode_bytes(b"Zm9v"), Ok(b"foo".to_vec()));
    assert_eq!(RFC4648.decode_bytes(b"Zm9vYg=="), Ok(b"foob".to_vec()));
    assert_eq!(RFC4648.decode_bytes(b"Zm9vYmE="), Ok(b"fooba".to_vec()));
    assert_eq!(RFC4648.decode_bytes(b"Zm9vYmFy"), Ok(b"foobar".to_vec()));
}
