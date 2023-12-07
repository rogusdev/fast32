use uuid::Uuid;

use crate::DecodeError;

use super::alphabet::BITS;

pub fn decode_uuid_str(dec: &'static [u8; 256], a: impl AsRef<str>) -> Result<Uuid, DecodeError> {
    decode_uuid(dec, a.as_ref().as_bytes())
}

pub fn decode_uuid(dec: &'static [u8; 256], a: &[u8]) -> Result<Uuid, DecodeError> {
    Ok(Uuid::from_u128(super::decode_u128(dec, a)?))
}

pub fn encode_uuid(enc: &'static [u8; BITS], n: Uuid) -> String {
    super::encode_u128(enc, n.as_u128())
}

#[test]
fn compare_uuid() {
    let b = [
        0x00, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB,
        0xCD,
    ];
    let u = Uuid::from_bytes(b);
    let n = u128::from_be_bytes(b);
    let x = "28T5CY4GNF6YY4HMASW91AYD";
    let e = super::CROCKFORD.encode_u128(n);
    assert_eq!(e, x);
    let e = super::CROCKFORD.encode_uuid(u);
    assert_eq!(e, x);
    let d = super::CROCKFORD.decode_u128(e.as_bytes()).unwrap();
    assert_eq!(d, n);
    let d = super::CROCKFORD.decode_uuid(e.as_bytes()).unwrap();
    assert_eq!(d, u);
}
