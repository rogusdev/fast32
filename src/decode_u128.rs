use crate::alphabet::DEC_CROCKFORD_UPPER;
use crate::decode_base::{bits_or_err_u128, DecodeError};

// _str version is basically identical perf to byte array,
// maybe 3-5% slower, beyond noise threshold, but not surprising
// modify comparisons like so:
//let a = "(\S+)";
//let a = "$1".to_owned();
//fast32::decode_([^(]+)\(black_box\(a
//fast32::decode_$1_str(black_box(&a
#[inline]
pub fn decode_u128_str(a: impl AsRef<str>) -> Result<u128, DecodeError> {
    decode_u128(a.as_ref().as_bytes())
}

#[inline]
pub fn decode_u128(a: &[u8]) -> Result<u128, DecodeError> {
    let dec = &DEC_CROCKFORD_UPPER;
    #[rustfmt::skip]
    let n = match a.len() {
        26 => {
            bits_or_err_u128(dec, a,  0)? << 125 |
            bits_or_err_u128(dec, a,  1)? << 120 |
            bits_or_err_u128(dec, a,  2)? << 115 |
            bits_or_err_u128(dec, a,  3)? << 110 |
            bits_or_err_u128(dec, a,  4)? << 105 |
            bits_or_err_u128(dec, a,  5)? << 100 |
            bits_or_err_u128(dec, a,  6)? << 95 |
            bits_or_err_u128(dec, a,  7)? << 90 |
            bits_or_err_u128(dec, a,  8)? << 85 |
            bits_or_err_u128(dec, a,  9)? << 80 |
            bits_or_err_u128(dec, a, 10)? << 75 |
            bits_or_err_u128(dec, a, 11)? << 70 |
            bits_or_err_u128(dec, a, 12)? << 65 |
            bits_or_err_u128(dec, a, 13)? << 60 |
            bits_or_err_u128(dec, a, 14)? << 55 |
            bits_or_err_u128(dec, a, 15)? << 50 |
            bits_or_err_u128(dec, a, 16)? << 45 |
            bits_or_err_u128(dec, a, 17)? << 40 |
            bits_or_err_u128(dec, a, 18)? << 35 |
            bits_or_err_u128(dec, a, 19)? << 30 |
            bits_or_err_u128(dec, a, 20)? << 25 |
            bits_or_err_u128(dec, a, 21)? << 20 |
            bits_or_err_u128(dec, a, 22)? << 15 |
            bits_or_err_u128(dec, a, 23)? << 10 |
            bits_or_err_u128(dec, a, 24)? <<  5 |
            bits_or_err_u128(dec, a, 25)?
        }
        25 => {
            bits_or_err_u128(dec, a,  0)? << 120 |
            bits_or_err_u128(dec, a,  1)? << 115 |
            bits_or_err_u128(dec, a,  2)? << 110 |
            bits_or_err_u128(dec, a,  3)? << 105 |
            bits_or_err_u128(dec, a,  4)? << 100 |
            bits_or_err_u128(dec, a,  5)? << 95 |
            bits_or_err_u128(dec, a,  6)? << 90 |
            bits_or_err_u128(dec, a,  7)? << 85 |
            bits_or_err_u128(dec, a,  8)? << 80 |
            bits_or_err_u128(dec, a,  9)? << 75 |
            bits_or_err_u128(dec, a, 10)? << 70 |
            bits_or_err_u128(dec, a, 11)? << 65 |
            bits_or_err_u128(dec, a, 12)? << 60 |
            bits_or_err_u128(dec, a, 13)? << 55 |
            bits_or_err_u128(dec, a, 14)? << 50 |
            bits_or_err_u128(dec, a, 15)? << 45 |
            bits_or_err_u128(dec, a, 16)? << 40 |
            bits_or_err_u128(dec, a, 17)? << 35 |
            bits_or_err_u128(dec, a, 18)? << 30 |
            bits_or_err_u128(dec, a, 19)? << 25 |
            bits_or_err_u128(dec, a, 20)? << 20 |
            bits_or_err_u128(dec, a, 21)? << 15 |
            bits_or_err_u128(dec, a, 22)? << 10 |
            bits_or_err_u128(dec, a, 23)? <<  5 |
            bits_or_err_u128(dec, a, 24)?
        }
        24 => {
            bits_or_err_u128(dec, a,  0)? << 115 |
            bits_or_err_u128(dec, a,  1)? << 110 |
            bits_or_err_u128(dec, a,  2)? << 105 |
            bits_or_err_u128(dec, a,  3)? << 100 |
            bits_or_err_u128(dec, a,  4)? << 95 |
            bits_or_err_u128(dec, a,  5)? << 90 |
            bits_or_err_u128(dec, a,  6)? << 85 |
            bits_or_err_u128(dec, a,  7)? << 80 |
            bits_or_err_u128(dec, a,  8)? << 75 |
            bits_or_err_u128(dec, a,  9)? << 70 |
            bits_or_err_u128(dec, a, 10)? << 65 |
            bits_or_err_u128(dec, a, 11)? << 60 |
            bits_or_err_u128(dec, a, 12)? << 55 |
            bits_or_err_u128(dec, a, 13)? << 50 |
            bits_or_err_u128(dec, a, 14)? << 45 |
            bits_or_err_u128(dec, a, 15)? << 40 |
            bits_or_err_u128(dec, a, 16)? << 35 |
            bits_or_err_u128(dec, a, 17)? << 30 |
            bits_or_err_u128(dec, a, 18)? << 25 |
            bits_or_err_u128(dec, a, 19)? << 20 |
            bits_or_err_u128(dec, a, 20)? << 15 |
            bits_or_err_u128(dec, a, 21)? << 10 |
            bits_or_err_u128(dec, a, 22)? <<  5 |
            bits_or_err_u128(dec, a, 23)?
        }
        23 => {
            bits_or_err_u128(dec, a,  0)? << 110 |
            bits_or_err_u128(dec, a,  1)? << 105 |
            bits_or_err_u128(dec, a,  2)? << 100 |
            bits_or_err_u128(dec, a,  3)? << 95 |
            bits_or_err_u128(dec, a,  4)? << 90 |
            bits_or_err_u128(dec, a,  5)? << 85 |
            bits_or_err_u128(dec, a,  6)? << 80 |
            bits_or_err_u128(dec, a,  7)? << 75 |
            bits_or_err_u128(dec, a,  8)? << 70 |
            bits_or_err_u128(dec, a,  9)? << 65 |
            bits_or_err_u128(dec, a, 10)? << 60 |
            bits_or_err_u128(dec, a, 11)? << 55 |
            bits_or_err_u128(dec, a, 12)? << 50 |
            bits_or_err_u128(dec, a, 13)? << 45 |
            bits_or_err_u128(dec, a, 14)? << 40 |
            bits_or_err_u128(dec, a, 15)? << 35 |
            bits_or_err_u128(dec, a, 16)? << 30 |
            bits_or_err_u128(dec, a, 17)? << 25 |
            bits_or_err_u128(dec, a, 18)? << 20 |
            bits_or_err_u128(dec, a, 19)? << 15 |
            bits_or_err_u128(dec, a, 20)? << 10 |
            bits_or_err_u128(dec, a, 21)? <<  5 |
            bits_or_err_u128(dec, a, 22)?
        }
        22 => {
            bits_or_err_u128(dec, a,  0)? << 105 |
            bits_or_err_u128(dec, a,  1)? << 100 |
            bits_or_err_u128(dec, a,  2)? << 95 |
            bits_or_err_u128(dec, a,  3)? << 90 |
            bits_or_err_u128(dec, a,  4)? << 85 |
            bits_or_err_u128(dec, a,  5)? << 80 |
            bits_or_err_u128(dec, a,  6)? << 75 |
            bits_or_err_u128(dec, a,  7)? << 70 |
            bits_or_err_u128(dec, a,  8)? << 65 |
            bits_or_err_u128(dec, a,  9)? << 60 |
            bits_or_err_u128(dec, a, 10)? << 55 |
            bits_or_err_u128(dec, a, 11)? << 50 |
            bits_or_err_u128(dec, a, 12)? << 45 |
            bits_or_err_u128(dec, a, 13)? << 40 |
            bits_or_err_u128(dec, a, 14)? << 35 |
            bits_or_err_u128(dec, a, 15)? << 30 |
            bits_or_err_u128(dec, a, 16)? << 25 |
            bits_or_err_u128(dec, a, 17)? << 20 |
            bits_or_err_u128(dec, a, 18)? << 15 |
            bits_or_err_u128(dec, a, 19)? << 10 |
            bits_or_err_u128(dec, a, 20)? <<  5 |
            bits_or_err_u128(dec, a, 21)?
        }
        21 => {
            bits_or_err_u128(dec, a,  0)? << 100 |
            bits_or_err_u128(dec, a,  1)? << 95 |
            bits_or_err_u128(dec, a,  2)? << 90 |
            bits_or_err_u128(dec, a,  3)? << 85 |
            bits_or_err_u128(dec, a,  4)? << 80 |
            bits_or_err_u128(dec, a,  5)? << 75 |
            bits_or_err_u128(dec, a,  6)? << 70 |
            bits_or_err_u128(dec, a,  7)? << 65 |
            bits_or_err_u128(dec, a,  8)? << 60 |
            bits_or_err_u128(dec, a,  9)? << 55 |
            bits_or_err_u128(dec, a, 10)? << 50 |
            bits_or_err_u128(dec, a, 11)? << 45 |
            bits_or_err_u128(dec, a, 12)? << 40 |
            bits_or_err_u128(dec, a, 13)? << 35 |
            bits_or_err_u128(dec, a, 14)? << 30 |
            bits_or_err_u128(dec, a, 15)? << 25 |
            bits_or_err_u128(dec, a, 16)? << 20 |
            bits_or_err_u128(dec, a, 17)? << 15 |
            bits_or_err_u128(dec, a, 18)? << 10 |
            bits_or_err_u128(dec, a, 19)? <<  5 |
            bits_or_err_u128(dec, a, 20)?
        }
        20 => {
            bits_or_err_u128(dec, a,  0)? << 95 |
            bits_or_err_u128(dec, a,  1)? << 90 |
            bits_or_err_u128(dec, a,  2)? << 85 |
            bits_or_err_u128(dec, a,  3)? << 80 |
            bits_or_err_u128(dec, a,  4)? << 75 |
            bits_or_err_u128(dec, a,  5)? << 70 |
            bits_or_err_u128(dec, a,  6)? << 65 |
            bits_or_err_u128(dec, a,  7)? << 60 |
            bits_or_err_u128(dec, a,  8)? << 55 |
            bits_or_err_u128(dec, a,  9)? << 50 |
            bits_or_err_u128(dec, a, 10)? << 45 |
            bits_or_err_u128(dec, a, 11)? << 40 |
            bits_or_err_u128(dec, a, 12)? << 35 |
            bits_or_err_u128(dec, a, 13)? << 30 |
            bits_or_err_u128(dec, a, 14)? << 25 |
            bits_or_err_u128(dec, a, 15)? << 20 |
            bits_or_err_u128(dec, a, 16)? << 15 |
            bits_or_err_u128(dec, a, 17)? << 10 |
            bits_or_err_u128(dec, a, 18)? <<  5 |
            bits_or_err_u128(dec, a, 19)?
        }
        19 => {
            bits_or_err_u128(dec, a,  0)? << 90 |
            bits_or_err_u128(dec, a,  1)? << 85 |
            bits_or_err_u128(dec, a,  2)? << 80 |
            bits_or_err_u128(dec, a,  3)? << 75 |
            bits_or_err_u128(dec, a,  4)? << 70 |
            bits_or_err_u128(dec, a,  5)? << 65 |
            bits_or_err_u128(dec, a,  6)? << 60 |
            bits_or_err_u128(dec, a,  7)? << 55 |
            bits_or_err_u128(dec, a,  8)? << 50 |
            bits_or_err_u128(dec, a,  9)? << 45 |
            bits_or_err_u128(dec, a, 10)? << 40 |
            bits_or_err_u128(dec, a, 11)? << 35 |
            bits_or_err_u128(dec, a, 12)? << 30 |
            bits_or_err_u128(dec, a, 13)? << 25 |
            bits_or_err_u128(dec, a, 14)? << 20 |
            bits_or_err_u128(dec, a, 15)? << 15 |
            bits_or_err_u128(dec, a, 16)? << 10 |
            bits_or_err_u128(dec, a, 17)? <<  5 |
            bits_or_err_u128(dec, a, 18)?
        }
        18 => {
            bits_or_err_u128(dec, a,  0)? << 85 |
            bits_or_err_u128(dec, a,  1)? << 80 |
            bits_or_err_u128(dec, a,  2)? << 75 |
            bits_or_err_u128(dec, a,  3)? << 70 |
            bits_or_err_u128(dec, a,  4)? << 65 |
            bits_or_err_u128(dec, a,  5)? << 60 |
            bits_or_err_u128(dec, a,  6)? << 55 |
            bits_or_err_u128(dec, a,  7)? << 50 |
            bits_or_err_u128(dec, a,  8)? << 45 |
            bits_or_err_u128(dec, a,  9)? << 40 |
            bits_or_err_u128(dec, a, 10)? << 35 |
            bits_or_err_u128(dec, a, 11)? << 30 |
            bits_or_err_u128(dec, a, 12)? << 25 |
            bits_or_err_u128(dec, a, 13)? << 20 |
            bits_or_err_u128(dec, a, 14)? << 15 |
            bits_or_err_u128(dec, a, 15)? << 10 |
            bits_or_err_u128(dec, a, 16)? <<  5 |
            bits_or_err_u128(dec, a, 17)?
        }
        17 => {
            bits_or_err_u128(dec, a,  0)? << 80 |
            bits_or_err_u128(dec, a,  1)? << 75 |
            bits_or_err_u128(dec, a,  2)? << 70 |
            bits_or_err_u128(dec, a,  3)? << 65 |
            bits_or_err_u128(dec, a,  4)? << 60 |
            bits_or_err_u128(dec, a,  5)? << 55 |
            bits_or_err_u128(dec, a,  6)? << 50 |
            bits_or_err_u128(dec, a,  7)? << 45 |
            bits_or_err_u128(dec, a,  8)? << 40 |
            bits_or_err_u128(dec, a,  9)? << 35 |
            bits_or_err_u128(dec, a, 10)? << 30 |
            bits_or_err_u128(dec, a, 11)? << 25 |
            bits_or_err_u128(dec, a, 12)? << 20 |
            bits_or_err_u128(dec, a, 13)? << 15 |
            bits_or_err_u128(dec, a, 14)? << 10 |
            bits_or_err_u128(dec, a, 15)? <<  5 |
            bits_or_err_u128(dec, a, 16)?
        }
        16 => {
            bits_or_err_u128(dec, a,  0)? << 75 |
            bits_or_err_u128(dec, a,  1)? << 70 |
            bits_or_err_u128(dec, a,  2)? << 65 |
            bits_or_err_u128(dec, a,  3)? << 60 |
            bits_or_err_u128(dec, a,  4)? << 55 |
            bits_or_err_u128(dec, a,  5)? << 50 |
            bits_or_err_u128(dec, a,  6)? << 45 |
            bits_or_err_u128(dec, a,  7)? << 40 |
            bits_or_err_u128(dec, a,  8)? << 35 |
            bits_or_err_u128(dec, a,  9)? << 30 |
            bits_or_err_u128(dec, a, 10)? << 25 |
            bits_or_err_u128(dec, a, 11)? << 20 |
            bits_or_err_u128(dec, a, 12)? << 15 |
            bits_or_err_u128(dec, a, 13)? << 10 |
            bits_or_err_u128(dec, a, 14)? <<  5 |
            bits_or_err_u128(dec, a, 15)?
        }
        15 => {
            bits_or_err_u128(dec, a,  0)? << 70 |
            bits_or_err_u128(dec, a,  1)? << 65 |
            bits_or_err_u128(dec, a,  2)? << 60 |
            bits_or_err_u128(dec, a,  3)? << 55 |
            bits_or_err_u128(dec, a,  4)? << 50 |
            bits_or_err_u128(dec, a,  5)? << 45 |
            bits_or_err_u128(dec, a,  6)? << 40 |
            bits_or_err_u128(dec, a,  7)? << 35 |
            bits_or_err_u128(dec, a,  8)? << 30 |
            bits_or_err_u128(dec, a,  9)? << 25 |
            bits_or_err_u128(dec, a, 10)? << 20 |
            bits_or_err_u128(dec, a, 11)? << 15 |
            bits_or_err_u128(dec, a, 12)? << 10 |
            bits_or_err_u128(dec, a, 13)? <<  5 |
            bits_or_err_u128(dec, a, 14)?
        }
        14 => {
            bits_or_err_u128(dec, a,  0)? << 65 |
            bits_or_err_u128(dec, a,  1)? << 60 |
            bits_or_err_u128(dec, a,  2)? << 55 |
            bits_or_err_u128(dec, a,  3)? << 50 |
            bits_or_err_u128(dec, a,  4)? << 45 |
            bits_or_err_u128(dec, a,  5)? << 40 |
            bits_or_err_u128(dec, a,  6)? << 35 |
            bits_or_err_u128(dec, a,  7)? << 30 |
            bits_or_err_u128(dec, a,  8)? << 25 |
            bits_or_err_u128(dec, a,  9)? << 20 |
            bits_or_err_u128(dec, a, 10)? << 15 |
            bits_or_err_u128(dec, a, 11)? << 10 |
            bits_or_err_u128(dec, a, 12)? <<  5 |
            bits_or_err_u128(dec, a, 13)?
        }
        13 => {
            bits_or_err_u128(dec, a,  0)? << 60 |
            bits_or_err_u128(dec, a,  1)? << 55 |
            bits_or_err_u128(dec, a,  2)? << 50 |
            bits_or_err_u128(dec, a,  3)? << 45 |
            bits_or_err_u128(dec, a,  4)? << 40 |
            bits_or_err_u128(dec, a,  5)? << 35 |
            bits_or_err_u128(dec, a,  6)? << 30 |
            bits_or_err_u128(dec, a,  7)? << 25 |
            bits_or_err_u128(dec, a,  8)? << 20 |
            bits_or_err_u128(dec, a,  9)? << 15 |
            bits_or_err_u128(dec, a, 10)? << 10 |
            bits_or_err_u128(dec, a, 11)? <<  5 |
            bits_or_err_u128(dec, a, 12)?
        }
        12 => {
            bits_or_err_u128(dec, a,  0)? << 55 |
            bits_or_err_u128(dec, a,  1)? << 50 |
            bits_or_err_u128(dec, a,  2)? << 45 |
            bits_or_err_u128(dec, a,  3)? << 40 |
            bits_or_err_u128(dec, a,  4)? << 35 |
            bits_or_err_u128(dec, a,  5)? << 30 |
            bits_or_err_u128(dec, a,  6)? << 25 |
            bits_or_err_u128(dec, a,  7)? << 20 |
            bits_or_err_u128(dec, a,  8)? << 15 |
            bits_or_err_u128(dec, a,  9)? << 10 |
            bits_or_err_u128(dec, a, 10)? <<  5 |
            bits_or_err_u128(dec, a, 11)?
        }
        11 => {
            bits_or_err_u128(dec, a,  0)? << 50 |
            bits_or_err_u128(dec, a,  1)? << 45 |
            bits_or_err_u128(dec, a,  2)? << 40 |
            bits_or_err_u128(dec, a,  3)? << 35 |
            bits_or_err_u128(dec, a,  4)? << 30 |
            bits_or_err_u128(dec, a,  5)? << 25 |
            bits_or_err_u128(dec, a,  6)? << 20 |
            bits_or_err_u128(dec, a,  7)? << 15 |
            bits_or_err_u128(dec, a,  8)? << 10 |
            bits_or_err_u128(dec, a,  9)? <<  5 |
            bits_or_err_u128(dec, a, 10)?
        }
        10 => {
            bits_or_err_u128(dec, a,  0)? << 45 |
            bits_or_err_u128(dec, a,  1)? << 40 |
            bits_or_err_u128(dec, a,  2)? << 35 |
            bits_or_err_u128(dec, a,  3)? << 30 |
            bits_or_err_u128(dec, a,  4)? << 25 |
            bits_or_err_u128(dec, a,  5)? << 20 |
            bits_or_err_u128(dec, a,  6)? << 15 |
            bits_or_err_u128(dec, a,  7)? << 10 |
            bits_or_err_u128(dec, a,  8)? <<  5 |
            bits_or_err_u128(dec, a,  9)?
        }
        9 => {
            bits_or_err_u128(dec, a,  0)? << 40 |
            bits_or_err_u128(dec, a,  1)? << 35 |
            bits_or_err_u128(dec, a,  2)? << 30 |
            bits_or_err_u128(dec, a,  3)? << 25 |
            bits_or_err_u128(dec, a,  4)? << 20 |
            bits_or_err_u128(dec, a,  5)? << 15 |
            bits_or_err_u128(dec, a,  6)? << 10 |
            bits_or_err_u128(dec, a,  7)? <<  5 |
            bits_or_err_u128(dec, a,  8)?
        }
        8 => {
            bits_or_err_u128(dec, a,  0)? << 35 |
            bits_or_err_u128(dec, a,  1)? << 30 |
            bits_or_err_u128(dec, a,  2)? << 25 |
            bits_or_err_u128(dec, a,  3)? << 20 |
            bits_or_err_u128(dec, a,  4)? << 15 |
            bits_or_err_u128(dec, a,  5)? << 10 |
            bits_or_err_u128(dec, a,  6)? <<  5 |
            bits_or_err_u128(dec, a,  7)?
        }
        7 => {
            bits_or_err_u128(dec, a,  0)? << 30 |
            bits_or_err_u128(dec, a,  1)? << 25 |
            bits_or_err_u128(dec, a,  2)? << 20 |
            bits_or_err_u128(dec, a,  3)? << 15 |
            bits_or_err_u128(dec, a,  4)? << 10 |
            bits_or_err_u128(dec, a,  5)? <<  5 |
            bits_or_err_u128(dec, a,  6)?
        }
        6 => {
            bits_or_err_u128(dec, a,  0)? << 25 |
            bits_or_err_u128(dec, a,  1)? << 20 |
            bits_or_err_u128(dec, a,  2)? << 15 |
            bits_or_err_u128(dec, a,  3)? << 10 |
            bits_or_err_u128(dec, a,  4)? <<  5 |
            bits_or_err_u128(dec, a,  5)?
        }
        5 => {
            bits_or_err_u128(dec, a,  0)? << 20 |
            bits_or_err_u128(dec, a,  1)? << 15 |
            bits_or_err_u128(dec, a,  2)? << 10 |
            bits_or_err_u128(dec, a,  3)? <<  5 |
            bits_or_err_u128(dec, a,  4)?
        }
        4 => {
            bits_or_err_u128(dec, a,  0)? << 15 |
            bits_or_err_u128(dec, a,  1)? << 10 |
            bits_or_err_u128(dec, a,  2)? <<  5 |
            bits_or_err_u128(dec, a,  3)?
        }
        3 => {
            bits_or_err_u128(dec, a,  0)? << 10 |
            bits_or_err_u128(dec, a,  1)? <<  5 |
            bits_or_err_u128(dec, a,  2)?
        }
        2 => {
            bits_or_err_u128(dec, a,  0)? <<  5 |
            bits_or_err_u128(dec, a,  1)?
        }
        1 => {
            bits_or_err_u128(dec, a,  0)?
        }
        0 => 0,
        len @ _ => Err(DecodeError::InvalidLength { length: len })?,
    };
    Ok(n)
}
