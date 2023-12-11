use uuid::Uuid;

use crate::DecodeError;

use super::alphabet::BITS;

/// Decode byte array with given decoding, into a [`Uuid`]
///
/// Examples:
/// ```
/// use uuid::Uuid;
/// use fast32::base32::CROCKFORD;
/// assert_eq!(CROCKFORD.decode_uuid(b"7ZZZZZZZZZZZZZZZZZZZZZZZZZ").unwrap(), Uuid::max());
/// ```
///
/// Returns [`DecodeError`] if input to decode is invalid
pub fn decode_uuid(dec: &'static [u8; 256], a: &[u8]) -> Result<Uuid, DecodeError> {
    Ok(Uuid::from_u128(super::decode_u128(dec, a)?))
}

/// Encode [`Uuid`] with given encoding, into a `String`
///
/// Examples:
/// ```
/// use uuid::Uuid;
/// use fast32::base32::CROCKFORD;
/// assert_eq!(CROCKFORD.encode_uuid(Uuid::max()), "7ZZZZZZZZZZZZZZZZZZZZZZZZZ");
/// ```
pub fn encode_uuid(enc: &'static [u8; BITS], n: Uuid) -> String {
    super::encode_u128(enc, n.as_u128())
}


/// Encode [`Uuid`] with given encoding, into an existing `Vec<u8>`
///
/// Example:
/// ```
/// use uuid::Uuid;
/// use fast32::base32::CROCKFORD;
/// let mut b = Vec::<u8>::with_capacity(26);
/// CROCKFORD.encode_uuid_into(Uuid::max(), &mut b);
/// assert_eq!(&b, b"7ZZZZZZZZZZZZZZZZZZZZZZZZZ");
/// ```
///
/// Panics if not enough capacity in `b` for encoding -- see [`capacity_encode_u128`](super::capacity_encode_u128())
pub fn encode_uuid_into(enc: &'static [u8; BITS], n: Uuid, b: &mut Vec<u8>) {
    super::encode_u128_into(enc, n.as_u128(), b)
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
