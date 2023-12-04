use crate::ENC_CROCKFORD_UPPER;

// either zeroes then ones, or else zeroes on each side of ones
const B32_MASK_3_5: u8 = 0b00011111;
const B32_MASK_4_4: u8 = 0b00001111;

#[rustfmt::skip]
#[inline]
pub fn encode_u64(n: u64) -> String {
    let b = {
        if n >= 1u64 << 60 {
            [
                ENC_CROCKFORD_UPPER[((n >> 60) as u8 & B32_MASK_4_4) as usize],
                ENC_CROCKFORD_UPPER[((n >> 55) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 50) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 45) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 40) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 35) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 30) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 25) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 20) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 15) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 10) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >>  5) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[( n        as u8 & B32_MASK_3_5) as usize],
            ]
            .to_vec()
        } else if n >= 1u64 << 55 {
            [
                ENC_CROCKFORD_UPPER[((n >> 55) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 50) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 45) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 40) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 35) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 30) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 25) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 20) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 15) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 10) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >>  5) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[( n        as u8 & B32_MASK_3_5) as usize],
            ]
            .to_vec()
        } else if n >= 1u64 << 50 {
            [
                ENC_CROCKFORD_UPPER[((n >> 50) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 45) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 40) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 35) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 30) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 25) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 20) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 15) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 10) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >>  5) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[( n        as u8 & B32_MASK_3_5) as usize],
            ]
            .to_vec()
        } else if n >= 1u64 << 45 {
            [
                ENC_CROCKFORD_UPPER[((n >> 45) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 40) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 35) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 30) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 25) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 20) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 15) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 10) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >>  5) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[( n        as u8 & B32_MASK_3_5) as usize],
            ]
            .to_vec()
        } else if n >= 1u64 << 40 {
            [
                ENC_CROCKFORD_UPPER[((n >> 40) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 35) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 30) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 25) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 20) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 15) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 10) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >>  5) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[( n        as u8 & B32_MASK_3_5) as usize],
            ]
            .to_vec()
        } else if n >= 1u64 << 35 {
            [
                ENC_CROCKFORD_UPPER[((n >> 35) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 30) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 25) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 20) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 15) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 10) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >>  5) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[( n        as u8 & B32_MASK_3_5) as usize],
            ]
            .to_vec()
        } else if n >= 1u64 << 30 {
            [
                ENC_CROCKFORD_UPPER[((n >> 30) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 25) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 20) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 15) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 10) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >>  5) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[( n        as u8 & B32_MASK_3_5) as usize],
            ]
            .to_vec()
        } else if n >= 1u64 << 25 {
            [
                ENC_CROCKFORD_UPPER[((n >> 25) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 20) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 15) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 10) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >>  5) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[( n        as u8 & B32_MASK_3_5) as usize],
            ]
            .to_vec()
        } else if n >= 1u64 << 20 {
            [
                ENC_CROCKFORD_UPPER[((n >> 20) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 15) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 10) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >>  5) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[( n        as u8 & B32_MASK_3_5) as usize],
            ]
            .to_vec()
        } else if n >= 1u64 << 15 {
            [
                ENC_CROCKFORD_UPPER[((n >> 15) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >> 10) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >>  5) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[( n        as u8 & B32_MASK_3_5) as usize],
            ]
            .to_vec()
        } else if n >= 1u64 << 10 {
            [
                ENC_CROCKFORD_UPPER[((n >> 10) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[((n >>  5) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[( n        as u8 & B32_MASK_3_5) as usize],
            ]
            .to_vec()
        } else if n >= 1u64 << 5 {
            [
                ENC_CROCKFORD_UPPER[((n >>  5) as u8 & B32_MASK_3_5) as usize],
                ENC_CROCKFORD_UPPER[( n        as u8 & B32_MASK_3_5) as usize],
            ]
            .to_vec()
        } else {
            [
                ENC_CROCKFORD_UPPER[( n        as u8 & B32_MASK_3_5) as usize],
            ]
            .to_vec()
        }
    };
    unsafe { String::from_utf8_unchecked(b) }
}
