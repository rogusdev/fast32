// unsafe writing adapted from https://doc.rust-lang.org/src/alloc/vec/mod.rs.html#1878
use core::ptr::write;

use crate::shared::U8_MASK_BOT_6;

use super::alphabet::{
    BITS, WIDTH_1, WIDTH_10, WIDTH_11, WIDTH_12, WIDTH_13, WIDTH_14, WIDTH_15, WIDTH_16, WIDTH_17,
    WIDTH_18, WIDTH_19, WIDTH_2, WIDTH_20, WIDTH_21, WIDTH_3, WIDTH_4, WIDTH_5, WIDTH_6, WIDTH_7,
    WIDTH_8, WIDTH_9,
};

#[inline]
pub const fn capacity_u128(n: u128) -> usize {
    if let Some(log) = n.checked_ilog2() {
        1 + (log / 6) as usize
    } else {
        1
    }
}

pub fn encode_u128(enc: &'static [u8; BITS], n: u128) -> String {
    let cap = capacity_u128(n);
    let mut b = Vec::<u8>::with_capacity(cap);
    encode_u128_inner(enc, n, &mut b, cap, 0, cap);
    unsafe { String::from_utf8_unchecked(b) }
}

pub fn encode_u128_into(enc: &'static [u8; BITS], n: u128, b: &mut Vec<u8>) {
    let cap = capacity_u128(n);
    let len = b.len();
    let new_len = len + cap;
    assert!(b.capacity() >= new_len, "Missing capacity for encoding");
    encode_u128_inner(enc, n, b, cap, len, new_len);
}

