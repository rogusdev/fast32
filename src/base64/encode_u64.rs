// unsafe writing adapted from https://doc.rust-lang.org/src/alloc/vec/mod.rs.html#1878
use core::ptr::write;

use crate::shared::U8_MASK_BOT_6;

use super::alphabet::{
    BITS, WIDTH_1, WIDTH_10, WIDTH_2, WIDTH_3, WIDTH_4, WIDTH_5, WIDTH_6, WIDTH_7, WIDTH_8, WIDTH_9,
};

/// Capacity required in a `Vec<u8>` to encode this u64
#[inline]
pub const fn capacity_u64(n: u64) -> usize {
    if let Some(log) = n.checked_ilog2() {
        1 + (log / 6) as usize
    } else {
        1
    }
}

/// Encode u64 with given encoding, into a `String`
///
/// Examples:
/// ```
/// use fast32::base64::RFC4648_NOPAD;
/// assert_eq!(RFC4648_NOPAD.encode_u64(31), "f");
/// ```
pub fn encode_u64(enc: &'static [u8; BITS], n: u64) -> String {
    let cap = capacity_u64(n);
    let mut b = Vec::<u8>::with_capacity(cap);
    encode_u64_inner(enc, n, &mut b, cap, 0, cap);
    unsafe { String::from_utf8_unchecked(b) }
}

/// Encode u64 with given encoding, into an existing `Vec<u8>`
///
/// Example:
/// ```
/// use fast32::base64::RFC4648_NOPAD;
/// let mut b = Vec::<u8>::with_capacity(1);
/// RFC4648_NOPAD.encode_u64_into(31, &mut b);
/// assert_eq!(&b, b"f");
/// ```
///
/// Panics if not enough capacity in `b` for encoding -- see [`capacity_u64`](self::capacity_u64())
pub fn encode_u64_into(enc: &'static [u8; BITS], n: u64, b: &mut Vec<u8>) {
    let cap = capacity_u64(n);
    let len = b.len();
    let new_len = len + cap;
    assert!(b.capacity() >= new_len, "Missing capacity for encoding");
    encode_u64_inner(enc, n, b, cap, len, new_len);
}

#[rustfmt::skip]
#[inline(always)]
fn encode_u64_inner(enc: &'static [u8; BITS], n: u64, b: &mut Vec<u8>, cap: usize, len: usize, new_len: usize) {
    match cap {
        11 => unsafe {
            let end = b.as_mut_ptr().add(len);

            // this top char is why u64 is diff output from normal bytes based base64
            // (other than multiples of 3 byte numbers):
            // for integer values (identifiers) we use remainder at the high bits, rather than low
            write(end        , enc[((n >> WIDTH_10) as u8               ) as usize]);
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
            let end = b.as_mut_ptr().add(len);

            write(end        , enc[( n             as u8 & U8_MASK_BOT_6) as usize]);

            b.set_len(new_len);
        }

        _ => panic!("Inconceivable! Impossible length for u64")
    }
}
