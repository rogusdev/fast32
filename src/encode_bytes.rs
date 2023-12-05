use core::ptr::write;

use crate::alphabet::ENC_CROCKFORD_UPPER;
use crate::encode_base::*;

pub fn encode_bytes(a: &[u8]) -> String {
    let enc = ENC_CROCKFORD_UPPER;

    let len = a.len();
    let rem = len % 5;
    // https://stackoverflow.com/questions/23636240/how-do-i-predict-the-required-size-of-a-base32-decode-output
    let cap = (len * 8 + 4) / 5;
    let mut b = Vec::<u8>::with_capacity(cap);
    let max = cap / 8;

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
            let c = i * 5;
            let p = i * 8;
            let end = b.as_mut_ptr().add(p);

            write(end       , enc[( (a[c  ] & B32_MASK_TOP_5) >> 3                                    ) as usize]);
            write(end.add(1), enc[(((a[c  ] & B32_MASK_BOT_3) << 2) | ((a[c+1] & B32_MASK_TOP_2) >> 6)) as usize]);
            write(end.add(2), enc[( (a[c+1] & B32_MASK_MID_1) >> 1                                    ) as usize]);
            write(end.add(3), enc[(((a[c+1] & B32_MASK_BOT_1) << 4) | ((a[c+2] & B32_MASK_TOP_4) >> 4)) as usize]);
            write(end.add(4), enc[(((a[c+2] & B32_MASK_BOT_4) << 1) | ((a[c+3] & B32_MASK_TOP_1) >> 7)) as usize]);
            write(end.add(5), enc[( (a[c+3] & B32_MASK_MID_2) >> 2                                    ) as usize]);
            write(end.add(6), enc[(((a[c+3] & B32_MASK_BOT_2) << 3) | ((a[c+4] & B32_MASK_TOP_3) >> 5)) as usize]);
            write(end.add(7), enc[(  a[c+4] & B32_MASK_BOT_5                                          ) as usize]);

            b.set_len(p + 8);
        }
    }

    match rem {
        4 => unsafe {
            let c = max * 5;
            let p = max * 8;
            let end = b.as_mut_ptr().add(p);

            write(end       , enc[( (a[c  ] & B32_MASK_TOP_5) >> 3                                    ) as usize]);
            write(end.add(1), enc[(((a[c  ] & B32_MASK_BOT_3) << 2) | ((a[c+1] & B32_MASK_TOP_2) >> 6)) as usize]);
            write(end.add(2), enc[( (a[c+1] & B32_MASK_MID_1) >> 1                                    ) as usize]);
            write(end.add(3), enc[(((a[c+1] & B32_MASK_BOT_1) << 4) | ((a[c+2] & B32_MASK_TOP_4) >> 4)) as usize]);
            write(end.add(4), enc[(((a[c+2] & B32_MASK_BOT_4) << 1) | ((a[c+3] & B32_MASK_TOP_1) >> 7)) as usize]);
            write(end.add(5), enc[( (a[c+3] & B32_MASK_MID_2) >> 2                                    ) as usize]);
            write(end.add(6), enc[(((a[c+3] & B32_MASK_BOT_2) << 3)                                   ) as usize]);

            b.set_len(p + 7);
        }
        3 => unsafe {
            let c = max * 5;
            let p = max * 8;
            let end = b.as_mut_ptr().add(p);

            write(end       , enc[( (a[c  ] & B32_MASK_TOP_5) >> 3                                    ) as usize]);
            write(end.add(1), enc[(((a[c  ] & B32_MASK_BOT_3) << 2) | ((a[c+1] & B32_MASK_TOP_2) >> 6)) as usize]);
            write(end.add(2), enc[( (a[c+1] & B32_MASK_MID_1) >> 1                                    ) as usize]);
            write(end.add(3), enc[(((a[c+1] & B32_MASK_BOT_1) << 4) | ((a[c+2] & B32_MASK_TOP_4) >> 4)) as usize]);
            write(end.add(4), enc[(((a[c+2] & B32_MASK_BOT_4) << 1)                                   ) as usize]);

            b.set_len(p + 5);
        }
        2 => unsafe {
            let c = max * 5;
            let p = max * 8;
            let end = b.as_mut_ptr().add(p);

            write(end       , enc[( (a[c  ] & B32_MASK_TOP_5) >> 3                                    ) as usize]);
            write(end.add(1), enc[(((a[c  ] & B32_MASK_BOT_3) << 2) | ((a[c+1] & B32_MASK_TOP_2) >> 6)) as usize]);
            write(end.add(2), enc[( (a[c+1] & B32_MASK_MID_1) >> 1                                    ) as usize]);
            write(end.add(3), enc[(((a[c+1] & B32_MASK_BOT_1) << 4)                                   ) as usize]);

            b.set_len(p + 4);
        }
        1 => unsafe {
            let c = max * 5;
            let p = max * 8;
            let end = b.as_mut_ptr().add(p);

            write(end       , enc[( (a[c  ] & B32_MASK_TOP_5) >> 3                                    ) as usize]);
            write(end.add(1), enc[(((a[c  ] & B32_MASK_BOT_3) << 2)                                   ) as usize]);

            b.set_len(p + 2);
        }
        _ => {}
    }

    unsafe { String::from_utf8_unchecked(b) }
}
