use core::ptr::write;

use crate::shared::{bits_or_err_u8, DecodeError};

use super::alphabet::{WIDTH_DEC, WIDTH_ENC};

/// Capacity needed in dest `Vec<u8>` to decode this byte array
#[inline]
pub const fn capacity_decode(a: &[u8]) -> usize {
    a.len() * WIDTH_DEC / WIDTH_ENC
}

#[inline]
const fn rem_dec(rem: usize) -> usize {
    match rem {
        3 => 2,
        2 => 1,
        _ => 0,
    }
}

/// Decode byte array with given decoding, into a `Vec<u8>`
///
/// Example:
/// ```
/// use fast32::base64::RFC4648;
/// assert_eq!(RFC4648.decode(b"ABI=").unwrap(), &[0x00, 0x12]);
/// ```
///
/// Returns [`DecodeError`] if input to decode is invalid
pub fn decode(dec: &'static [u8; 256], a: &[u8]) -> Result<Vec<u8>, DecodeError> {
    let len_enc = a.len();
    let rem = len_enc % WIDTH_ENC;
    let max = len_enc / WIDTH_ENC;

    let c_max = max * WIDTH_DEC;
    let rem_dec = rem_dec(rem);

    let mut b = Vec::<u8>::with_capacity(c_max + rem_dec);

    decode_inner(dec, a, &mut b, max, 0, c_max, rem, rem_dec, len_enc)?;

    Ok(b)
}

/// Decode byte array with given decoding, into an existing `Vec<u8>`
///
/// Example:
/// ```
/// use fast32::base64::RFC4648;
/// let mut b = Vec::<u8>::with_capacity(2);
/// RFC4648.decode_into(b"ABI=", &mut b);
/// assert_eq!(&b, &[0x00, 0x12]);
/// ```
///
/// Returns [`DecodeError`] if input to decode is invalid
///
/// Panics if not enough capacity in `b` for decoding -- see [`capacity_decode`](self::capacity_decode())
pub fn decode_into(dec: &'static [u8; 256], a: &[u8], b: &mut Vec<u8>) -> Result<(), DecodeError> {
    let len_enc = a.len();
    let rem = len_enc % WIDTH_ENC;
    let max = len_enc / WIDTH_ENC;

    let len_dec = b.len();
    let c_max = len_dec + max * WIDTH_DEC;
    let rem_dec = rem_dec(rem);

    assert!(
        b.capacity() >= c_max + rem_dec,
        "Missing capacity for decoding"
    );

    decode_inner(dec, a, b, max, len_dec, c_max, rem, rem_dec, len_enc)
}

#[rustfmt::skip]
#[inline(always)]
fn decode_inner(dec: &'static [u8; 256], a: &[u8], b: &mut Vec<u8>, max: usize, len_dec: usize, c_max: usize, rem: usize, rem_dec: usize, len_enc: usize) -> Result<(), DecodeError> {
    for i in 0..max {
        let c = len_dec + i * WIDTH_DEC;
        let p = i * WIDTH_ENC;

        let p1 = bits_or_err_u8(dec, a, p  )?;
        let p2 = bits_or_err_u8(dec, a, p+1)?;
        let p3 = bits_or_err_u8(dec, a, p+2)?;
        let p4 = bits_or_err_u8(dec, a, p+3)?;

        unsafe {
            let end = b.as_mut_ptr().add(c);

            write(end       , (p1 << 2) | (p2 >> 4));
            write(end.add(1), (p2 << 4) | (p3 >> 2));
            write(end.add(2), (p3 << 6) |  p4      );

            b.set_len(c + WIDTH_DEC);
        }
    }

    match rem {
        3 => {
            let p = max * WIDTH_ENC;

            let p1 = bits_or_err_u8(dec, a, p  )?;
            let p2 = bits_or_err_u8(dec, a, p+1)?;
            let p3 = bits_or_err_u8(dec, a, p+2)?;

            if p3 & 0b0011 != 0 {
                Err(DecodeError::InvalidChar { char: a[p+2] as char, index: p+2 })?
            }

            unsafe {
                let end = b.as_mut_ptr().add(c_max);

                write(end       , (p1 << 2) | (p2 >> 4));
                write(end.add(1), (p2 << 4) | (p3 >> 2));

                b.set_len(c_max + rem_dec);
            }
        }
        2 => {
            let p = max * WIDTH_ENC;

            let p1 = bits_or_err_u8(dec, a, p  )?;
            let p2 = bits_or_err_u8(dec, a, p+1)?;

            if p2 & 0b1111 != 0 {
                Err(DecodeError::InvalidChar { char: a[p+1] as char, index: p+1 })?
            }

            unsafe {
                let end = b.as_mut_ptr().add(c_max);

                write(end       , (p1 << 2) | (p2 >> 4));

                b.set_len(c_max + rem_dec);
            }
        }
        0 => {}
        _ => Err(DecodeError::InvalidLength { length: len_enc })?
    }

    Ok(())
}
