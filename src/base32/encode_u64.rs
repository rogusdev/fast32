use core::ptr::write;

use crate::shared::U8_MASK_BOT_5;

use super::alphabet::{BITS, WIDTH_1, WIDTH_2, WIDTH_3, WIDTH_4, WIDTH_5, WIDTH_6, WIDTH_7, WIDTH_8, WIDTH_9, WIDTH_10, WIDTH_11, WIDTH_12};

#[rustfmt::skip]
pub fn encode_u64(enc: &'static [u8; BITS], n: u64) -> String {
    // need this to not panic on ilog2
    if n == 0 {
        return (enc[0] as char).to_string()
    }

    let cap = 1 + (n.ilog2() / 5) as usize;
    let mut b = Vec::<u8>::with_capacity(cap);

    match cap {
        13 => unsafe {
            let end = b.as_mut_ptr();

            // this top char is why u64 is diff output from normal bytes based base32
            // (other than exactly 5 byte numbers):
            // for integer values (identifiers) we use remainder at the high bits, rather than low
            write(end        , enc[((n >> WIDTH_12) as u8                ) as usize]);
            write(end.add( 1), enc[((n >> WIDTH_11) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 2), enc[((n >> WIDTH_10) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 3), enc[((n >> WIDTH_9) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 4), enc[((n >> WIDTH_8) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 5), enc[((n >> WIDTH_7) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 6), enc[((n >> WIDTH_6) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 7), enc[((n >> WIDTH_5) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 8), enc[((n >> WIDTH_4) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 9), enc[((n >> WIDTH_3) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add(10), enc[((n >> WIDTH_2) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add(11), enc[((n >> WIDTH_1) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add(12), enc[( n             as u8 & U8_MASK_BOT_5) as usize]);

            b.set_len(cap);
        }

        12 => unsafe {
            let end = b.as_mut_ptr();

            write(end        , enc[((n >> WIDTH_11) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 1), enc[((n >> WIDTH_10) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 2), enc[((n >> WIDTH_9) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 3), enc[((n >> WIDTH_8) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 4), enc[((n >> WIDTH_7) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 5), enc[((n >> WIDTH_6) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 6), enc[((n >> WIDTH_5) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 7), enc[((n >> WIDTH_4) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 8), enc[((n >> WIDTH_3) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 9), enc[((n >> WIDTH_2) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add(10), enc[((n >> WIDTH_1) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add(11), enc[( n             as u8 & U8_MASK_BOT_5) as usize]);

            b.set_len(cap);
        }

        11 => unsafe {
            let end = b.as_mut_ptr();

            write(end        , enc[((n >> WIDTH_10) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 1), enc[((n >> WIDTH_9) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 2), enc[((n >> WIDTH_8) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 3), enc[((n >> WIDTH_7) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 4), enc[((n >> WIDTH_6) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 5), enc[((n >> WIDTH_5) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 6), enc[((n >> WIDTH_4) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 7), enc[((n >> WIDTH_3) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 8), enc[((n >> WIDTH_2) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 9), enc[((n >> WIDTH_1) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add(10), enc[( n             as u8 & U8_MASK_BOT_5) as usize]);

            b.set_len(cap);
        }

        10 => unsafe {
            let end = b.as_mut_ptr();

            write(end        , enc[((n >> WIDTH_9) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 1), enc[((n >> WIDTH_8) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 2), enc[((n >> WIDTH_7) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 3), enc[((n >> WIDTH_6) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 4), enc[((n >> WIDTH_5) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 5), enc[((n >> WIDTH_4) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 6), enc[((n >> WIDTH_3) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 7), enc[((n >> WIDTH_2) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 8), enc[((n >> WIDTH_1) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 9), enc[( n             as u8 & U8_MASK_BOT_5) as usize]);

            b.set_len(cap);
        }

        9 => unsafe {
            let end = b.as_mut_ptr();

            write(end        , enc[((n >> WIDTH_8) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 1), enc[((n >> WIDTH_7) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 2), enc[((n >> WIDTH_6) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 3), enc[((n >> WIDTH_5) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 4), enc[((n >> WIDTH_4) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 5), enc[((n >> WIDTH_3) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 6), enc[((n >> WIDTH_2) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 7), enc[((n >> WIDTH_1) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 8), enc[( n             as u8 & U8_MASK_BOT_5) as usize]);

            b.set_len(cap);
        }

        8 => unsafe {
            let end = b.as_mut_ptr();

            write(end        , enc[((n >> WIDTH_7) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 1), enc[((n >> WIDTH_6) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 2), enc[((n >> WIDTH_5) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 3), enc[((n >> WIDTH_4) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 4), enc[((n >> WIDTH_3) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 5), enc[((n >> WIDTH_2) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 6), enc[((n >> WIDTH_1) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 7), enc[( n             as u8 & U8_MASK_BOT_5) as usize]);

            b.set_len(cap);
        }

        7 => unsafe {
            let end = b.as_mut_ptr();

            write(end        , enc[((n >> WIDTH_6) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 1), enc[((n >> WIDTH_5) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 2), enc[((n >> WIDTH_4) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 3), enc[((n >> WIDTH_3) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 4), enc[((n >> WIDTH_2) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 5), enc[((n >> WIDTH_1) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 6), enc[( n             as u8 & U8_MASK_BOT_5) as usize]);

            b.set_len(cap);
        }

        6 => unsafe {
            let end = b.as_mut_ptr();

            write(end        , enc[((n >> WIDTH_5) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 1), enc[((n >> WIDTH_4) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 2), enc[((n >> WIDTH_3) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 3), enc[((n >> WIDTH_2) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 4), enc[((n >> WIDTH_1) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 5), enc[( n             as u8 & U8_MASK_BOT_5) as usize]);

            b.set_len(cap);
        }

        5 => unsafe {
            let end = b.as_mut_ptr();

            write(end        , enc[((n >> WIDTH_4) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 1), enc[((n >> WIDTH_3) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 2), enc[((n >> WIDTH_2) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 3), enc[((n >> WIDTH_1) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 4), enc[( n             as u8 & U8_MASK_BOT_5) as usize]);

            b.set_len(cap);
        }

        4 => unsafe {
            let end = b.as_mut_ptr();

            write(end        , enc[((n >> WIDTH_3) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 1), enc[((n >> WIDTH_2) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 2), enc[((n >> WIDTH_1) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 3), enc[( n             as u8 & U8_MASK_BOT_5) as usize]);

            b.set_len(cap);
        }

        3 => unsafe {
            let end = b.as_mut_ptr();

            write(end        , enc[((n >> WIDTH_2) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 1), enc[((n >> WIDTH_1) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 2), enc[( n             as u8 & U8_MASK_BOT_5) as usize]);

            b.set_len(cap);
        }

        2 => unsafe {
            let end = b.as_mut_ptr();

            write(end        , enc[((n >> WIDTH_1) as u8 & U8_MASK_BOT_5) as usize]);
            write(end.add( 1), enc[( n             as u8 & U8_MASK_BOT_5) as usize]);

            b.set_len(cap);
        }

        _ => unsafe {
            let end = b.as_mut_ptr();

            write(end        , enc[( n             as u8 & U8_MASK_BOT_5) as usize]);

            b.set_len(1);
        }
    }

    unsafe { String::from_utf8_unchecked(b) }
}
