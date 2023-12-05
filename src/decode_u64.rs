use crate::alphabet::DEC_CROCKFORD_UPPER;
use crate::decode_base::{bits_or_err_u64, DecodeError};

#[inline]
pub fn decode_u64(a: impl AsRef<str>) -> Result<u64, DecodeError> {
    let a = a.as_ref().as_bytes();
    let dec = &DEC_CROCKFORD_UPPER;
    #[rustfmt::skip]
    let n = match a.len() {
        13 => {
            bits_or_err_u64(dec, a,  0)? << 60 |
            bits_or_err_u64(dec, a,  1)? << 55 |
            bits_or_err_u64(dec, a,  2)? << 50 |
            bits_or_err_u64(dec, a,  3)? << 45 |
            bits_or_err_u64(dec, a,  4)? << 40 |
            bits_or_err_u64(dec, a,  5)? << 35 |
            bits_or_err_u64(dec, a,  6)? << 30 |
            bits_or_err_u64(dec, a,  7)? << 25 |
            bits_or_err_u64(dec, a,  8)? << 20 |
            bits_or_err_u64(dec, a,  9)? << 15 |
            bits_or_err_u64(dec, a, 10)? << 10 |
            bits_or_err_u64(dec, a, 11)? <<  5 |
            bits_or_err_u64(dec, a, 12)?
        }
        12 => {
            bits_or_err_u64(dec, a,  0)? << 55 |
            bits_or_err_u64(dec, a,  1)? << 50 |
            bits_or_err_u64(dec, a,  2)? << 45 |
            bits_or_err_u64(dec, a,  3)? << 40 |
            bits_or_err_u64(dec, a,  4)? << 35 |
            bits_or_err_u64(dec, a,  5)? << 30 |
            bits_or_err_u64(dec, a,  6)? << 25 |
            bits_or_err_u64(dec, a,  7)? << 20 |
            bits_or_err_u64(dec, a,  8)? << 15 |
            bits_or_err_u64(dec, a,  9)? << 10 |
            bits_or_err_u64(dec, a, 10)? <<  5 |
            bits_or_err_u64(dec, a, 11)?
        }
        11 => {
            bits_or_err_u64(dec, a,  0)? << 50 |
            bits_or_err_u64(dec, a,  1)? << 45 |
            bits_or_err_u64(dec, a,  2)? << 40 |
            bits_or_err_u64(dec, a,  3)? << 35 |
            bits_or_err_u64(dec, a,  4)? << 30 |
            bits_or_err_u64(dec, a,  5)? << 25 |
            bits_or_err_u64(dec, a,  6)? << 20 |
            bits_or_err_u64(dec, a,  7)? << 15 |
            bits_or_err_u64(dec, a,  8)? << 10 |
            bits_or_err_u64(dec, a,  9)? <<  5 |
            bits_or_err_u64(dec, a, 10)?
        }
        10 => {
            bits_or_err_u64(dec, a,  0)? << 45 |
            bits_or_err_u64(dec, a,  1)? << 40 |
            bits_or_err_u64(dec, a,  2)? << 35 |
            bits_or_err_u64(dec, a,  3)? << 30 |
            bits_or_err_u64(dec, a,  4)? << 25 |
            bits_or_err_u64(dec, a,  5)? << 20 |
            bits_or_err_u64(dec, a,  6)? << 15 |
            bits_or_err_u64(dec, a,  7)? << 10 |
            bits_or_err_u64(dec, a,  8)? <<  5 |
            bits_or_err_u64(dec, a,  9)?
        }
        9 => {
            bits_or_err_u64(dec, a,  0)? << 40 |
            bits_or_err_u64(dec, a,  1)? << 35 |
            bits_or_err_u64(dec, a,  2)? << 30 |
            bits_or_err_u64(dec, a,  3)? << 25 |
            bits_or_err_u64(dec, a,  4)? << 20 |
            bits_or_err_u64(dec, a,  5)? << 15 |
            bits_or_err_u64(dec, a,  6)? << 10 |
            bits_or_err_u64(dec, a,  7)? <<  5 |
            bits_or_err_u64(dec, a,  8)?
        }
        8 => {
            bits_or_err_u64(dec, a,  0)? << 35 |
            bits_or_err_u64(dec, a,  1)? << 30 |
            bits_or_err_u64(dec, a,  2)? << 25 |
            bits_or_err_u64(dec, a,  3)? << 20 |
            bits_or_err_u64(dec, a,  4)? << 15 |
            bits_or_err_u64(dec, a,  5)? << 10 |
            bits_or_err_u64(dec, a,  6)? <<  5 |
            bits_or_err_u64(dec, a,  7)?
        }
        7 => {
            bits_or_err_u64(dec, a,  0)? << 30 |
            bits_or_err_u64(dec, a,  1)? << 25 |
            bits_or_err_u64(dec, a,  2)? << 20 |
            bits_or_err_u64(dec, a,  3)? << 15 |
            bits_or_err_u64(dec, a,  4)? << 10 |
            bits_or_err_u64(dec, a,  5)? <<  5 |
            bits_or_err_u64(dec, a,  6)?
        }
        6 => {
            bits_or_err_u64(dec, a,  0)? << 25 |
            bits_or_err_u64(dec, a,  1)? << 20 |
            bits_or_err_u64(dec, a,  2)? << 15 |
            bits_or_err_u64(dec, a,  3)? << 10 |
            bits_or_err_u64(dec, a,  4)? <<  5 |
            bits_or_err_u64(dec, a,  5)?
        }
        5 => {
            bits_or_err_u64(dec, a,  0)? << 20 |
            bits_or_err_u64(dec, a,  1)? << 15 |
            bits_or_err_u64(dec, a,  2)? << 10 |
            bits_or_err_u64(dec, a,  3)? <<  5 |
            bits_or_err_u64(dec, a,  4)?
        }
        4 => {
            bits_or_err_u64(dec, a,  0)? << 15 |
            bits_or_err_u64(dec, a,  1)? << 10 |
            bits_or_err_u64(dec, a,  2)? <<  5 |
            bits_or_err_u64(dec, a,  3)?
        }
        3 => {
            bits_or_err_u64(dec, a,  0)? << 10 |
            bits_or_err_u64(dec, a,  1)? <<  5 |
            bits_or_err_u64(dec, a,  2)?
        }
        2 => {
            bits_or_err_u64(dec, a,  0)? <<  5 |
            bits_or_err_u64(dec, a,  1)?
        }
        1 => {
            #[allow(unused_parens)]
            bits_or_err_u64(dec, a,  0)?
        }
        0 => 0,
        len @ _ => Err(DecodeError::InvalidLength { length: len })?,
    };
    Ok(n)
}
