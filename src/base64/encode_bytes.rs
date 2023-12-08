use core::ptr::write;

use crate::shared::*;

use super::alphabet::{BITS, WIDTH_DEC, WIDTH_ENC};

pub fn encode_bytes_str(enc: &'static [u8; BITS], a: impl AsRef<str>) -> String {
    encode_bytes(enc, a.as_ref().as_bytes())
}

#[rustfmt::skip]
pub fn encode_bytes(enc: &'static [u8; BITS], a: &[u8]) -> String {
    let len = a.len();
    let rem = len % WIDTH_DEC;
    let cap = (len * WIDTH_ENC + (WIDTH_DEC - 1)) / WIDTH_DEC;
    let mut b = Vec::<u8>::with_capacity(cap);
    let max = cap / WIDTH_ENC;

    for i in 0..max {
        unsafe {
            let c = i * WIDTH_DEC;
            let p = i * WIDTH_ENC;
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
            let p = max * WIDTH_ENC;
            let end = b.as_mut_ptr().add(p);

            write(end       , enc[( (a[c  ] & U8_MASK_TOP_6) >> 2                                   ) as usize]);
            write(end.add(1), enc[(((a[c  ] & U8_MASK_BOT_2) << 4) | ((a[c+1] & U8_MASK_TOP_4) >> 4)) as usize]);
            write(end.add(2), enc[( (a[c+1] & U8_MASK_BOT_4) << 2                                   ) as usize]);

            b.set_len(p + 3);
        }
        1 => unsafe {
            let c = max * WIDTH_DEC;
            let p = max * WIDTH_ENC;
            let end = b.as_mut_ptr().add(p);

            write(end       , enc[( (a[c  ] & U8_MASK_TOP_6) >> 2                                   ) as usize]);
            write(end.add(1), enc[( (a[c  ] & U8_MASK_BOT_2) << 4                                   ) as usize]);

            b.set_len(p + 2);
        }
        _ => {}
    }

    unsafe { String::from_utf8_unchecked(b) }
}
