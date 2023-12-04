use crate::alphabet::DEC_CROCKFORD_UPPER;
use crate::decode_base::{bits_or_err, DecodeError};

#[inline]
pub fn decode_bytes(a: impl AsRef<str>) -> Result<Vec<u8>, DecodeError> {
    let a = a.as_ref().as_bytes();
    let dec = &DEC_CROCKFORD_UPPER;

    Err(DecodeError::InvalidChar { char: '!', index: 0 })
}
