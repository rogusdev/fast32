use crate::alphabet::ENC_CROCKFORD_UPPER;
use crate::encode_base::{B32_MASK_BOT_4, B32_MASK_BOT_5};

#[rustfmt::skip]
#[inline]
pub fn encode_u64(n: u64) -> String {
    let enc = ENC_CROCKFORD_UPPER;

    // need this to not panic on ilog2
    if n == 0 {
        return "0".to_owned()
    }

    let b = match n.ilog2() / 5 {
        12 => [
            enc[((n >> 60) as u8 & B32_MASK_BOT_4) as usize],
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
        .to_vec(),

        11 => [
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
        .to_vec(),

        10 => [
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
        .to_vec(),

        9 => [
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
        .to_vec(),

        8 => [
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
        .to_vec(),

        7 => [
            enc[((n >> 35) as u8 & B32_MASK_BOT_5) as usize],
            enc[((n >> 30) as u8 & B32_MASK_BOT_5) as usize],
            enc[((n >> 25) as u8 & B32_MASK_BOT_5) as usize],
            enc[((n >> 20) as u8 & B32_MASK_BOT_5) as usize],
            enc[((n >> 15) as u8 & B32_MASK_BOT_5) as usize],
            enc[((n >> 10) as u8 & B32_MASK_BOT_5) as usize],
            enc[((n >>  5) as u8 & B32_MASK_BOT_5) as usize],
            enc[( n        as u8 & B32_MASK_BOT_5) as usize],
        ]
        .to_vec(),

        6 => [
            enc[((n >> 30) as u8 & B32_MASK_BOT_5) as usize],
            enc[((n >> 25) as u8 & B32_MASK_BOT_5) as usize],
            enc[((n >> 20) as u8 & B32_MASK_BOT_5) as usize],
            enc[((n >> 15) as u8 & B32_MASK_BOT_5) as usize],
            enc[((n >> 10) as u8 & B32_MASK_BOT_5) as usize],
            enc[((n >>  5) as u8 & B32_MASK_BOT_5) as usize],
            enc[( n        as u8 & B32_MASK_BOT_5) as usize],
        ]
        .to_vec(),

        5 => [
            enc[((n >> 25) as u8 & B32_MASK_BOT_5) as usize],
            enc[((n >> 20) as u8 & B32_MASK_BOT_5) as usize],
            enc[((n >> 15) as u8 & B32_MASK_BOT_5) as usize],
            enc[((n >> 10) as u8 & B32_MASK_BOT_5) as usize],
            enc[((n >>  5) as u8 & B32_MASK_BOT_5) as usize],
            enc[( n        as u8 & B32_MASK_BOT_5) as usize],
        ]
        .to_vec(),

        4 => [
            enc[((n >> 20) as u8 & B32_MASK_BOT_5) as usize],
            enc[((n >> 15) as u8 & B32_MASK_BOT_5) as usize],
            enc[((n >> 10) as u8 & B32_MASK_BOT_5) as usize],
            enc[((n >>  5) as u8 & B32_MASK_BOT_5) as usize],
            enc[( n        as u8 & B32_MASK_BOT_5) as usize],
        ]
        .to_vec(),

        3 => [
            enc[((n >> 15) as u8 & B32_MASK_BOT_5) as usize],
            enc[((n >> 10) as u8 & B32_MASK_BOT_5) as usize],
            enc[((n >>  5) as u8 & B32_MASK_BOT_5) as usize],
            enc[( n        as u8 & B32_MASK_BOT_5) as usize],
        ]
        .to_vec(),

        2 => [
            enc[((n >> 10) as u8 & B32_MASK_BOT_5) as usize],
            enc[((n >>  5) as u8 & B32_MASK_BOT_5) as usize],
            enc[( n        as u8 & B32_MASK_BOT_5) as usize],
        ]
        .to_vec(),

        1 => [
            enc[((n >>  5) as u8 & B32_MASK_BOT_5) as usize],
            enc[( n        as u8 & B32_MASK_BOT_5) as usize],
        ]
        .to_vec(),

        _ => [
            enc[( n        as u8 & B32_MASK_BOT_5) as usize],
        ]
        .to_vec(),
    };

    unsafe { String::from_utf8_unchecked(b) }
}
