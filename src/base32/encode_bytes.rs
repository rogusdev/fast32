// unsafe writing adapted from https://doc.rust-lang.org/src/alloc/vec/mod.rs.html#1878
use core::ptr::write;

use crate::shared::*;

use super::alphabet::{BITS, WIDTH_DEC, WIDTH_ENC};

const U8_MASK_MID_2: u8 = 0b01111100;
const U8_MASK_MID_1: u8 = 0b00111110;

/// Capacity needed in dest `Vec<u8>` to encode this byte array
#[inline]
pub const fn capacity_encode(a: &[u8]) -> usize {
    // https://stackoverflow.com/questions/23636240/how-do-i-predict-the-required-size-of-a-base32-decode-output
    (a.len() * WIDTH_ENC + (WIDTH_DEC - 1)) / WIDTH_DEC
}

#[inline]
const fn rem_enc(rem: usize) -> usize {
    match rem {
        4 => 7,
        3 => 5,
        2 => 4,
        1 => 2,
        _ => 0,
    }
}

/// Encode byte array with given encoding, into a `String`
///
/// Example:
/// ```
/// use fast32::base32::RFC4648;
/// assert_eq!(RFC4648.encode_bytes(&[0x00, 0x12]), "AAJA====");
/// ```
pub fn encode_bytes(enc: &'static [u8; BITS], a: &[u8]) -> String {
    let len_dec = a.len();
    let rem = len_dec % WIDTH_DEC;
    let max = len_dec / WIDTH_DEC;

    let p_max = max * WIDTH_ENC;
    let rem_enc = rem_enc(rem);

    let mut b = Vec::<u8>::with_capacity(p_max + rem_enc);

    encode_bytes_inner(enc, a, &mut b, max, 0, p_max, rem, rem_enc);

    unsafe { String::from_utf8_unchecked(b) }
}

/// Encode byte array with given encoding, into an existing `Vec<u8>`
///
/// Example:
/// ```
/// use fast32::base32::RFC4648;
/// let mut b = Vec::<u8>::with_capacity(8);
/// RFC4648.encode_bytes_into(&[0x00, 0x12], &mut b);
/// assert_eq!(&b, b"AAJA====");
/// ```
///
/// Panics if not enough capacity in `b` for encoding -- see [`capacity_encode`](self::capacity_encode())
pub fn encode_bytes_into(enc: &'static [u8; BITS], a: &[u8], b: &mut Vec<u8>) {
    let len_dec = a.len();
    let rem = len_dec % WIDTH_DEC;
    let max = len_dec / WIDTH_DEC;

    let len_enc = b.len();
    let p_max = len_enc + max * WIDTH_ENC;
    let rem_enc = rem_enc(rem);

    assert!(
        b.capacity() >= p_max + rem_enc,
        "Missing capacity for encoding"
    );

    encode_bytes_inner(enc, a, b, max, len_enc, p_max, rem, rem_enc);
}

