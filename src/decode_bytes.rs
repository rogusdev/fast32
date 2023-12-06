use core::ptr::write;

use crate::decode_base::{bits_or_err_u8, DecodeError};

pub fn decode_bytes_str(dec: &'static [u8; 256], a: impl AsRef<str>) -> Result<Vec<u8>, DecodeError> {
    decode_bytes(dec, a.as_ref().as_bytes())
}

pub fn decode_bytes(dec: &'static [u8; 256], a: &[u8]) -> Result<Vec<u8>, DecodeError> {
    let len = a.len();
    // TODO? optionally count padding as rem (it's the same thing)
    let rem = len % 8;
    let cap = (len * 5) / 8;
    let mut b = Vec::<u8>::with_capacity(cap);
    let max = cap / 5;

    for i in 0..max {
        let c = i * 5;
        let p = i * 8;

        let p1 = bits_or_err_u8(dec, a, p  )?;
        let p2 = bits_or_err_u8(dec, a, p+1)?;
        let p3 = bits_or_err_u8(dec, a, p+2)?;
        let p4 = bits_or_err_u8(dec, a, p+3)?;
        let p5 = bits_or_err_u8(dec, a, p+4)?;
        let p6 = bits_or_err_u8(dec, a, p+5)?;
        let p7 = bits_or_err_u8(dec, a, p+6)?;
        let p8 = bits_or_err_u8(dec, a, p+7)?;

        unsafe {
            let end = b.as_mut_ptr().add(c);

            write(end       , (p1 << 3) | (p2 >> 2)            );
            write(end.add(1), (p2 << 6) | (p3 << 1) | (p4 >> 4));
            write(end.add(2), (p4 << 4) | (p5 >> 1)            );
            write(end.add(3), (p5 << 7) | (p6 << 2) | (p7 >> 3));
            write(end.add(4), (p7 << 5) | (p8     )            );

            b.set_len(c + 5);
        }
    }

    match rem {
        7 => {
            let c = max * 5;
            let p = max * 8;

            let p1 = bits_or_err_u8(dec, a, p  )?;
            let p2 = bits_or_err_u8(dec, a, p+1)?;
            let p3 = bits_or_err_u8(dec, a, p+2)?;
            let p4 = bits_or_err_u8(dec, a, p+3)?;
            let p5 = bits_or_err_u8(dec, a, p+4)?;
            let p6 = bits_or_err_u8(dec, a, p+5)?;
            let p7 = bits_or_err_u8(dec, a, p+6)?;

            if p7 & 0b0111 != 0 {
                Err(DecodeError::InvalidChar { char: a[p+6] as char, index: p+6 })?
            }

            unsafe {
                let end = b.as_mut_ptr().add(c);

                write(end       , (p1 << 3) | (p2 >> 2)            );
                write(end.add(1), (p2 << 6) | (p3 << 1) | (p4 >> 4));
                write(end.add(2), (p4 << 4) | (p5 >> 1)            );
                write(end.add(3), (p5 << 7) | (p6 << 2) | (p7 >> 3));

                b.set_len(c + 4);
            }
        }
        5 => {
            let c = max * 5;
            let p = max * 8;

            let p1 = bits_or_err_u8(dec, a, p  )?;
            let p2 = bits_or_err_u8(dec, a, p+1)?;
            let p3 = bits_or_err_u8(dec, a, p+2)?;
            let p4 = bits_or_err_u8(dec, a, p+3)?;
            let p5 = bits_or_err_u8(dec, a, p+4)?;

            if p5 & 0b0001 != 0 {
                Err(DecodeError::InvalidChar { char: a[p+4] as char, index: p+4 })?
            }

            unsafe {
                let end = b.as_mut_ptr().add(c);

                write(end       , (p1 << 3) | (p2 >> 2)            );
                write(end.add(1), (p2 << 6) | (p3 << 1) | (p4 >> 4));
                write(end.add(2), (p4 << 4) | (p5 >> 1)            );

                b.set_len(c + 3);
            }
        }
        4 => {
            let c = max * 5;
            let p = max * 8;

            let p1 = bits_or_err_u8(dec, a, p  )?;
            let p2 = bits_or_err_u8(dec, a, p+1)?;
            let p3 = bits_or_err_u8(dec, a, p+2)?;
            let p4 = bits_or_err_u8(dec, a, p+3)?;

            if p4 & 0b1111 != 0 {
                Err(DecodeError::InvalidChar { char: a[p+3] as char, index: p+3 })?
            }

            unsafe {
                let end = b.as_mut_ptr().add(c);

                write(end       , (p1 << 3) | (p2 >> 2)            );
                write(end.add(1), (p2 << 6) | (p3 << 1) | (p4 >> 4));

                b.set_len(c + 2);
            }
        }
        2 => {
            let c = max * 5;
            let p = max * 8;

            let p1 = bits_or_err_u8(dec, a, p  )?;
            let p2 = bits_or_err_u8(dec, a, p+1)?;

            if p2 & 0b0011 != 0 {
                Err(DecodeError::InvalidChar { char: a[p+1] as char, index: p+1 })?
            }

            unsafe {
                let end = b.as_mut_ptr().add(c);

                write(end       , (p1 << 3) | (p2 >> 2));

                b.set_len(c + 1);
            }
        }
        0 => {}
        _ => Err(DecodeError::InvalidLength { length: len })?
    }

    Ok(b)
}
