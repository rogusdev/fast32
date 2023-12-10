use uuid::Uuid;

use crate::DecodeError;

use super::alphabet::BITS;

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
    let x = "EjRWeJCrze8SNFZ4kKvN";
    let e = super::RFC4648.encode_u128(n);
    assert_eq!(e, x);
    let e = super::RFC4648.encode_uuid(u);
    assert_eq!(e, x);
    let d = super::RFC4648.decode_u128(e.as_bytes()).unwrap();
    assert_eq!(d, n);
    let d = super::RFC4648.decode_uuid(e.as_bytes()).unwrap();
    assert_eq!(d, u);
}
