use crate::decode_base::{bits_or_err, DecodeError, DEC_CROCKFORD_UPPER};

#[inline]
pub fn decode_u64(a: impl AsRef<str>) -> Result<u64, DecodeError> {
    let a = a.as_ref().as_bytes();
    #[rustfmt::skip]
    let n = match a.len() {
        13 => {
            bits_or_err(a[ 0])? << 60 |
            bits_or_err(a[ 1])? << 55 |
            bits_or_err(a[ 2])? << 50 |
            bits_or_err(a[ 3])? << 45 |
            bits_or_err(a[ 4])? << 40 |
            bits_or_err(a[ 5])? << 35 |
            bits_or_err(a[ 6])? << 30 |
            bits_or_err(a[ 7])? << 25 |
            bits_or_err(a[ 8])? << 20 |
            bits_or_err(a[ 9])? << 15 |
            bits_or_err(a[10])? << 10 |
            bits_or_err(a[11])? <<  5 |
            bits_or_err(a[12])?
        }
        12 => {
            bits_or_err(a[ 0])? << 55 |
            bits_or_err(a[ 1])? << 50 |
            bits_or_err(a[ 2])? << 45 |
            bits_or_err(a[ 3])? << 40 |
            bits_or_err(a[ 4])? << 35 |
            bits_or_err(a[ 5])? << 30 |
            bits_or_err(a[ 6])? << 25 |
            bits_or_err(a[ 7])? << 20 |
            bits_or_err(a[ 8])? << 15 |
            bits_or_err(a[ 9])? << 10 |
            bits_or_err(a[10])? <<  5 |
            bits_or_err(a[11])?
        }
        11 => {
            bits_or_err(a[ 0])? << 50 |
            bits_or_err(a[ 1])? << 45 |
            bits_or_err(a[ 2])? << 40 |
            bits_or_err(a[ 3])? << 35 |
            bits_or_err(a[ 4])? << 30 |
            bits_or_err(a[ 5])? << 25 |
            bits_or_err(a[ 6])? << 20 |
            bits_or_err(a[ 7])? << 15 |
            bits_or_err(a[ 8])? << 10 |
            bits_or_err(a[ 9])? <<  5 |
            bits_or_err(a[10])?
        }
        10 => {
            bits_or_err(a[ 0])? << 45 |
            bits_or_err(a[ 1])? << 40 |
            bits_or_err(a[ 2])? << 35 |
            bits_or_err(a[ 3])? << 30 |
            bits_or_err(a[ 4])? << 25 |
            bits_or_err(a[ 5])? << 20 |
            bits_or_err(a[ 6])? << 15 |
            bits_or_err(a[ 7])? << 10 |
            bits_or_err(a[ 8])? <<  5 |
            bits_or_err(a[ 9])?
        }
        9 => {
            bits_or_err(a[ 0])? << 40 |
            bits_or_err(a[ 1])? << 35 |
            bits_or_err(a[ 2])? << 30 |
            bits_or_err(a[ 3])? << 25 |
            bits_or_err(a[ 4])? << 20 |
            bits_or_err(a[ 5])? << 15 |
            bits_or_err(a[ 6])? << 10 |
            bits_or_err(a[ 7])? <<  5 |
            bits_or_err(a[ 8])?
        }
        8 => {
            bits_or_err(a[ 0])? << 35 |
            bits_or_err(a[ 1])? << 30 |
            bits_or_err(a[ 2])? << 25 |
            bits_or_err(a[ 3])? << 20 |
            bits_or_err(a[ 4])? << 15 |
            bits_or_err(a[ 5])? << 10 |
            bits_or_err(a[ 6])? <<  5 |
            bits_or_err(a[ 7])?
        }
        7 => {
            bits_or_err(a[ 0])? << 30 |
            bits_or_err(a[ 1])? << 25 |
            bits_or_err(a[ 2])? << 20 |
            bits_or_err(a[ 3])? << 15 |
            bits_or_err(a[ 4])? << 10 |
            bits_or_err(a[ 5])? <<  5 |
            bits_or_err(a[ 6])?
        }
        6 => {
            bits_or_err(a[ 0])? << 25 |
            bits_or_err(a[ 1])? << 20 |
            bits_or_err(a[ 2])? << 15 |
            bits_or_err(a[ 3])? << 10 |
            bits_or_err(a[ 4])? <<  5 |
            bits_or_err(a[ 5])?
        }
        5 => {
            bits_or_err(a[ 0])? << 20 |
            bits_or_err(a[ 1])? << 15 |
            bits_or_err(a[ 2])? << 10 |
            bits_or_err(a[ 3])? <<  5 |
            bits_or_err(a[ 4])?
        }
        4 => {
            bits_or_err(a[ 0])? << 15 |
            bits_or_err(a[ 1])? << 10 |
            bits_or_err(a[ 2])? <<  5 |
            bits_or_err(a[ 3])?
        }
        3 => {
            bits_or_err(a[ 0])? << 10 |
            bits_or_err(a[ 1])? <<  5 |
            bits_or_err(a[ 2])?
        }
        2 => {
            bits_or_err(a[ 0])? <<  5 |
            bits_or_err(a[ 1])?
        }
        1 => {
            #[allow(unused_parens)]
            bits_or_err(a[ 0])?
        }
        0 => 0,
        len @ _ => Err(DecodeError::InvalidLength { length: len })?,
    };
    Ok(n)
}

