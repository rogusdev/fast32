use crate::alphabet::ENC_CROCKFORD_UPPER;
use crate::encode_base::{B32_MASK_4_4, B32_MASK_BOT_5};

#[rustfmt::skip]
#[inline]
pub fn encode_u64(n: u64) -> String {
    let enc = ENC_CROCKFORD_UPPER;
    let b = {
        if n >= 1u64 << 60 {
            [
                enc[((n >> 60) as u8 & B32_MASK_4_4) as usize],
                enc[((n >> 55) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 50) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 45) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 40) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 35) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 30) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 25) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 20) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 15) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 10) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >>  5) as u8 & B32_MASK_BOT_5) as usize],
                enc[( n        as u8 & B32_MASK_BOT_5) as usize],
            ]
            .to_vec()
        } else if n >= 1u64 << 55 {
            [
                enc[((n >> 55) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 50) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 45) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 40) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 35) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 30) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 25) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 20) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 15) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 10) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >>  5) as u8 & B32_MASK_BOT_5) as usize],
                enc[( n        as u8 & B32_MASK_BOT_5) as usize],
            ]
            .to_vec()
        } else if n >= 1u64 << 50 {
            [
                enc[((n >> 50) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 45) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 40) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 35) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 30) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 25) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 20) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 15) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 10) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >>  5) as u8 & B32_MASK_BOT_5) as usize],
                enc[( n        as u8 & B32_MASK_BOT_5) as usize],
            ]
            .to_vec()
        } else if n >= 1u64 << 45 {
            [
                enc[((n >> 45) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 40) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 35) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 30) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 25) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 20) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 15) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 10) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >>  5) as u8 & B32_MASK_BOT_5) as usize],
                enc[( n        as u8 & B32_MASK_BOT_5) as usize],
            ]
            .to_vec()
        } else if n >= 1u64 << 40 {
            [
                enc[((n >> 40) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 35) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 30) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 25) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 20) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 15) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 10) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >>  5) as u8 & B32_MASK_BOT_5) as usize],
                enc[( n        as u8 & B32_MASK_BOT_5) as usize],
            ]
            .to_vec()
        } else if n >= 1u64 << 35 {
            [
                enc[((n >> 35) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 30) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 25) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 20) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 15) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 10) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >>  5) as u8 & B32_MASK_BOT_5) as usize],
                enc[( n        as u8 & B32_MASK_BOT_5) as usize],
            ]
            .to_vec()
        } else if n >= 1u64 << 30 {
            [
                enc[((n >> 30) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 25) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 20) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 15) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 10) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >>  5) as u8 & B32_MASK_BOT_5) as usize],
                enc[( n        as u8 & B32_MASK_BOT_5) as usize],
            ]
            .to_vec()
        } else if n >= 1u64 << 25 {
            [
                enc[((n >> 25) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 20) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 15) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 10) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >>  5) as u8 & B32_MASK_BOT_5) as usize],
                enc[( n        as u8 & B32_MASK_BOT_5) as usize],
            ]
            .to_vec()
        } else if n >= 1u64 << 20 {
            [
                enc[((n >> 20) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 15) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 10) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >>  5) as u8 & B32_MASK_BOT_5) as usize],
                enc[( n        as u8 & B32_MASK_BOT_5) as usize],
            ]
            .to_vec()
        } else if n >= 1u64 << 15 {
            [
                enc[((n >> 15) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >> 10) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >>  5) as u8 & B32_MASK_BOT_5) as usize],
                enc[( n        as u8 & B32_MASK_BOT_5) as usize],
            ]
            .to_vec()
        } else if n >= 1u64 << 10 {
            [
                enc[((n >> 10) as u8 & B32_MASK_BOT_5) as usize],
                enc[((n >>  5) as u8 & B32_MASK_BOT_5) as usize],
                enc[( n        as u8 & B32_MASK_BOT_5) as usize],
            ]
            .to_vec()
        } else if n >= 1u64 << 5 {
            [
                enc[((n >>  5) as u8 & B32_MASK_BOT_5) as usize],
                enc[( n        as u8 & B32_MASK_BOT_5) as usize],
            ]
            .to_vec()
        } else {
            [
                enc[( n        as u8 & B32_MASK_BOT_5) as usize],
            ]
            .to_vec()
        }
    };
    unsafe { String::from_utf8_unchecked(b) }
}