#[rustfmt::skip]
#[inline(always)]
fn encode_bytes_inner(enc: &'static [u8; BITS], a: &[u8], b: &mut Vec<u8>, max: usize, len_enc: usize, p_max: usize, rem: usize, rem_enc: usize) {
    // 5 bytes is 8 chars exactly
    // 4 bytes is 7 chars
    // 3 bytes is 5 chars
    // 2 bytes is 4 chars
    // 1 bytes is 2 chars

    // 6 bytes is 10 chars
    // 7 bytes is 12 chars
    // 8 bytes is 13 chars
    // 9 bytes is 15 chars

    // (src_size * 8 + 4) / 5
    // (1 * 8 + 4) / 5 == 2
    // (2 * 8 + 4) / 5 == 4
    // (3 * 8 + 4) / 5 == 5
    // (4 * 8 + 4) / 5 == 7
    // (5 * 8 + 4) / 5 == 8
    // (6 * 8 + 4) / 5 == 10
    // (7 * 8 + 4) / 5 == 12
    // (8 * 8 + 4) / 5 == 13
    // (9 * 8 + 4) / 5 == 15

    // (dst_size * 5) / 8
    // (2 * 5) / 8 == 1
    // (4 * 5) / 8 == 2
    // (5 * 5) / 8 == 3
    // (7 * 5) / 8 == 4
    // (8 * 5) / 8 == 5
    // (10 * 5) / 8 == 6
    // (12 * 5) / 8 == 7
    // (13 * 5) / 8 == 8
    // (15 * 5) / 8 == 9

    for i in 0..max {
        // adapted from https://doc.rust-lang.org/src/alloc/vec/mod.rs.html#1878
        unsafe {
            let c = i * WIDTH_DEC;
            let p = len_enc + i * WIDTH_ENC;
            let end = b.as_mut_ptr().add(p);

            write(end       , enc[( (a[c  ] & U8_MASK_TOP_5) >> 3                                   ) as usize]);
            write(end.add(1), enc[(((a[c  ] & U8_MASK_BOT_3) << 2) | ((a[c+1] & U8_MASK_TOP_2) >> 6)) as usize]);
            write(end.add(2), enc[( (a[c+1] & U8_MASK_MID_1) >> 1                                   ) as usize]);
            write(end.add(3), enc[(((a[c+1] & U8_MASK_BOT_1) << 4) | ((a[c+2] & U8_MASK_TOP_4) >> 4)) as usize]);
            write(end.add(4), enc[(((a[c+2] & U8_MASK_BOT_4) << 1) | ((a[c+3] & U8_MASK_TOP_1) >> 7)) as usize]);
            write(end.add(5), enc[( (a[c+3] & U8_MASK_MID_2) >> 2                                   ) as usize]);
            write(end.add(6), enc[(((a[c+3] & U8_MASK_BOT_2) << 3) | ((a[c+4] & U8_MASK_TOP_3) >> 5)) as usize]);
            write(end.add(7), enc[(  a[c+4] & U8_MASK_BOT_5                                         ) as usize]);

            b.set_len(p + WIDTH_ENC);
        }
    }

    match rem {
        4 => unsafe {
            let c = max * WIDTH_DEC;
            let end = b.as_mut_ptr().add(p_max);

            write(end       , enc[( (a[c  ] & U8_MASK_TOP_5) >> 3                                   ) as usize]);
            write(end.add(1), enc[(((a[c  ] & U8_MASK_BOT_3) << 2) | ((a[c+1] & U8_MASK_TOP_2) >> 6)) as usize]);
            write(end.add(2), enc[( (a[c+1] & U8_MASK_MID_1) >> 1                                   ) as usize]);
            write(end.add(3), enc[(((a[c+1] & U8_MASK_BOT_1) << 4) | ((a[c+2] & U8_MASK_TOP_4) >> 4)) as usize]);
            write(end.add(4), enc[(((a[c+2] & U8_MASK_BOT_4) << 1) | ((a[c+3] & U8_MASK_TOP_1) >> 7)) as usize]);
            write(end.add(5), enc[( (a[c+3] & U8_MASK_MID_2) >> 2                                   ) as usize]);
            write(end.add(6), enc[(((a[c+3] & U8_MASK_BOT_2) << 3)                                  ) as usize]);

            b.set_len(p_max + rem_enc);
        }
        3 => unsafe {
            let c = max * WIDTH_DEC;
            let end = b.as_mut_ptr().add(p_max);

            write(end       , enc[( (a[c  ] & U8_MASK_TOP_5) >> 3                                   ) as usize]);
            write(end.add(1), enc[(((a[c  ] & U8_MASK_BOT_3) << 2) | ((a[c+1] & U8_MASK_TOP_2) >> 6)) as usize]);
            write(end.add(2), enc[( (a[c+1] & U8_MASK_MID_1) >> 1                                   ) as usize]);
            write(end.add(3), enc[(((a[c+1] & U8_MASK_BOT_1) << 4) | ((a[c+2] & U8_MASK_TOP_4) >> 4)) as usize]);
            write(end.add(4), enc[(((a[c+2] & U8_MASK_BOT_4) << 1)                                  ) as usize]);

            b.set_len(p_max + rem_enc);
        }
        2 => unsafe {
            let c = max * WIDTH_DEC;
            let end = b.as_mut_ptr().add(p_max);

            write(end       , enc[( (a[c  ] & U8_MASK_TOP_5) >> 3                                   ) as usize]);
            write(end.add(1), enc[(((a[c  ] & U8_MASK_BOT_3) << 2) | ((a[c+1] & U8_MASK_TOP_2) >> 6)) as usize]);
            write(end.add(2), enc[( (a[c+1] & U8_MASK_MID_1) >> 1                                   ) as usize]);
            write(end.add(3), enc[(((a[c+1] & U8_MASK_BOT_1) << 4)                                  ) as usize]);

            b.set_len(p_max + rem_enc);
        }
        1 => unsafe {
            let c = max * WIDTH_DEC;
            let end = b.as_mut_ptr().add(p_max);

            write(end       , enc[( (a[c  ] & U8_MASK_TOP_5) >> 3                                   ) as usize]);
            write(end.add(1), enc[(((a[c  ] & U8_MASK_BOT_3) << 2)                                  ) as usize]);

            b.set_len(p_max + rem_enc);
        }
        _ => {}
    }
}
