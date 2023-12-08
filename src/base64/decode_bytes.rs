use core::ptr::write;

use crate::shared::{bits_or_err_u8, DecodeError};

use super::alphabet::{WIDTH_IN, WIDTH_OUT};

pub fn decode_bytes_str(
    dec: &'static [u8; 256],
    a: impl AsRef<str>,
) -> Result<Vec<u8>, DecodeError> {
    decode_bytes(dec, a.as_ref().as_bytes())
}

pub fn decode_bytes(dec: &'static [u8; 256], a: &[u8]) -> Result<Vec<u8>, DecodeError> {
    let cap = a.len() * WIDTH_IN / WIDTH_OUT;
    let mut b = Vec::<u8>::with_capacity(cap);
    decode_bytes_into(dec, a, &mut b)?;
    Ok(b)
}

#[rustfmt::skip]
pub fn decode_bytes_into(dec: &'static [u8; 256], a: &[u8], b: &mut Vec<u8>) -> Result<(), DecodeError> {
    let len_enc = a.len();
    let rem = len_enc % WIDTH_OUT;
    let max = len_enc / WIDTH_OUT;
    let len_dec = b.len();

    for i in 0..max {
        let c = len_dec + i * WIDTH_IN;
        let p = i * WIDTH_OUT;

        let p1 = bits_or_err_u8(dec, a, p  )?;
        let p2 = bits_or_err_u8(dec, a, p+1)?;
        let p3 = bits_or_err_u8(dec, a, p+2)?;
        let p4 = bits_or_err_u8(dec, a, p+3)?;

        unsafe {
            let end = b.as_mut_ptr().add(c);

            write(end       , (p1 << 2) | (p2 >> 4));
            write(end.add(1), (p2 << 4) | (p3 >> 2));
            write(end.add(2), (p3 << 6) |  p4      );

            b.set_len(c + WIDTH_IN);
        }
    }

    match rem {
        3 => {
            let c = len_dec + max * WIDTH_IN;
            let p = max * WIDTH_OUT;

            let p1 = bits_or_err_u8(dec, a, p  )?;
            let p2 = bits_or_err_u8(dec, a, p+1)?;
            let p3 = bits_or_err_u8(dec, a, p+2)?;

            if p3 & 0b0011 != 0 {
                Err(DecodeError::InvalidChar { char: a[p+2] as char, index: p+2 })?
            }

            unsafe {
                let end = b.as_mut_ptr().add(c);

                write(end       , (p1 << 2) | (p2 >> 4));
                write(end.add(1), (p2 << 4) | (p3 >> 2));

                b.set_len(c + 2);
            }
        }
        2 => {
            let c = len_dec + max * WIDTH_IN;
            let p = max * WIDTH_OUT;

            let p1 = bits_or_err_u8(dec, a, p  )?;
            let p2 = bits_or_err_u8(dec, a, p+1)?;

            if p2 & 0b1111 != 0 {
                Err(DecodeError::InvalidChar { char: a[p+1] as char, index: p+1 })?
            }

            unsafe {
                let end = b.as_mut_ptr().add(c);

                write(end       , (p1 << 2) | (p2 >> 4));

                b.set_len(c + 1);
            }
        }
        0 => {}
        _ => Err(DecodeError::InvalidLength { length: len_enc })?
    }

    Ok(())
}
