use crate::shared::{bits_or_err_u64, DecodeError};

use super::alphabet::{WIDTH_1, WIDTH_2, WIDTH_3, WIDTH_4, WIDTH_5, WIDTH_6, WIDTH_7, WIDTH_8, WIDTH_9, WIDTH_10};

// _str version is basically identical perf to byte array,
// maybe 3-5% slower, beyond noise threshold, but not surprising
// modify comparisons like so:
//let a = "(\S+)";
//let a = "$1".to_owned();
//fast32::decode_([^(]+)\(black_box\(a
//fast32::decode_$1_str(black_box(&a
pub fn decode_u64_str(dec: &'static [u8; 256], a: impl AsRef<str>) -> Result<u64, DecodeError> {
    decode_u64(dec, a.as_ref().as_bytes())
}

pub fn decode_u64(dec: &'static [u8; 256], a: &[u8]) -> Result<u64, DecodeError> {
    #[rustfmt::skip]
    let n = match a.len() {
        11 => {
            bits_or_err_u64(dec, a,  0)? << WIDTH_10 |
            bits_or_err_u64(dec, a,  1)? << WIDTH_9 |
            bits_or_err_u64(dec, a,  2)? << WIDTH_8 |
            bits_or_err_u64(dec, a,  3)? << WIDTH_7 |
            bits_or_err_u64(dec, a,  4)? << WIDTH_6 |
            bits_or_err_u64(dec, a,  5)? << WIDTH_5 |
            bits_or_err_u64(dec, a,  6)? << WIDTH_4 |
            bits_or_err_u64(dec, a,  7)? << WIDTH_3 |
            bits_or_err_u64(dec, a,  8)? << WIDTH_2 |
            bits_or_err_u64(dec, a,  9)? << WIDTH_1 |
            bits_or_err_u64(dec, a, 10)?
        }
        10 => {
            bits_or_err_u64(dec, a,  0)? << WIDTH_9 |
            bits_or_err_u64(dec, a,  1)? << WIDTH_8 |
            bits_or_err_u64(dec, a,  2)? << WIDTH_7 |
            bits_or_err_u64(dec, a,  3)? << WIDTH_6 |
            bits_or_err_u64(dec, a,  4)? << WIDTH_5 |
            bits_or_err_u64(dec, a,  5)? << WIDTH_4 |
            bits_or_err_u64(dec, a,  6)? << WIDTH_3 |
            bits_or_err_u64(dec, a,  7)? << WIDTH_2 |
            bits_or_err_u64(dec, a,  8)? << WIDTH_1 |
            bits_or_err_u64(dec, a,  9)?
        }
        9 => {
            bits_or_err_u64(dec, a,  0)? << WIDTH_8 |
            bits_or_err_u64(dec, a,  1)? << WIDTH_7 |
            bits_or_err_u64(dec, a,  2)? << WIDTH_6 |
            bits_or_err_u64(dec, a,  3)? << WIDTH_5 |
            bits_or_err_u64(dec, a,  4)? << WIDTH_4 |
            bits_or_err_u64(dec, a,  5)? << WIDTH_3 |
            bits_or_err_u64(dec, a,  6)? << WIDTH_2 |
            bits_or_err_u64(dec, a,  7)? << WIDTH_1 |
            bits_or_err_u64(dec, a,  8)?
        }
        8 => {
            bits_or_err_u64(dec, a,  0)? << WIDTH_7 |
            bits_or_err_u64(dec, a,  1)? << WIDTH_6 |
            bits_or_err_u64(dec, a,  2)? << WIDTH_5 |
            bits_or_err_u64(dec, a,  3)? << WIDTH_4 |
            bits_or_err_u64(dec, a,  4)? << WIDTH_3 |
            bits_or_err_u64(dec, a,  5)? << WIDTH_2 |
            bits_or_err_u64(dec, a,  6)? << WIDTH_1 |
            bits_or_err_u64(dec, a,  7)?
        }
        7 => {
            bits_or_err_u64(dec, a,  0)? << WIDTH_6 |
            bits_or_err_u64(dec, a,  1)? << WIDTH_5 |
            bits_or_err_u64(dec, a,  2)? << WIDTH_4 |
            bits_or_err_u64(dec, a,  3)? << WIDTH_3 |
            bits_or_err_u64(dec, a,  4)? << WIDTH_2 |
            bits_or_err_u64(dec, a,  5)? << WIDTH_1 |
            bits_or_err_u64(dec, a,  6)?
        }
        6 => {
            bits_or_err_u64(dec, a,  0)? << WIDTH_5 |
            bits_or_err_u64(dec, a,  1)? << WIDTH_4 |
            bits_or_err_u64(dec, a,  2)? << WIDTH_3 |
            bits_or_err_u64(dec, a,  3)? << WIDTH_2 |
            bits_or_err_u64(dec, a,  4)? << WIDTH_1 |
            bits_or_err_u64(dec, a,  5)?
        }
        5 => {
            bits_or_err_u64(dec, a,  0)? << WIDTH_4 |
            bits_or_err_u64(dec, a,  1)? << WIDTH_3 |
            bits_or_err_u64(dec, a,  2)? << WIDTH_2 |
            bits_or_err_u64(dec, a,  3)? << WIDTH_1 |
            bits_or_err_u64(dec, a,  4)?
        }
        4 => {
            bits_or_err_u64(dec, a,  0)? << WIDTH_3 |
            bits_or_err_u64(dec, a,  1)? << WIDTH_2 |
            bits_or_err_u64(dec, a,  2)? << WIDTH_1 |
            bits_or_err_u64(dec, a,  3)?
        }
        3 => {
            bits_or_err_u64(dec, a,  0)? << WIDTH_2 |
            bits_or_err_u64(dec, a,  1)? << WIDTH_1 |
            bits_or_err_u64(dec, a,  2)?
        }
        2 => {
            bits_or_err_u64(dec, a,  0)? << WIDTH_1 |
            bits_or_err_u64(dec, a,  1)?
        }
        1 => {
            bits_or_err_u64(dec, a,  0)?
        }
        0 => 0,
        len @ _ => Err(DecodeError::InvalidLength { length: len })?,
    };
    Ok(n)
}
