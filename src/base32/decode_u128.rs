use crate::shared::{bits_or_err_u128, DecodeError};

use super::alphabet::{
    WIDTH_1, WIDTH_10, WIDTH_11, WIDTH_12, WIDTH_13, WIDTH_14, WIDTH_15, WIDTH_16, WIDTH_17,
    WIDTH_18, WIDTH_19, WIDTH_2, WIDTH_20, WIDTH_21, WIDTH_22, WIDTH_23, WIDTH_24, WIDTH_25,
    WIDTH_3, WIDTH_4, WIDTH_5, WIDTH_6, WIDTH_7, WIDTH_8, WIDTH_9,
};

// _str version is basically identical perf to byte array,
// maybe 3-5% slower, beyond noise threshold, but not surprising
// modify comparisons like so:
//let a = "(\S+)";
//let a = "$1".to_owned();
//fast32::decode_([^(]+)\(black_box\(a
//fast32::decode_$1_str(black_box(&a
pub fn decode_u128_str(dec: &'static [u8; 256], a: impl AsRef<str>) -> Result<u128, DecodeError> {
    decode_u128(dec, a.as_ref().as_bytes())
}

pub fn decode_u128(dec: &'static [u8; 256], a: &[u8]) -> Result<u128, DecodeError> {
    #[rustfmt::skip]
    let n = match a.len() {
        26 => {
            bits_or_err_u128(dec, a,  0)? << WIDTH_25 |
            bits_or_err_u128(dec, a,  1)? << WIDTH_24 |
            bits_or_err_u128(dec, a,  2)? << WIDTH_23 |
            bits_or_err_u128(dec, a,  3)? << WIDTH_22 |
            bits_or_err_u128(dec, a,  4)? << WIDTH_21 |
            bits_or_err_u128(dec, a,  5)? << WIDTH_20 |
            bits_or_err_u128(dec, a,  6)? << WIDTH_19 |
            bits_or_err_u128(dec, a,  7)? << WIDTH_18 |
            bits_or_err_u128(dec, a,  8)? << WIDTH_17 |
            bits_or_err_u128(dec, a,  9)? << WIDTH_16 |
            bits_or_err_u128(dec, a, 10)? << WIDTH_15 |
            bits_or_err_u128(dec, a, 11)? << WIDTH_14 |
            bits_or_err_u128(dec, a, 12)? << WIDTH_13 |
            bits_or_err_u128(dec, a, 13)? << WIDTH_12 |
            bits_or_err_u128(dec, a, 14)? << WIDTH_11 |
            bits_or_err_u128(dec, a, 15)? << WIDTH_10 |
            bits_or_err_u128(dec, a, 16)? << WIDTH_9 |
            bits_or_err_u128(dec, a, 17)? << WIDTH_8 |
            bits_or_err_u128(dec, a, 18)? << WIDTH_7 |
            bits_or_err_u128(dec, a, 19)? << WIDTH_6 |
            bits_or_err_u128(dec, a, 20)? << WIDTH_5 |
            bits_or_err_u128(dec, a, 21)? << WIDTH_4 |
            bits_or_err_u128(dec, a, 22)? << WIDTH_3 |
            bits_or_err_u128(dec, a, 23)? << WIDTH_2 |
            bits_or_err_u128(dec, a, 24)? << WIDTH_1 |
            bits_or_err_u128(dec, a, 25)?
        }
        25 => {
            bits_or_err_u128(dec, a,  0)? << WIDTH_24 |
            bits_or_err_u128(dec, a,  1)? << WIDTH_23 |
            bits_or_err_u128(dec, a,  2)? << WIDTH_22 |
            bits_or_err_u128(dec, a,  3)? << WIDTH_21 |
            bits_or_err_u128(dec, a,  4)? << WIDTH_20 |
            bits_or_err_u128(dec, a,  5)? << WIDTH_19 |
            bits_or_err_u128(dec, a,  6)? << WIDTH_18 |
            bits_or_err_u128(dec, a,  7)? << WIDTH_17 |
            bits_or_err_u128(dec, a,  8)? << WIDTH_16 |
            bits_or_err_u128(dec, a,  9)? << WIDTH_15 |
            bits_or_err_u128(dec, a, 10)? << WIDTH_14 |
            bits_or_err_u128(dec, a, 11)? << WIDTH_13 |
            bits_or_err_u128(dec, a, 12)? << WIDTH_12 |
            bits_or_err_u128(dec, a, 13)? << WIDTH_11 |
            bits_or_err_u128(dec, a, 14)? << WIDTH_10 |
            bits_or_err_u128(dec, a, 15)? << WIDTH_9 |
            bits_or_err_u128(dec, a, 16)? << WIDTH_8 |
            bits_or_err_u128(dec, a, 17)? << WIDTH_7 |
            bits_or_err_u128(dec, a, 18)? << WIDTH_6 |
            bits_or_err_u128(dec, a, 19)? << WIDTH_5 |
            bits_or_err_u128(dec, a, 20)? << WIDTH_4 |
            bits_or_err_u128(dec, a, 21)? << WIDTH_3 |
            bits_or_err_u128(dec, a, 22)? << WIDTH_2 |
            bits_or_err_u128(dec, a, 23)? << WIDTH_1 |
            bits_or_err_u128(dec, a, 24)?
        }
        24 => {
            bits_or_err_u128(dec, a,  0)? << WIDTH_23 |
            bits_or_err_u128(dec, a,  1)? << WIDTH_22 |
            bits_or_err_u128(dec, a,  2)? << WIDTH_21 |
            bits_or_err_u128(dec, a,  3)? << WIDTH_20 |
            bits_or_err_u128(dec, a,  4)? << WIDTH_19 |
            bits_or_err_u128(dec, a,  5)? << WIDTH_18 |
            bits_or_err_u128(dec, a,  6)? << WIDTH_17 |
            bits_or_err_u128(dec, a,  7)? << WIDTH_16 |
            bits_or_err_u128(dec, a,  8)? << WIDTH_15 |
            bits_or_err_u128(dec, a,  9)? << WIDTH_14 |
            bits_or_err_u128(dec, a, 10)? << WIDTH_13 |
            bits_or_err_u128(dec, a, 11)? << WIDTH_12 |
            bits_or_err_u128(dec, a, 12)? << WIDTH_11 |
            bits_or_err_u128(dec, a, 13)? << WIDTH_10 |
            bits_or_err_u128(dec, a, 14)? << WIDTH_9 |
            bits_or_err_u128(dec, a, 15)? << WIDTH_8 |
            bits_or_err_u128(dec, a, 16)? << WIDTH_7 |
            bits_or_err_u128(dec, a, 17)? << WIDTH_6 |
            bits_or_err_u128(dec, a, 18)? << WIDTH_5 |
            bits_or_err_u128(dec, a, 19)? << WIDTH_4 |
            bits_or_err_u128(dec, a, 20)? << WIDTH_3 |
            bits_or_err_u128(dec, a, 21)? << WIDTH_2 |
            bits_or_err_u128(dec, a, 22)? << WIDTH_1 |
            bits_or_err_u128(dec, a, 23)?
        }
        23 => {
            bits_or_err_u128(dec, a,  0)? << WIDTH_22 |
            bits_or_err_u128(dec, a,  1)? << WIDTH_21 |
            bits_or_err_u128(dec, a,  2)? << WIDTH_20 |
            bits_or_err_u128(dec, a,  3)? << WIDTH_19 |
            bits_or_err_u128(dec, a,  4)? << WIDTH_18 |
            bits_or_err_u128(dec, a,  5)? << WIDTH_17 |
            bits_or_err_u128(dec, a,  6)? << WIDTH_16 |
            bits_or_err_u128(dec, a,  7)? << WIDTH_15 |
            bits_or_err_u128(dec, a,  8)? << WIDTH_14 |
            bits_or_err_u128(dec, a,  9)? << WIDTH_13 |
            bits_or_err_u128(dec, a, 10)? << WIDTH_12 |
            bits_or_err_u128(dec, a, 11)? << WIDTH_11 |
            bits_or_err_u128(dec, a, 12)? << WIDTH_10 |
            bits_or_err_u128(dec, a, 13)? << WIDTH_9 |
            bits_or_err_u128(dec, a, 14)? << WIDTH_8 |
            bits_or_err_u128(dec, a, 15)? << WIDTH_7 |
            bits_or_err_u128(dec, a, 16)? << WIDTH_6 |
            bits_or_err_u128(dec, a, 17)? << WIDTH_5 |
            bits_or_err_u128(dec, a, 18)? << WIDTH_4 |
            bits_or_err_u128(dec, a, 19)? << WIDTH_3 |
            bits_or_err_u128(dec, a, 20)? << WIDTH_2 |
            bits_or_err_u128(dec, a, 21)? << WIDTH_1 |
            bits_or_err_u128(dec, a, 22)?
        }
        22 => {
            bits_or_err_u128(dec, a,  0)? << WIDTH_21 |
            bits_or_err_u128(dec, a,  1)? << WIDTH_20 |
            bits_or_err_u128(dec, a,  2)? << WIDTH_19 |
            bits_or_err_u128(dec, a,  3)? << WIDTH_18 |
            bits_or_err_u128(dec, a,  4)? << WIDTH_17 |
            bits_or_err_u128(dec, a,  5)? << WIDTH_16 |
            bits_or_err_u128(dec, a,  6)? << WIDTH_15 |
            bits_or_err_u128(dec, a,  7)? << WIDTH_14 |
            bits_or_err_u128(dec, a,  8)? << WIDTH_13 |
            bits_or_err_u128(dec, a,  9)? << WIDTH_12 |
            bits_or_err_u128(dec, a, 10)? << WIDTH_11 |
            bits_or_err_u128(dec, a, 11)? << WIDTH_10 |
            bits_or_err_u128(dec, a, 12)? << WIDTH_9 |
            bits_or_err_u128(dec, a, 13)? << WIDTH_8 |
            bits_or_err_u128(dec, a, 14)? << WIDTH_7 |
            bits_or_err_u128(dec, a, 15)? << WIDTH_6 |
            bits_or_err_u128(dec, a, 16)? << WIDTH_5 |
            bits_or_err_u128(dec, a, 17)? << WIDTH_4 |
            bits_or_err_u128(dec, a, 18)? << WIDTH_3 |
            bits_or_err_u128(dec, a, 19)? << WIDTH_2 |
            bits_or_err_u128(dec, a, 20)? << WIDTH_1 |
            bits_or_err_u128(dec, a, 21)?
        }
        21 => {
            bits_or_err_u128(dec, a,  0)? << WIDTH_20 |
            bits_or_err_u128(dec, a,  1)? << WIDTH_19 |
            bits_or_err_u128(dec, a,  2)? << WIDTH_18 |
            bits_or_err_u128(dec, a,  3)? << WIDTH_17 |
            bits_or_err_u128(dec, a,  4)? << WIDTH_16 |
            bits_or_err_u128(dec, a,  5)? << WIDTH_15 |
            bits_or_err_u128(dec, a,  6)? << WIDTH_14 |
            bits_or_err_u128(dec, a,  7)? << WIDTH_13 |
            bits_or_err_u128(dec, a,  8)? << WIDTH_12 |
            bits_or_err_u128(dec, a,  9)? << WIDTH_11 |
            bits_or_err_u128(dec, a, 10)? << WIDTH_10 |
            bits_or_err_u128(dec, a, 11)? << WIDTH_9 |
            bits_or_err_u128(dec, a, 12)? << WIDTH_8 |
            bits_or_err_u128(dec, a, 13)? << WIDTH_7 |
            bits_or_err_u128(dec, a, 14)? << WIDTH_6 |
            bits_or_err_u128(dec, a, 15)? << WIDTH_5 |
            bits_or_err_u128(dec, a, 16)? << WIDTH_4 |
            bits_or_err_u128(dec, a, 17)? << WIDTH_3 |
            bits_or_err_u128(dec, a, 18)? << WIDTH_2 |
            bits_or_err_u128(dec, a, 19)? << WIDTH_1 |
            bits_or_err_u128(dec, a, 20)?
        }
        20 => {
            bits_or_err_u128(dec, a,  0)? << WIDTH_19 |
            bits_or_err_u128(dec, a,  1)? << WIDTH_18 |
            bits_or_err_u128(dec, a,  2)? << WIDTH_17 |
            bits_or_err_u128(dec, a,  3)? << WIDTH_16 |
            bits_or_err_u128(dec, a,  4)? << WIDTH_15 |
            bits_or_err_u128(dec, a,  5)? << WIDTH_14 |
            bits_or_err_u128(dec, a,  6)? << WIDTH_13 |
            bits_or_err_u128(dec, a,  7)? << WIDTH_12 |
            bits_or_err_u128(dec, a,  8)? << WIDTH_11 |
            bits_or_err_u128(dec, a,  9)? << WIDTH_10 |
            bits_or_err_u128(dec, a, 10)? << WIDTH_9 |
            bits_or_err_u128(dec, a, 11)? << WIDTH_8 |
            bits_or_err_u128(dec, a, 12)? << WIDTH_7 |
            bits_or_err_u128(dec, a, 13)? << WIDTH_6 |
            bits_or_err_u128(dec, a, 14)? << WIDTH_5 |
            bits_or_err_u128(dec, a, 15)? << WIDTH_4 |
            bits_or_err_u128(dec, a, 16)? << WIDTH_3 |
            bits_or_err_u128(dec, a, 17)? << WIDTH_2 |
            bits_or_err_u128(dec, a, 18)? << WIDTH_1 |
            bits_or_err_u128(dec, a, 19)?
        }
        19 => {
            bits_or_err_u128(dec, a,  0)? << WIDTH_18 |
            bits_or_err_u128(dec, a,  1)? << WIDTH_17 |
            bits_or_err_u128(dec, a,  2)? << WIDTH_16 |
            bits_or_err_u128(dec, a,  3)? << WIDTH_15 |
            bits_or_err_u128(dec, a,  4)? << WIDTH_14 |
            bits_or_err_u128(dec, a,  5)? << WIDTH_13 |
            bits_or_err_u128(dec, a,  6)? << WIDTH_12 |
            bits_or_err_u128(dec, a,  7)? << WIDTH_11 |
            bits_or_err_u128(dec, a,  8)? << WIDTH_10 |
            bits_or_err_u128(dec, a,  9)? << WIDTH_9 |
            bits_or_err_u128(dec, a, 10)? << WIDTH_8 |
            bits_or_err_u128(dec, a, 11)? << WIDTH_7 |
            bits_or_err_u128(dec, a, 12)? << WIDTH_6 |
            bits_or_err_u128(dec, a, 13)? << WIDTH_5 |
            bits_or_err_u128(dec, a, 14)? << WIDTH_4 |
            bits_or_err_u128(dec, a, 15)? << WIDTH_3 |
            bits_or_err_u128(dec, a, 16)? << WIDTH_2 |
            bits_or_err_u128(dec, a, 17)? << WIDTH_1 |
            bits_or_err_u128(dec, a, 18)?
        }
        18 => {
            bits_or_err_u128(dec, a,  0)? << WIDTH_17 |
            bits_or_err_u128(dec, a,  1)? << WIDTH_16 |
            bits_or_err_u128(dec, a,  2)? << WIDTH_15 |
            bits_or_err_u128(dec, a,  3)? << WIDTH_14 |
            bits_or_err_u128(dec, a,  4)? << WIDTH_13 |
            bits_or_err_u128(dec, a,  5)? << WIDTH_12 |
            bits_or_err_u128(dec, a,  6)? << WIDTH_11 |
            bits_or_err_u128(dec, a,  7)? << WIDTH_10 |
            bits_or_err_u128(dec, a,  8)? << WIDTH_9 |
            bits_or_err_u128(dec, a,  9)? << WIDTH_8 |
            bits_or_err_u128(dec, a, 10)? << WIDTH_7 |
            bits_or_err_u128(dec, a, 11)? << WIDTH_6 |
            bits_or_err_u128(dec, a, 12)? << WIDTH_5 |
            bits_or_err_u128(dec, a, 13)? << WIDTH_4 |
            bits_or_err_u128(dec, a, 14)? << WIDTH_3 |
            bits_or_err_u128(dec, a, 15)? << WIDTH_2 |
            bits_or_err_u128(dec, a, 16)? << WIDTH_1 |
            bits_or_err_u128(dec, a, 17)?
        }
        17 => {
            bits_or_err_u128(dec, a,  0)? << WIDTH_16 |
            bits_or_err_u128(dec, a,  1)? << WIDTH_15 |
            bits_or_err_u128(dec, a,  2)? << WIDTH_14 |
            bits_or_err_u128(dec, a,  3)? << WIDTH_13 |
            bits_or_err_u128(dec, a,  4)? << WIDTH_12 |
            bits_or_err_u128(dec, a,  5)? << WIDTH_11 |
            bits_or_err_u128(dec, a,  6)? << WIDTH_10 |
            bits_or_err_u128(dec, a,  7)? << WIDTH_9 |
            bits_or_err_u128(dec, a,  8)? << WIDTH_8 |
            bits_or_err_u128(dec, a,  9)? << WIDTH_7 |
            bits_or_err_u128(dec, a, 10)? << WIDTH_6 |
            bits_or_err_u128(dec, a, 11)? << WIDTH_5 |
            bits_or_err_u128(dec, a, 12)? << WIDTH_4 |
            bits_or_err_u128(dec, a, 13)? << WIDTH_3 |
            bits_or_err_u128(dec, a, 14)? << WIDTH_2 |
            bits_or_err_u128(dec, a, 15)? << WIDTH_1 |
            bits_or_err_u128(dec, a, 16)?
        }
        16 => {
            bits_or_err_u128(dec, a,  0)? << WIDTH_15 |
            bits_or_err_u128(dec, a,  1)? << WIDTH_14 |
            bits_or_err_u128(dec, a,  2)? << WIDTH_13 |
            bits_or_err_u128(dec, a,  3)? << WIDTH_12 |
            bits_or_err_u128(dec, a,  4)? << WIDTH_11 |
            bits_or_err_u128(dec, a,  5)? << WIDTH_10 |
            bits_or_err_u128(dec, a,  6)? << WIDTH_9 |
            bits_or_err_u128(dec, a,  7)? << WIDTH_8 |
            bits_or_err_u128(dec, a,  8)? << WIDTH_7 |
            bits_or_err_u128(dec, a,  9)? << WIDTH_6 |
            bits_or_err_u128(dec, a, 10)? << WIDTH_5 |
            bits_or_err_u128(dec, a, 11)? << WIDTH_4 |
            bits_or_err_u128(dec, a, 12)? << WIDTH_3 |
            bits_or_err_u128(dec, a, 13)? << WIDTH_2 |
            bits_or_err_u128(dec, a, 14)? << WIDTH_1 |
            bits_or_err_u128(dec, a, 15)?
        }
        15 => {
            bits_or_err_u128(dec, a,  0)? << WIDTH_14 |
            bits_or_err_u128(dec, a,  1)? << WIDTH_13 |
            bits_or_err_u128(dec, a,  2)? << WIDTH_12 |
            bits_or_err_u128(dec, a,  3)? << WIDTH_11 |
            bits_or_err_u128(dec, a,  4)? << WIDTH_10 |
            bits_or_err_u128(dec, a,  5)? << WIDTH_9 |
            bits_or_err_u128(dec, a,  6)? << WIDTH_8 |
            bits_or_err_u128(dec, a,  7)? << WIDTH_7 |
            bits_or_err_u128(dec, a,  8)? << WIDTH_6 |
            bits_or_err_u128(dec, a,  9)? << WIDTH_5 |
            bits_or_err_u128(dec, a, 10)? << WIDTH_4 |
            bits_or_err_u128(dec, a, 11)? << WIDTH_3 |
            bits_or_err_u128(dec, a, 12)? << WIDTH_2 |
            bits_or_err_u128(dec, a, 13)? << WIDTH_1 |
            bits_or_err_u128(dec, a, 14)?
        }
        14 => {
            bits_or_err_u128(dec, a,  0)? << WIDTH_13 |
            bits_or_err_u128(dec, a,  1)? << WIDTH_12 |
            bits_or_err_u128(dec, a,  2)? << WIDTH_11 |
            bits_or_err_u128(dec, a,  3)? << WIDTH_10 |
            bits_or_err_u128(dec, a,  4)? << WIDTH_9 |
            bits_or_err_u128(dec, a,  5)? << WIDTH_8 |
            bits_or_err_u128(dec, a,  6)? << WIDTH_7 |
            bits_or_err_u128(dec, a,  7)? << WIDTH_6 |
            bits_or_err_u128(dec, a,  8)? << WIDTH_5 |
            bits_or_err_u128(dec, a,  9)? << WIDTH_4 |
            bits_or_err_u128(dec, a, 10)? << WIDTH_3 |
            bits_or_err_u128(dec, a, 11)? << WIDTH_2 |
            bits_or_err_u128(dec, a, 12)? << WIDTH_1 |
            bits_or_err_u128(dec, a, 13)?
        }
        13 => {
            bits_or_err_u128(dec, a,  0)? << WIDTH_12 |
            bits_or_err_u128(dec, a,  1)? << WIDTH_11 |
            bits_or_err_u128(dec, a,  2)? << WIDTH_10 |
            bits_or_err_u128(dec, a,  3)? << WIDTH_9 |
            bits_or_err_u128(dec, a,  4)? << WIDTH_8 |
            bits_or_err_u128(dec, a,  5)? << WIDTH_7 |
            bits_or_err_u128(dec, a,  6)? << WIDTH_6 |
            bits_or_err_u128(dec, a,  7)? << WIDTH_5 |
            bits_or_err_u128(dec, a,  8)? << WIDTH_4 |
            bits_or_err_u128(dec, a,  9)? << WIDTH_3 |
            bits_or_err_u128(dec, a, 10)? << WIDTH_2 |
            bits_or_err_u128(dec, a, 11)? << WIDTH_1 |
            bits_or_err_u128(dec, a, 12)?
        }
        12 => {
            bits_or_err_u128(dec, a,  0)? << WIDTH_11 |
            bits_or_err_u128(dec, a,  1)? << WIDTH_10 |
            bits_or_err_u128(dec, a,  2)? << WIDTH_9 |
            bits_or_err_u128(dec, a,  3)? << WIDTH_8 |
            bits_or_err_u128(dec, a,  4)? << WIDTH_7 |
            bits_or_err_u128(dec, a,  5)? << WIDTH_6 |
            bits_or_err_u128(dec, a,  6)? << WIDTH_5 |
            bits_or_err_u128(dec, a,  7)? << WIDTH_4 |
            bits_or_err_u128(dec, a,  8)? << WIDTH_3 |
            bits_or_err_u128(dec, a,  9)? << WIDTH_2 |
            bits_or_err_u128(dec, a, 10)? << WIDTH_1 |
            bits_or_err_u128(dec, a, 11)?
        }
        11 => {
            bits_or_err_u128(dec, a,  0)? << WIDTH_10 |
            bits_or_err_u128(dec, a,  1)? << WIDTH_9 |
            bits_or_err_u128(dec, a,  2)? << WIDTH_8 |
            bits_or_err_u128(dec, a,  3)? << WIDTH_7 |
            bits_or_err_u128(dec, a,  4)? << WIDTH_6 |
            bits_or_err_u128(dec, a,  5)? << WIDTH_5 |
            bits_or_err_u128(dec, a,  6)? << WIDTH_4 |
            bits_or_err_u128(dec, a,  7)? << WIDTH_3 |
            bits_or_err_u128(dec, a,  8)? << WIDTH_2 |
            bits_or_err_u128(dec, a,  9)? << WIDTH_1 |
            bits_or_err_u128(dec, a, 10)?
        }
        10 => {
            bits_or_err_u128(dec, a,  0)? << WIDTH_9 |
            bits_or_err_u128(dec, a,  1)? << WIDTH_8 |
            bits_or_err_u128(dec, a,  2)? << WIDTH_7 |
            bits_or_err_u128(dec, a,  3)? << WIDTH_6 |
            bits_or_err_u128(dec, a,  4)? << WIDTH_5 |
            bits_or_err_u128(dec, a,  5)? << WIDTH_4 |
            bits_or_err_u128(dec, a,  6)? << WIDTH_3 |
            bits_or_err_u128(dec, a,  7)? << WIDTH_2 |
            bits_or_err_u128(dec, a,  8)? << WIDTH_1 |
            bits_or_err_u128(dec, a,  9)?
        }
        9 => {
            bits_or_err_u128(dec, a,  0)? << WIDTH_8 |
            bits_or_err_u128(dec, a,  1)? << WIDTH_7 |
            bits_or_err_u128(dec, a,  2)? << WIDTH_6 |
            bits_or_err_u128(dec, a,  3)? << WIDTH_5 |
            bits_or_err_u128(dec, a,  4)? << WIDTH_4 |
            bits_or_err_u128(dec, a,  5)? << WIDTH_3 |
            bits_or_err_u128(dec, a,  6)? << WIDTH_2 |
            bits_or_err_u128(dec, a,  7)? << WIDTH_1 |
            bits_or_err_u128(dec, a,  8)?
        }
        8 => {
            bits_or_err_u128(dec, a,  0)? << WIDTH_7 |
            bits_or_err_u128(dec, a,  1)? << WIDTH_6 |
            bits_or_err_u128(dec, a,  2)? << WIDTH_5 |
            bits_or_err_u128(dec, a,  3)? << WIDTH_4 |
            bits_or_err_u128(dec, a,  4)? << WIDTH_3 |
            bits_or_err_u128(dec, a,  5)? << WIDTH_2 |
            bits_or_err_u128(dec, a,  6)? << WIDTH_1 |
            bits_or_err_u128(dec, a,  7)?
        }
        7 => {
            bits_or_err_u128(dec, a,  0)? << WIDTH_6 |
            bits_or_err_u128(dec, a,  1)? << WIDTH_5 |
            bits_or_err_u128(dec, a,  2)? << WIDTH_4 |
            bits_or_err_u128(dec, a,  3)? << WIDTH_3 |
            bits_or_err_u128(dec, a,  4)? << WIDTH_2 |
            bits_or_err_u128(dec, a,  5)? << WIDTH_1 |
            bits_or_err_u128(dec, a,  6)?
        }
        6 => {
            bits_or_err_u128(dec, a,  0)? << WIDTH_5 |
            bits_or_err_u128(dec, a,  1)? << WIDTH_4 |
            bits_or_err_u128(dec, a,  2)? << WIDTH_3 |
            bits_or_err_u128(dec, a,  3)? << WIDTH_2 |
            bits_or_err_u128(dec, a,  4)? << WIDTH_1 |
            bits_or_err_u128(dec, a,  5)?
        }
       5 => {
            bits_or_err_u128(dec, a,  0)? << WIDTH_4 |
            bits_or_err_u128(dec, a,  1)? << WIDTH_3 |
            bits_or_err_u128(dec, a,  2)? << WIDTH_2 |
            bits_or_err_u128(dec, a,  3)? << WIDTH_1 |
            bits_or_err_u128(dec, a,  4)?
        }
        4 => {
            bits_or_err_u128(dec, a,  0)? << WIDTH_3 |
            bits_or_err_u128(dec, a,  1)? << WIDTH_2 |
            bits_or_err_u128(dec, a,  2)? << WIDTH_1 |
            bits_or_err_u128(dec, a,  3)?
        }
        3 => {
            bits_or_err_u128(dec, a,  0)? << WIDTH_2 |
            bits_or_err_u128(dec, a,  1)? << WIDTH_1 |
            bits_or_err_u128(dec, a,  2)?
        }
        2 => {
            bits_or_err_u128(dec, a,  0)? << WIDTH_1 |
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
