use core::ptr::write;

use crate::alphabet::ENC_CROCKFORD_UPPER;
use crate::encode_base::{B32_MASK_BOT_3, B32_MASK_BOT_5};

#[rustfmt::skip]
#[inline]
pub fn encode_u128(n: u128) -> String {
    let enc = ENC_CROCKFORD_UPPER;

    // need this to not panic on ilog2
    if n == 0 {
        return "0".to_owned()
    }

    let cap = 1 + (n.ilog2() / 5) as usize;
    let mut b = Vec::<u8>::with_capacity(cap);

    match cap {
        26 => unsafe {
            let end = b.as_mut_ptr();

            // this top char is why u128 is diff output from normal bytes based base32
            // (other than exactly 5, 10, or 15 byte numbers):
            // for integer values (identifiers) we use remainder at the high bits, rather than low
            write(end        , enc[((n >> 125) as u8 & B32_MASK_BOT_3) as usize]);
            write(end.add( 1), enc[((n >> 120) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 2), enc[((n >> 115) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 3), enc[((n >> 110) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 4), enc[((n >> 105) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 5), enc[((n >> 100) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 6), enc[((n >>  95) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 7), enc[((n >>  90) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 8), enc[((n >>  85) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 9), enc[((n >>  80) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(10), enc[((n >>  75) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(11), enc[((n >>  70) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(12), enc[((n >>  65) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(13), enc[((n >>  60) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(14), enc[((n >>  55) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(15), enc[((n >>  50) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(16), enc[((n >>  45) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(17), enc[((n >>  40) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(18), enc[((n >>  35) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(19), enc[((n >>  30) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(20), enc[((n >>  25) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(21), enc[((n >>  20) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(22), enc[((n >>  15) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(23), enc[((n >>  10) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(24), enc[((n >>   5) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(25), enc[( n         as u8 & B32_MASK_BOT_5) as usize]);

            b.set_len(cap);
        }

        25 => unsafe {
            let end = b.as_mut_ptr();

            write(end        , enc[((n >> 120) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 1), enc[((n >> 115) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 2), enc[((n >> 110) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 3), enc[((n >> 105) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 4), enc[((n >> 100) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 5), enc[((n >>  95) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 6), enc[((n >>  90) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 7), enc[((n >>  85) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 8), enc[((n >>  80) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 9), enc[((n >>  75) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(10), enc[((n >>  70) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(11), enc[((n >>  65) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(12), enc[((n >>  60) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(13), enc[((n >>  55) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(14), enc[((n >>  50) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(15), enc[((n >>  45) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(16), enc[((n >>  40) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(17), enc[((n >>  35) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(18), enc[((n >>  30) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(19), enc[((n >>  25) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(20), enc[((n >>  20) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(21), enc[((n >>  15) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(22), enc[((n >>  10) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(23), enc[((n >>   5) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(24), enc[( n         as u8 & B32_MASK_BOT_5) as usize]);

            b.set_len(cap);
        }

        24 => unsafe {
            let end = b.as_mut_ptr();

            write(end        , enc[((n >> 115) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 1), enc[((n >> 110) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 2), enc[((n >> 105) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 3), enc[((n >> 100) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 4), enc[((n >>  95) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 5), enc[((n >>  90) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 6), enc[((n >>  85) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 7), enc[((n >>  80) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 8), enc[((n >>  75) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 9), enc[((n >>  70) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(10), enc[((n >>  65) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(11), enc[((n >>  60) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(12), enc[((n >>  55) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(13), enc[((n >>  50) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(14), enc[((n >>  45) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(15), enc[((n >>  40) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(16), enc[((n >>  35) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(17), enc[((n >>  30) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(18), enc[((n >>  25) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(19), enc[((n >>  20) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(20), enc[((n >>  15) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(21), enc[((n >>  10) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(22), enc[((n >>   5) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(23), enc[( n         as u8 & B32_MASK_BOT_5) as usize]);

            b.set_len(cap);
        }

        23 => unsafe {
            let end = b.as_mut_ptr();

            write(end        , enc[((n >> 110) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 1), enc[((n >> 105) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 2), enc[((n >> 100) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 3), enc[((n >>  95) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 4), enc[((n >>  90) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 5), enc[((n >>  85) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 6), enc[((n >>  80) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 7), enc[((n >>  75) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 8), enc[((n >>  70) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 9), enc[((n >>  65) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(10), enc[((n >>  60) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(11), enc[((n >>  55) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(12), enc[((n >>  50) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(13), enc[((n >>  45) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(14), enc[((n >>  40) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(15), enc[((n >>  35) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(16), enc[((n >>  30) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(17), enc[((n >>  25) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(18), enc[((n >>  20) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(19), enc[((n >>  15) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(20), enc[((n >>  10) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(21), enc[((n >>   5) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(22), enc[( n         as u8 & B32_MASK_BOT_5) as usize]);

            b.set_len(cap);
        }

        22 => unsafe {
            let end = b.as_mut_ptr();

            write(end        , enc[((n >> 105) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 1), enc[((n >> 100) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 2), enc[((n >>  95) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 3), enc[((n >>  90) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 4), enc[((n >>  85) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 5), enc[((n >>  80) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 6), enc[((n >>  75) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 7), enc[((n >>  70) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 8), enc[((n >>  65) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 9), enc[((n >>  60) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(10), enc[((n >>  55) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(11), enc[((n >>  50) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(12), enc[((n >>  45) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(13), enc[((n >>  40) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(14), enc[((n >>  35) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(15), enc[((n >>  30) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(16), enc[((n >>  25) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(17), enc[((n >>  20) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(18), enc[((n >>  15) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(19), enc[((n >>  10) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(20), enc[((n >>   5) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(21), enc[( n         as u8 & B32_MASK_BOT_5) as usize]);

            b.set_len(cap);
        }

        21 => unsafe {
            let end = b.as_mut_ptr();

            write(end        , enc[((n >> 100) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 1), enc[((n >>  95) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 2), enc[((n >>  90) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 3), enc[((n >>  85) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 4), enc[((n >>  80) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 5), enc[((n >>  75) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 6), enc[((n >>  70) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 7), enc[((n >>  65) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 8), enc[((n >>  60) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 9), enc[((n >>  55) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(10), enc[((n >>  50) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(11), enc[((n >>  45) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(12), enc[((n >>  40) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(13), enc[((n >>  35) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(14), enc[((n >>  30) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(15), enc[((n >>  25) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(16), enc[((n >>  20) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(17), enc[((n >>  15) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(18), enc[((n >>  10) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(19), enc[((n >>   5) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(20), enc[( n         as u8 & B32_MASK_BOT_5) as usize]);

            b.set_len(cap);
        }

        20 => unsafe {
            let end = b.as_mut_ptr();

            write(end        , enc[((n >>  95) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 1), enc[((n >>  90) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 2), enc[((n >>  85) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 3), enc[((n >>  80) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 4), enc[((n >>  75) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 5), enc[((n >>  70) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 6), enc[((n >>  65) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 7), enc[((n >>  60) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 8), enc[((n >>  55) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 9), enc[((n >>  50) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(10), enc[((n >>  45) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(11), enc[((n >>  40) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(12), enc[((n >>  35) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(13), enc[((n >>  30) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(14), enc[((n >>  25) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(15), enc[((n >>  20) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(16), enc[((n >>  15) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(17), enc[((n >>  10) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(18), enc[((n >>   5) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(19), enc[( n         as u8 & B32_MASK_BOT_5) as usize]);

            b.set_len(cap);
        }

        19 => unsafe {
            let end = b.as_mut_ptr();

            write(end        , enc[((n >>  90) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 1), enc[((n >>  85) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 2), enc[((n >>  80) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 3), enc[((n >>  75) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 4), enc[((n >>  70) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 5), enc[((n >>  65) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 6), enc[((n >>  60) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 7), enc[((n >>  55) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 8), enc[((n >>  50) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 9), enc[((n >>  45) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(10), enc[((n >>  40) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(11), enc[((n >>  35) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(12), enc[((n >>  30) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(13), enc[((n >>  25) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(14), enc[((n >>  20) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(15), enc[((n >>  15) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(16), enc[((n >>  10) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(17), enc[((n >>   5) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(18), enc[( n         as u8 & B32_MASK_BOT_5) as usize]);

            b.set_len(cap);
        }

        18 => unsafe {
            let end = b.as_mut_ptr();

            write(end        , enc[((n >>  85) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 1), enc[((n >>  80) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 2), enc[((n >>  75) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 3), enc[((n >>  70) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 4), enc[((n >>  65) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 5), enc[((n >>  60) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 6), enc[((n >>  55) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 7), enc[((n >>  50) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 8), enc[((n >>  45) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 9), enc[((n >>  40) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(10), enc[((n >>  35) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(11), enc[((n >>  30) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(12), enc[((n >>  25) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(13), enc[((n >>  20) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(14), enc[((n >>  15) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(15), enc[((n >>  10) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(16), enc[((n >>   5) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(17), enc[( n         as u8 & B32_MASK_BOT_5) as usize]);

            b.set_len(cap);
        }

        17 => unsafe {
            let end = b.as_mut_ptr();

            write(end        , enc[((n >>  80) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 1), enc[((n >>  75) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 2), enc[((n >>  70) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 3), enc[((n >>  65) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 4), enc[((n >>  60) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 5), enc[((n >>  55) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 6), enc[((n >>  50) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 7), enc[((n >>  45) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 8), enc[((n >>  40) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 9), enc[((n >>  35) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(10), enc[((n >>  30) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(11), enc[((n >>  25) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(12), enc[((n >>  20) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(13), enc[((n >>  15) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(14), enc[((n >>  10) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(15), enc[((n >>   5) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(16), enc[( n         as u8 & B32_MASK_BOT_5) as usize]);

            b.set_len(cap);
        }

        16 => unsafe {
            let end = b.as_mut_ptr();

            write(end        , enc[((n >>  75) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 1), enc[((n >>  70) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 2), enc[((n >>  65) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 3), enc[((n >>  60) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 4), enc[((n >>  55) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 5), enc[((n >>  50) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 6), enc[((n >>  45) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 7), enc[((n >>  40) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 8), enc[((n >>  35) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 9), enc[((n >>  30) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(10), enc[((n >>  25) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(11), enc[((n >>  20) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(12), enc[((n >>  15) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(13), enc[((n >>  10) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(14), enc[((n >>   5) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(15), enc[( n         as u8 & B32_MASK_BOT_5) as usize]);

            b.set_len(cap);
        }

        15 => unsafe {
            let end = b.as_mut_ptr();

            write(end        , enc[((n >>  70) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 1), enc[((n >>  65) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 2), enc[((n >>  60) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 3), enc[((n >>  55) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 4), enc[((n >>  50) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 5), enc[((n >>  45) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 6), enc[((n >>  40) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 7), enc[((n >>  35) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 8), enc[((n >>  30) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 9), enc[((n >>  25) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(10), enc[((n >>  20) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(11), enc[((n >>  15) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(12), enc[((n >>  10) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(13), enc[((n >>   5) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(14), enc[( n         as u8 & B32_MASK_BOT_5) as usize]);

            b.set_len(cap);
        }

        14 => unsafe {
            let end = b.as_mut_ptr();

            write(end        , enc[((n >>  65) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 1), enc[((n >>  60) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 2), enc[((n >>  55) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 3), enc[((n >>  50) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 4), enc[((n >>  45) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 5), enc[((n >>  40) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 6), enc[((n >>  35) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 7), enc[((n >>  30) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 8), enc[((n >>  25) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 9), enc[((n >>  20) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(10), enc[((n >>  15) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(11), enc[((n >>  10) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(12), enc[((n >>   5) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(13), enc[( n         as u8 & B32_MASK_BOT_5) as usize]);

            b.set_len(cap);
        }

        13 => unsafe {
            let end = b.as_mut_ptr();

            write(end        , enc[((n >>  60) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 1), enc[((n >>  55) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 2), enc[((n >>  50) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 3), enc[((n >>  45) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 4), enc[((n >>  40) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 5), enc[((n >>  35) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 6), enc[((n >>  30) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 7), enc[((n >>  25) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 8), enc[((n >>  20) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 9), enc[((n >>  15) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(10), enc[((n >>  10) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(11), enc[((n >>   5) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(12), enc[( n         as u8 & B32_MASK_BOT_5) as usize]);

            b.set_len(cap);
        }

        12 => unsafe {
            let end = b.as_mut_ptr();

            write(end        , enc[((n >> 55) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 1), enc[((n >> 50) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 2), enc[((n >> 45) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 3), enc[((n >> 40) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 4), enc[((n >> 35) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 5), enc[((n >> 30) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 6), enc[((n >> 25) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 7), enc[((n >> 20) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 8), enc[((n >> 15) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 9), enc[((n >> 10) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(10), enc[((n >>  5) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(11), enc[( n        as u8 & B32_MASK_BOT_5) as usize]);

            b.set_len(cap);
        }

        11 => unsafe {
            let end = b.as_mut_ptr();

            write(end        , enc[((n >> 50) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 1), enc[((n >> 45) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 2), enc[((n >> 40) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 3), enc[((n >> 35) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 4), enc[((n >> 30) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 5), enc[((n >> 25) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 6), enc[((n >> 20) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 7), enc[((n >> 15) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 8), enc[((n >> 10) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 9), enc[((n >>  5) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add(10), enc[( n        as u8 & B32_MASK_BOT_5) as usize]);

            b.set_len(cap);
        }

        10 => unsafe {
            let end = b.as_mut_ptr();

            write(end        , enc[((n >> 45) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 1), enc[((n >> 40) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 2), enc[((n >> 35) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 3), enc[((n >> 30) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 4), enc[((n >> 25) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 5), enc[((n >> 20) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 6), enc[((n >> 15) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 7), enc[((n >> 10) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 8), enc[((n >>  5) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 9), enc[( n        as u8 & B32_MASK_BOT_5) as usize]);

            b.set_len(cap);
        }

        9 => unsafe {
            let end = b.as_mut_ptr();

            write(end        , enc[((n >> 40) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 1), enc[((n >> 35) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 2), enc[((n >> 30) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 3), enc[((n >> 25) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 4), enc[((n >> 20) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 5), enc[((n >> 15) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 6), enc[((n >> 10) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 7), enc[((n >>  5) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 8), enc[( n        as u8 & B32_MASK_BOT_5) as usize]);

            b.set_len(cap);
        }

        8 => unsafe {
            let end = b.as_mut_ptr();

            write(end        , enc[((n >> 35) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 1), enc[((n >> 30) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 2), enc[((n >> 25) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 3), enc[((n >> 20) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 4), enc[((n >> 15) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 5), enc[((n >> 10) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 6), enc[((n >>  5) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 7), enc[( n        as u8 & B32_MASK_BOT_5) as usize]);

            b.set_len(cap);
        }

        7 => unsafe {
            let end = b.as_mut_ptr();

            write(end        , enc[((n >> 30) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 1), enc[((n >> 25) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 2), enc[((n >> 20) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 3), enc[((n >> 15) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 4), enc[((n >> 10) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 5), enc[((n >>  5) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 6), enc[( n        as u8 & B32_MASK_BOT_5) as usize]);

            b.set_len(cap);
        }

        6 => unsafe {
            let end = b.as_mut_ptr();

            write(end        , enc[((n >> 25) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 1), enc[((n >> 20) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 2), enc[((n >> 15) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 3), enc[((n >> 10) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 4), enc[((n >>  5) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 5), enc[( n        as u8 & B32_MASK_BOT_5) as usize]);

            b.set_len(cap);
        }

        5 => unsafe {
            let end = b.as_mut_ptr();

            write(end        , enc[((n >> 20) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 1), enc[((n >> 15) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 2), enc[((n >> 10) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 3), enc[((n >>  5) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 4), enc[( n        as u8 & B32_MASK_BOT_5) as usize]);

            b.set_len(cap);
        }

        4 => unsafe {
            let end = b.as_mut_ptr();

            write(end        , enc[((n >> 15) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 1), enc[((n >> 10) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 2), enc[((n >>  5) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 3), enc[( n        as u8 & B32_MASK_BOT_5) as usize]);

            b.set_len(cap);
        }

        3 => unsafe {
            let end = b.as_mut_ptr();

            write(end        , enc[((n >> 10) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 1), enc[((n >>  5) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 2), enc[( n        as u8 & B32_MASK_BOT_5) as usize]);

            b.set_len(cap);
        }

        2 => unsafe {
            let end = b.as_mut_ptr();

            write(end        , enc[((n >>  5) as u8 & B32_MASK_BOT_5) as usize]);
            write(end.add( 1), enc[( n        as u8 & B32_MASK_BOT_5) as usize]);

            b.set_len(cap);
        }

        _ => unsafe {
            let end = b.as_mut_ptr();

            write(end        , enc[( n        as u8 & B32_MASK_BOT_5) as usize]);

            b.set_len(1);
        }
    }

    unsafe { String::from_utf8_unchecked(b) }
}