#[allow(dead_code)]
pub fn decode_u64_unsafe(a: &[u8]) -> u64 {
    #[rustfmt::skip]
    let n = match a.len() {
        13 => {
            (DEC_CROCKFORD_UPPER[a[ 0] as usize] as u64) << 60 |
            (DEC_CROCKFORD_UPPER[a[ 1] as usize] as u64) << 55 |
            (DEC_CROCKFORD_UPPER[a[ 2] as usize] as u64) << 50 |
            (DEC_CROCKFORD_UPPER[a[ 3] as usize] as u64) << 45 |
            (DEC_CROCKFORD_UPPER[a[ 4] as usize] as u64) << 40 |
            (DEC_CROCKFORD_UPPER[a[ 5] as usize] as u64) << 35 |
            (DEC_CROCKFORD_UPPER[a[ 6] as usize] as u64) << 30 |
            (DEC_CROCKFORD_UPPER[a[ 7] as usize] as u64) << 25 |
            (DEC_CROCKFORD_UPPER[a[ 8] as usize] as u64) << 20 |
            (DEC_CROCKFORD_UPPER[a[ 9] as usize] as u64) << 15 |
            (DEC_CROCKFORD_UPPER[a[10] as usize] as u64) << 10 |
            (DEC_CROCKFORD_UPPER[a[11] as usize] as u64) <<  5 |
            (DEC_CROCKFORD_UPPER[a[12] as usize] as u64)
        }
        12 => {
            (DEC_CROCKFORD_UPPER[a[ 0] as usize] as u64) << 55 |
            (DEC_CROCKFORD_UPPER[a[ 1] as usize] as u64) << 50 |
            (DEC_CROCKFORD_UPPER[a[ 2] as usize] as u64) << 45 |
            (DEC_CROCKFORD_UPPER[a[ 3] as usize] as u64) << 40 |
            (DEC_CROCKFORD_UPPER[a[ 4] as usize] as u64) << 35 |
            (DEC_CROCKFORD_UPPER[a[ 5] as usize] as u64) << 30 |
            (DEC_CROCKFORD_UPPER[a[ 6] as usize] as u64) << 25 |
            (DEC_CROCKFORD_UPPER[a[ 7] as usize] as u64) << 20 |
            (DEC_CROCKFORD_UPPER[a[ 8] as usize] as u64) << 15 |
            (DEC_CROCKFORD_UPPER[a[ 9] as usize] as u64) << 10 |
            (DEC_CROCKFORD_UPPER[a[10] as usize] as u64) <<  5 |
            (DEC_CROCKFORD_UPPER[a[11] as usize] as u64)
        }
        11 => {
            (DEC_CROCKFORD_UPPER[a[ 0] as usize] as u64) << 50 |
            (DEC_CROCKFORD_UPPER[a[ 1] as usize] as u64) << 45 |
            (DEC_CROCKFORD_UPPER[a[ 2] as usize] as u64) << 40 |
            (DEC_CROCKFORD_UPPER[a[ 3] as usize] as u64) << 35 |
            (DEC_CROCKFORD_UPPER[a[ 4] as usize] as u64) << 30 |
            (DEC_CROCKFORD_UPPER[a[ 5] as usize] as u64) << 25 |
            (DEC_CROCKFORD_UPPER[a[ 6] as usize] as u64) << 20 |
            (DEC_CROCKFORD_UPPER[a[ 7] as usize] as u64) << 15 |
            (DEC_CROCKFORD_UPPER[a[ 8] as usize] as u64) << 10 |
            (DEC_CROCKFORD_UPPER[a[ 9] as usize] as u64) <<  5 |
            (DEC_CROCKFORD_UPPER[a[10] as usize] as u64)
        }
        10 => {
            (DEC_CROCKFORD_UPPER[a[ 0] as usize] as u64) << 45 |
            (DEC_CROCKFORD_UPPER[a[ 1] as usize] as u64) << 40 |
            (DEC_CROCKFORD_UPPER[a[ 2] as usize] as u64) << 35 |
            (DEC_CROCKFORD_UPPER[a[ 3] as usize] as u64) << 30 |
            (DEC_CROCKFORD_UPPER[a[ 4] as usize] as u64) << 25 |
            (DEC_CROCKFORD_UPPER[a[ 5] as usize] as u64) << 20 |
            (DEC_CROCKFORD_UPPER[a[ 6] as usize] as u64) << 15 |
            (DEC_CROCKFORD_UPPER[a[ 7] as usize] as u64) << 10 |
            (DEC_CROCKFORD_UPPER[a[ 8] as usize] as u64) <<  5 |
            (DEC_CROCKFORD_UPPER[a[ 9] as usize] as u64)
        }
        9 => {
            (DEC_CROCKFORD_UPPER[a[ 0] as usize] as u64) << 40 |
            (DEC_CROCKFORD_UPPER[a[ 1] as usize] as u64) << 35 |
            (DEC_CROCKFORD_UPPER[a[ 2] as usize] as u64) << 30 |
            (DEC_CROCKFORD_UPPER[a[ 3] as usize] as u64) << 25 |
            (DEC_CROCKFORD_UPPER[a[ 4] as usize] as u64) << 20 |
            (DEC_CROCKFORD_UPPER[a[ 5] as usize] as u64) << 15 |
            (DEC_CROCKFORD_UPPER[a[ 6] as usize] as u64) << 10 |
            (DEC_CROCKFORD_UPPER[a[ 7] as usize] as u64) <<  5 |
            (DEC_CROCKFORD_UPPER[a[ 8] as usize] as u64)
        }
        8 => {
            (DEC_CROCKFORD_UPPER[a[ 0] as usize] as u64) << 35 |
            (DEC_CROCKFORD_UPPER[a[ 1] as usize] as u64) << 30 |
            (DEC_CROCKFORD_UPPER[a[ 2] as usize] as u64) << 25 |
            (DEC_CROCKFORD_UPPER[a[ 3] as usize] as u64) << 20 |
            (DEC_CROCKFORD_UPPER[a[ 4] as usize] as u64) << 15 |
            (DEC_CROCKFORD_UPPER[a[ 5] as usize] as u64) << 10 |
            (DEC_CROCKFORD_UPPER[a[ 6] as usize] as u64) <<  5 |
            (DEC_CROCKFORD_UPPER[a[ 7] as usize] as u64)
        }
        7 => {
            (DEC_CROCKFORD_UPPER[a[ 0] as usize] as u64) << 30 |
            (DEC_CROCKFORD_UPPER[a[ 1] as usize] as u64) << 25 |
            (DEC_CROCKFORD_UPPER[a[ 2] as usize] as u64) << 20 |
            (DEC_CROCKFORD_UPPER[a[ 3] as usize] as u64) << 15 |
            (DEC_CROCKFORD_UPPER[a[ 4] as usize] as u64) << 10 |
            (DEC_CROCKFORD_UPPER[a[ 5] as usize] as u64) <<  5 |
            (DEC_CROCKFORD_UPPER[a[ 6] as usize] as u64)
        }
        6 => {
            (DEC_CROCKFORD_UPPER[a[ 0] as usize] as u64) << 25 |
            (DEC_CROCKFORD_UPPER[a[ 1] as usize] as u64) << 20 |
            (DEC_CROCKFORD_UPPER[a[ 2] as usize] as u64) << 15 |
            (DEC_CROCKFORD_UPPER[a[ 3] as usize] as u64) << 10 |
            (DEC_CROCKFORD_UPPER[a[ 4] as usize] as u64) <<  5 |
            (DEC_CROCKFORD_UPPER[a[ 5] as usize] as u64)
        }
        5 => {
            (DEC_CROCKFORD_UPPER[a[ 0] as usize] as u64) << 20 |
            (DEC_CROCKFORD_UPPER[a[ 1] as usize] as u64) << 15 |
            (DEC_CROCKFORD_UPPER[a[ 2] as usize] as u64) << 10 |
            (DEC_CROCKFORD_UPPER[a[ 3] as usize] as u64) <<  5 |
            (DEC_CROCKFORD_UPPER[a[ 4] as usize] as u64)
        }
        4 => {
            (DEC_CROCKFORD_UPPER[a[ 0] as usize] as u64) << 15 |
            (DEC_CROCKFORD_UPPER[a[ 1] as usize] as u64) << 10 |
            (DEC_CROCKFORD_UPPER[a[ 2] as usize] as u64) <<  5 |
            (DEC_CROCKFORD_UPPER[a[ 3] as usize] as u64)
        }
        3 => {
            (DEC_CROCKFORD_UPPER[a[ 0] as usize] as u64) << 10 |
            (DEC_CROCKFORD_UPPER[a[ 1] as usize] as u64) <<  5 |
            (DEC_CROCKFORD_UPPER[a[ 2] as usize] as u64)
        }
        2 => {
            (DEC_CROCKFORD_UPPER[a[ 0] as usize] as u64) <<  5 |
            (DEC_CROCKFORD_UPPER[a[ 1] as usize] as u64)
        }
        1 => {
            #[allow(unused_parens)]
            (DEC_CROCKFORD_UPPER[a[ 0] as usize] as u64)
        }
        0 => 0,
        _ => panic!("Bad size for decoding to u64!"),
    };
    n
}
