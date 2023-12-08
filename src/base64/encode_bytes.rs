// unsafe writing adapted from https://doc.rust-lang.org/src/alloc/vec/mod.rs.html#1878
use core::ptr::write;

use crate::shared::*;

use super::alphabet::{BITS, WIDTH_DEC, WIDTH_ENC};

#[inline]
pub const fn capacity_bytes(a: &[u8]) -> usize {
    // https://stackoverflow.com/questions/23636240/how-do-i-predict-the-required-size-of-a-base32-decode-output
    (a.len() * WIDTH_ENC + (WIDTH_DEC - 1)) / WIDTH_DEC
}

#[inline]
const fn rem_enc(rem: usize) -> usize {
    match rem {
        2 => 3,
        1 => 2,
        _ => 0,
    }
}

pub fn encode_bytes(enc: &'static [u8; BITS], a: &[u8]) -> String {
    let cap = capacity_bytes(a);
    let mut b = Vec::<u8>::with_capacity(cap);
    encode_bytes_into(enc, a, &mut b);
    unsafe { String::from_utf8_unchecked(b) }
}

#[rustfmt::skip]
pub fn encode_bytes_into(enc: &'static [u8; BITS], a: &[u8], b: &mut Vec<u8>) {
    let len_dec = a.len();
    let rem = len_dec % WIDTH_DEC;
    let max = len_dec / WIDTH_DEC;

    let len_enc = b.len();
    let p_max = len_enc + max * WIDTH_ENC;
    let rem_enc = rem_enc(rem);

    assert!(b.capacity() >= p_max + rem_enc, "Missing capacity for encoding");

    for i in 0..max {
        unsafe {
            let c = i * WIDTH_DEC;
            let p = len_enc + i * WIDTH_ENC;
            let end = b.as_mut_ptr().add(p);

            write(end       , enc[( (a[c  ] & U8_MASK_TOP_6) >> 2                                   ) as usize]);
            write(end.add(1), enc[(((a[c  ] & U8_MASK_BOT_2) << 4) | ((a[c+1] & U8_MASK_TOP_4) >> 4)) as usize]);
            write(end.add(2), enc[(((a[c+1] & U8_MASK_BOT_4) << 2) | ((a[c+2] & U8_MASK_TOP_2) >> 6)) as usize]);
            write(end.add(3), enc[( (a[c+2] & U8_MASK_BOT_6)                                        ) as usize]);

            b.set_len(p + WIDTH_ENC);
        }
    }

    match rem {
        2 => unsafe {
            let c = max * WIDTH_DEC;
            let end = b.as_mut_ptr().add(p_max);

            write(end       , enc[( (a[c  ] & U8_MASK_TOP_6) >> 2                                   ) as usize]);
            write(end.add(1), enc[(((a[c  ] & U8_MASK_BOT_2) << 4) | ((a[c+1] & U8_MASK_TOP_4) >> 4)) as usize]);
            write(end.add(2), enc[( (a[c+1] & U8_MASK_BOT_4) << 2                                   ) as usize]);

            b.set_len(p_max + rem_enc);
        }
        1 => unsafe {
            let c = max * WIDTH_DEC;
            let end = b.as_mut_ptr().add(p_max);

            write(end       , enc[( (a[c  ] & U8_MASK_TOP_6) >> 2                                   ) as usize]);
            write(end.add(1), enc[( (a[c  ] & U8_MASK_BOT_2) << 4                                   ) as usize]);

            b.set_len(p_max + rem_enc);
        }
        _ => {}
    }
}