#[rustfmt::skip]
#[inline(always)]
fn encode_u128_inner(enc: &'static [u8; BITS], n: u128, b: &mut Vec<u8>, cap: usize, len: usize, new_len: usize) {
    match cap {
        22 => unsafe {
            let end = b.as_mut_ptr().add(len);

            // this top char is why u128 is diff output from normal bytes based base64
            // (other than multiples of 3 byte numbers):
            // for integer values (identifiers) we use remainder at the high bits, rather than low
            write(end        , enc[((n >> WIDTH_21) as u8                ) as usize]);
            write(end.add( 1), enc[((n >> WIDTH_20) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 2), enc[((n >> WIDTH_19) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 3), enc[((n >> WIDTH_18) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 4), enc[((n >> WIDTH_17) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 5), enc[((n >> WIDTH_16) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 6), enc[((n >> WIDTH_15) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 7), enc[((n >> WIDTH_14) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 8), enc[((n >> WIDTH_13) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 9), enc[((n >> WIDTH_12) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(10), enc[((n >> WIDTH_11) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(11), enc[((n >> WIDTH_10) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(12), enc[((n >> WIDTH_9) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(13), enc[((n >> WIDTH_8) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(14), enc[((n >> WIDTH_7) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(15), enc[((n >> WIDTH_6) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(16), enc[((n >> WIDTH_5) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(17), enc[((n >> WIDTH_4) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(18), enc[((n >> WIDTH_3) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(19), enc[((n >> WIDTH_2) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(20), enc[((n >> WIDTH_1) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(21), enc[( n             as u8 & U8_MASK_BOT_6) as usize]);

            b.set_len(new_len);
        }

        21 => unsafe {
            let end = b.as_mut_ptr().add(len);

            write(end        , enc[((n >> WIDTH_20) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 1), enc[((n >> WIDTH_19) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 2), enc[((n >> WIDTH_18) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 3), enc[((n >> WIDTH_17) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 4), enc[((n >> WIDTH_16) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 5), enc[((n >> WIDTH_15) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 6), enc[((n >> WIDTH_14) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 7), enc[((n >> WIDTH_13) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 8), enc[((n >> WIDTH_12) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 9), enc[((n >> WIDTH_11) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(10), enc[((n >> WIDTH_10) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(11), enc[((n >> WIDTH_9) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(12), enc[((n >> WIDTH_8) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(13), enc[((n >> WIDTH_7) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(14), enc[((n >> WIDTH_6) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(15), enc[((n >> WIDTH_5) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(16), enc[((n >> WIDTH_4) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(17), enc[((n >> WIDTH_3) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(18), enc[((n >> WIDTH_2) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(19), enc[((n >> WIDTH_1) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(20), enc[( n             as u8 & U8_MASK_BOT_6) as usize]);

            b.set_len(new_len);
        }

        20 => unsafe {
            let end = b.as_mut_ptr().add(len);

            write(end        , enc[((n >> WIDTH_19) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 1), enc[((n >> WIDTH_18) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 2), enc[((n >> WIDTH_17) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 3), enc[((n >> WIDTH_16) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 4), enc[((n >> WIDTH_15) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 5), enc[((n >> WIDTH_14) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 6), enc[((n >> WIDTH_13) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 7), enc[((n >> WIDTH_12) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 8), enc[((n >> WIDTH_11) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 9), enc[((n >> WIDTH_10) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(10), enc[((n >> WIDTH_9) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(11), enc[((n >> WIDTH_8) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(12), enc[((n >> WIDTH_7) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(13), enc[((n >> WIDTH_6) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(14), enc[((n >> WIDTH_5) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(15), enc[((n >> WIDTH_4) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(16), enc[((n >> WIDTH_3) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(17), enc[((n >> WIDTH_2) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(18), enc[((n >> WIDTH_1) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(19), enc[( n             as u8 & U8_MASK_BOT_6) as usize]);

            b.set_len(new_len);
        }

        19 => unsafe {
            let end = b.as_mut_ptr().add(len);

            write(end        , enc[((n >> WIDTH_18) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 1), enc[((n >> WIDTH_17) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 2), enc[((n >> WIDTH_16) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 3), enc[((n >> WIDTH_15) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 4), enc[((n >> WIDTH_14) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 5), enc[((n >> WIDTH_13) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 6), enc[((n >> WIDTH_12) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 7), enc[((n >> WIDTH_11) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 8), enc[((n >> WIDTH_10) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 9), enc[((n >> WIDTH_9) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(10), enc[((n >> WIDTH_8) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(11), enc[((n >> WIDTH_7) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(12), enc[((n >> WIDTH_6) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(13), enc[((n >> WIDTH_5) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(14), enc[((n >> WIDTH_4) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(15), enc[((n >> WIDTH_3) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(16), enc[((n >> WIDTH_2) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(17), enc[((n >> WIDTH_1) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(18), enc[( n             as u8 & U8_MASK_BOT_6) as usize]);

            b.set_len(new_len);
        }

        18 => unsafe {
            let end = b.as_mut_ptr().add(len);

            write(end        , enc[((n >> WIDTH_17) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 1), enc[((n >> WIDTH_16) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 2), enc[((n >> WIDTH_15) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 3), enc[((n >> WIDTH_14) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 4), enc[((n >> WIDTH_13) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 5), enc[((n >> WIDTH_12) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 6), enc[((n >> WIDTH_11) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 7), enc[((n >> WIDTH_10) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 8), enc[((n >> WIDTH_9) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 9), enc[((n >> WIDTH_8) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(10), enc[((n >> WIDTH_7) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(11), enc[((n >> WIDTH_6) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(12), enc[((n >> WIDTH_5) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(13), enc[((n >> WIDTH_4) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(14), enc[((n >> WIDTH_3) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(15), enc[((n >> WIDTH_2) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(16), enc[((n >> WIDTH_1) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(17), enc[( n             as u8 & U8_MASK_BOT_6) as usize]);

            b.set_len(new_len);
        }

        17 => unsafe {
            let end = b.as_mut_ptr().add(len);

            write(end        , enc[((n >> WIDTH_16) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 1), enc[((n >> WIDTH_15) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 2), enc[((n >> WIDTH_14) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 3), enc[((n >> WIDTH_13) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 4), enc[((n >> WIDTH_12) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 5), enc[((n >> WIDTH_11) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 6), enc[((n >> WIDTH_10) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 7), enc[((n >> WIDTH_9) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 8), enc[((n >> WIDTH_8) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 9), enc[((n >> WIDTH_7) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(10), enc[((n >> WIDTH_6) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(11), enc[((n >> WIDTH_5) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(12), enc[((n >> WIDTH_4) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(13), enc[((n >> WIDTH_3) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(14), enc[((n >> WIDTH_2) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(15), enc[((n >> WIDTH_1) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(16), enc[( n             as u8 & U8_MASK_BOT_6) as usize]);

            b.set_len(new_len);
        }

        16 => unsafe {
            let end = b.as_mut_ptr().add(len);

            write(end        , enc[((n >> WIDTH_15) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 1), enc[((n >> WIDTH_14) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 2), enc[((n >> WIDTH_13) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 3), enc[((n >> WIDTH_12) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 4), enc[((n >> WIDTH_11) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 5), enc[((n >> WIDTH_10) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 6), enc[((n >> WIDTH_9) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 7), enc[((n >> WIDTH_8) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 8), enc[((n >> WIDTH_7) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 9), enc[((n >> WIDTH_6) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(10), enc[((n >> WIDTH_5) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(11), enc[((n >> WIDTH_4) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(12), enc[((n >> WIDTH_3) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(13), enc[((n >> WIDTH_2) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(14), enc[((n >> WIDTH_1) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(15), enc[( n             as u8 & U8_MASK_BOT_6) as usize]);

            b.set_len(new_len);
        }

        15 => unsafe {
            let end = b.as_mut_ptr().add(len);

            write(end        , enc[((n >> WIDTH_14) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 1), enc[((n >> WIDTH_13) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 2), enc[((n >> WIDTH_12) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 3), enc[((n >> WIDTH_11) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 4), enc[((n >> WIDTH_10) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 5), enc[((n >> WIDTH_9) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 6), enc[((n >> WIDTH_8) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 7), enc[((n >> WIDTH_7) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 8), enc[((n >> WIDTH_6) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 9), enc[((n >> WIDTH_5) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(10), enc[((n >> WIDTH_4) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(11), enc[((n >> WIDTH_3) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(12), enc[((n >> WIDTH_2) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(13), enc[((n >> WIDTH_1) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(14), enc[( n             as u8 & U8_MASK_BOT_6) as usize]);

            b.set_len(new_len);
        }

        14 => unsafe {
            let end = b.as_mut_ptr().add(len);

            write(end        , enc[((n >> WIDTH_13) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 1), enc[((n >> WIDTH_12) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 2), enc[((n >> WIDTH_11) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 3), enc[((n >> WIDTH_10) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 4), enc[((n >> WIDTH_9) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 5), enc[((n >> WIDTH_8) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 6), enc[((n >> WIDTH_7) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 7), enc[((n >> WIDTH_6) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 8), enc[((n >> WIDTH_5) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 9), enc[((n >> WIDTH_4) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(10), enc[((n >> WIDTH_3) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(11), enc[((n >> WIDTH_2) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(12), enc[((n >> WIDTH_1) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(13), enc[( n             as u8 & U8_MASK_BOT_6) as usize]);

            b.set_len(new_len);
        }

        13 => unsafe {
            let end = b.as_mut_ptr().add(len);

            write(end        , enc[((n >> WIDTH_12) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 1), enc[((n >> WIDTH_11) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 2), enc[((n >> WIDTH_10) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 3), enc[((n >> WIDTH_9) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 4), enc[((n >> WIDTH_8) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 5), enc[((n >> WIDTH_7) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 6), enc[((n >> WIDTH_6) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 7), enc[((n >> WIDTH_5) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 8), enc[((n >> WIDTH_4) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 9), enc[((n >> WIDTH_3) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(10), enc[((n >> WIDTH_2) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(11), enc[((n >> WIDTH_1) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(12), enc[( n             as u8 & U8_MASK_BOT_6) as usize]);

            b.set_len(new_len);
        }

        12 => unsafe {
            let end = b.as_mut_ptr().add(len);

            write(end        , enc[((n >> WIDTH_11) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 1), enc[((n >> WIDTH_10) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 2), enc[((n >> WIDTH_9) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 3), enc[((n >> WIDTH_8) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 4), enc[((n >> WIDTH_7) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 5), enc[((n >> WIDTH_6) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 6), enc[((n >> WIDTH_5) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 7), enc[((n >> WIDTH_4) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 8), enc[((n >> WIDTH_3) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 9), enc[((n >> WIDTH_2) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(10), enc[((n >> WIDTH_1) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(11), enc[( n             as u8 & U8_MASK_BOT_6) as usize]);

            b.set_len(new_len);
        }

        11 => unsafe {
            let end = b.as_mut_ptr().add(len);

            write(end        , enc[((n >> WIDTH_10) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 1), enc[((n >> WIDTH_9) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 2), enc[((n >> WIDTH_8) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 3), enc[((n >> WIDTH_7) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 4), enc[((n >> WIDTH_6) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 5), enc[((n >> WIDTH_5) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 6), enc[((n >> WIDTH_4) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 7), enc[((n >> WIDTH_3) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 8), enc[((n >> WIDTH_2) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 9), enc[((n >> WIDTH_1) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add(10), enc[( n             as u8 & U8_MASK_BOT_6) as usize]);

            b.set_len(new_len);
        }

        10 => unsafe {
            let end = b.as_mut_ptr().add(len);

            write(end        , enc[((n >> WIDTH_9) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 1), enc[((n >> WIDTH_8) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 2), enc[((n >> WIDTH_7) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 3), enc[((n >> WIDTH_6) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 4), enc[((n >> WIDTH_5) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 5), enc[((n >> WIDTH_4) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 6), enc[((n >> WIDTH_3) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 7), enc[((n >> WIDTH_2) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 8), enc[((n >> WIDTH_1) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 9), enc[( n             as u8 & U8_MASK_BOT_6) as usize]);

            b.set_len(new_len);
        }

        9 => unsafe {
            let end = b.as_mut_ptr().add(len);

            write(end        , enc[((n >> WIDTH_8) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 1), enc[((n >> WIDTH_7) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 2), enc[((n >> WIDTH_6) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 3), enc[((n >> WIDTH_5) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 4), enc[((n >> WIDTH_4) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 5), enc[((n >> WIDTH_3) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 6), enc[((n >> WIDTH_2) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 7), enc[((n >> WIDTH_1) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 8), enc[( n             as u8 & U8_MASK_BOT_6) as usize]);

            b.set_len(new_len);
        }

        8 => unsafe {
            let end = b.as_mut_ptr().add(len);

            write(end        , enc[((n >> WIDTH_7) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 1), enc[((n >> WIDTH_6) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 2), enc[((n >> WIDTH_5) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 3), enc[((n >> WIDTH_4) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 4), enc[((n >> WIDTH_3) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 5), enc[((n >> WIDTH_2) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 6), enc[((n >> WIDTH_1) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 7), enc[( n             as u8 & U8_MASK_BOT_6) as usize]);

            b.set_len(new_len);
        }

        7 => unsafe {
            let end = b.as_mut_ptr().add(len);

            write(end        , enc[((n >> WIDTH_6) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 1), enc[((n >> WIDTH_5) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 2), enc[((n >> WIDTH_4) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 3), enc[((n >> WIDTH_3) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 4), enc[((n >> WIDTH_2) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 5), enc[((n >> WIDTH_1) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 6), enc[( n             as u8 & U8_MASK_BOT_6) as usize]);

            b.set_len(new_len);
        }

        6 => unsafe {
            let end = b.as_mut_ptr().add(len);

            write(end        , enc[((n >> WIDTH_5) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 1), enc[((n >> WIDTH_4) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 2), enc[((n >> WIDTH_3) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 3), enc[((n >> WIDTH_2) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 4), enc[((n >> WIDTH_1) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 5), enc[( n             as u8 & U8_MASK_BOT_6) as usize]);

            b.set_len(new_len);
        }

        5 => unsafe {
            let end = b.as_mut_ptr().add(len);

            write(end        , enc[((n >> WIDTH_4) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 1), enc[((n >> WIDTH_3) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 2), enc[((n >> WIDTH_2) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 3), enc[((n >> WIDTH_1) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 4), enc[( n             as u8 & U8_MASK_BOT_6) as usize]);

            b.set_len(new_len);
        }

        4 => unsafe {
            let end = b.as_mut_ptr().add(len);

            write(end        , enc[((n >> WIDTH_3) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 1), enc[((n >> WIDTH_2) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 2), enc[((n >> WIDTH_1) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 3), enc[( n             as u8 & U8_MASK_BOT_6) as usize]);

            b.set_len(new_len);
        }

        3 => unsafe {
            let end = b.as_mut_ptr().add(len);

            write(end        , enc[((n >> WIDTH_2) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 1), enc[((n >> WIDTH_1) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 2), enc[( n             as u8 & U8_MASK_BOT_6) as usize]);

            b.set_len(new_len);
        }

        2 => unsafe {
            let end = b.as_mut_ptr().add(len);

            write(end        , enc[((n >> WIDTH_1) as u8 & U8_MASK_BOT_6) as usize]);
            write(end.add( 1), enc[( n             as u8 & U8_MASK_BOT_6) as usize]);

            b.set_len(new_len);
        }

        1 => unsafe {
            let end = b.as_mut_ptr();

            write(end        , enc[( n             as u8 & U8_MASK_BOT_6) as usize]);

            b.set_len(new_len);
        }

        _ => panic!("Inconceivable! Impossible length for u128")
    }
}
