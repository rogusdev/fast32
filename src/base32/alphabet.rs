use core::ptr::write;

#[cfg(feature = "uuid")]
use uuid::Uuid;

use crate::shared::{decoder_map, decoder_map_simple};
use crate::DecodeError;

use super::decode_bytes::{decode, decode_into};
use super::decode_u128::decode_u128;
use super::decode_u64::decode_u64;
use super::encode_bytes::{encode, encode_into};
use super::encode_u128::{encode_u128, encode_u128_into};
use super::encode_u64::{encode_u64, encode_u64_into};

#[cfg(feature = "uuid")]
use super::uuid::{decode_uuid, encode_uuid};

const ENC_CROCKFORD: &'static [u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";
const DEC_CROCKFORD: [u8; 256] = decoder_map(
    ENC_CROCKFORD,
    b"ILOabcdefghijklmnopqrstvwxyz",
    b"110ABCDEFGH1JK1MN0PQRSTVWXYZ",
);
/// Crockford Base32 (no padding)
///
/// [https://www.crockford.com/base32.html](https://www.crockford.com/base32.html)
pub const CROCKFORD: Alphabet32 = Alphabet32::new(ENC_CROCKFORD, &DEC_CROCKFORD, None);

const ENC_RFC4648: &'static [u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
const DEC_RFC4648: [u8; 256] = decoder_map_simple(ENC_RFC4648);
/// RFC 4648 Base32 normal, with padding
///
/// `"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567"` and `'='`
///
/// [https://datatracker.ietf.org/doc/html/rfc4648#section-6](https://datatracker.ietf.org/doc/html/rfc4648#section-6)
pub const RFC4648: Alphabet32 = Alphabet32::new(ENC_RFC4648, &DEC_RFC4648, Some('='));
/// RFC 4648 Base32 normal, no padding
///
/// `"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567"`
///
/// [https://datatracker.ietf.org/doc/html/rfc4648#section-6](https://datatracker.ietf.org/doc/html/rfc4648#section-6)
pub const RFC4648_NOPAD: Alphabet32 = Alphabet32::new(ENC_RFC4648, &DEC_RFC4648, None);

const ENC_RFC4648_HEX: &'static [u8; 32] = b"0123456789ABCDEFGHIJKLMNOPQRSTUV";
const DEC_RFC4648_HEX: [u8; 256] = decoder_map_simple(ENC_RFC4648_HEX);
/// RFC 4648 Base32 "hex" form, with padding
///
/// `"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567"` and `'='`
///
/// [https://datatracker.ietf.org/doc/html/rfc4648#section-7](https://datatracker.ietf.org/doc/html/rfc4648#section-7)
pub const RFC4648_HEX: Alphabet32 = Alphabet32::new(ENC_RFC4648_HEX, &DEC_RFC4648_HEX, Some('='));
/// RFC 4648 Base32 "hex" form, no padding
///
/// `"0123456789ABCDEFGHIJKLMNOPQRSTUV"`
///
/// [https://datatracker.ietf.org/doc/html/rfc4648#section-7](https://datatracker.ietf.org/doc/html/rfc4648#section-7)
pub const RFC4648_HEX_NOPAD: Alphabet32 = Alphabet32::new(ENC_RFC4648_HEX, &DEC_RFC4648_HEX, None);

pub const BITS: usize = 32;

pub const WIDTH_DEC: usize = 5;
pub const WIDTH_ENC: usize = 8;

pub const WIDTH_1: usize = 5;
pub const WIDTH_2: usize = 10;
pub const WIDTH_3: usize = 15;
pub const WIDTH_4: usize = 20;
pub const WIDTH_5: usize = 25;
pub const WIDTH_6: usize = 30;
pub const WIDTH_7: usize = 35;
pub const WIDTH_8: usize = 40;
pub const WIDTH_9: usize = 45;
pub const WIDTH_10: usize = 50;
pub const WIDTH_11: usize = 55;
pub const WIDTH_12: usize = 60;
pub const WIDTH_13: usize = 65;
pub const WIDTH_14: usize = 70;
pub const WIDTH_15: usize = 75;
pub const WIDTH_16: usize = 80;
pub const WIDTH_17: usize = 85;
pub const WIDTH_18: usize = 90;
pub const WIDTH_19: usize = 95;
pub const WIDTH_20: usize = 100;
pub const WIDTH_21: usize = 105;
pub const WIDTH_22: usize = 110;
pub const WIDTH_23: usize = 115;
pub const WIDTH_24: usize = 120;
pub const WIDTH_25: usize = 125;

unsafe fn add_pad(b: &mut Vec<u8>, pad: char) {
    let len = b.len();
    match len % WIDTH_ENC {
        2 => {
            assert!(b.capacity() >= len + 6, "Missing capacity for padding");
            let end = b.as_mut_ptr().add(len);

            write(end, pad as u8);
            write(end.add(1), pad as u8);
            write(end.add(2), pad as u8);
            write(end.add(3), pad as u8);
            write(end.add(4), pad as u8);
            write(end.add(5), pad as u8);

            b.set_len(len + 6);
        }
        4 => {
            assert!(b.capacity() >= len + 4, "Missing capacity for padding");
            let end = b.as_mut_ptr().add(len);

            write(end, pad as u8);
            write(end.add(1), pad as u8);
            write(end.add(2), pad as u8);
            write(end.add(3), pad as u8);

            b.set_len(len + 4);
        }
        5 => {
            assert!(b.capacity() >= len + 3, "Missing capacity for padding");
            let end = b.as_mut_ptr().add(len);

            write(end, pad as u8);
            write(end.add(1), pad as u8);
            write(end.add(2), pad as u8);

            b.set_len(len + 3);
        }
        7 => {
            assert!(b.capacity() >= len + 1, "Missing capacity for padding");
            let end = b.as_mut_ptr().add(len);

            write(end, pad as u8);

            b.set_len(len + 1);
        }
        _ => {}
    }
}

fn rem_pad(a: &[u8], pad: char) -> &[u8] {
    let len = a.len();
    let pad = pad as u8;
    if len == 0 {
        a
    } else if a[len - 6] == pad {
        &a[..len - 6]
    } else if a[len - 4] == pad {
        &a[..len - 4]
    } else if a[len - 3] == pad {
        &a[..len - 3]
    } else if a[len - 1] == pad {
        &a[..len - 1]
    } else {
        a
    }
}

/// Hold a specific base32 encoding and decoding map pair, with functions to do encoding + decoding
pub struct Alphabet32 {
    enc: &'static [u8; BITS],
    dec: &'static [u8; 256],
    pad: Option<char>,
}

impl Alphabet32 {
    pub const fn new(enc: &'static [u8; BITS], dec: &'static [u8; 256], pad: Option<char>) -> Self {
        Self { enc, dec, pad }
    }

    /// Pass encoder array to [`encode_u64`](super::encode_u64())
    #[inline]
    pub fn encode_u64(&self, n: u64) -> String {
        encode_u64(self.enc, n)
    }

    /// Pass encoder array to [`encode_u128`](super::encode_u128())
    #[inline]
    pub fn encode_u128(&self, n: u128) -> String {
        encode_u128(self.enc, n)
    }

    /// Pass encoder array to [`encode_u64_into`](super::encode_u64_into())
    #[inline]
    pub fn encode_u64_into(&self, n: u64, b: &mut Vec<u8>) {
        encode_u64_into(self.enc, n, b)
    }

    /// Pass encoder array to [`encode_u128_into`](super::encode_u128_into())
    #[inline]
    pub fn encode_u128_into(&self, n: u128, b: &mut Vec<u8>) {
        encode_u128_into(self.enc, n, b)
    }

    /// Pass encoder array to [`encode`](super::encode()), and add padding as needed
    #[inline]
    pub fn encode(&self, a: &[u8]) -> String {
        if let Some(pad) = self.pad {
            let mut s = encode(self.enc, a);
            unsafe {
                let b = s.as_mut_vec();
                // make space for max possible padding
                b.reserve(6);
                add_pad(b, pad);
            }
            s
        } else {
            encode(self.enc, a)
        }
    }

    /// Pass encoder array to [`encode_into`](super::encode_into()), and add padding as needed
    #[inline]
    pub fn encode_into(&self, a: &[u8], b: &mut Vec<u8>) {
        if let Some(pad) = self.pad {
            encode_into(self.enc, a, b);
            unsafe { add_pad(b, pad) }
        } else {
            encode_into(self.enc, a, b)
        }
    }

    /// Pass decoder array to [`decode_u64`](super::decode_u64())
    #[inline]
    pub fn decode_u64(&self, a: &[u8]) -> Result<u64, DecodeError> {
        decode_u64(self.dec, a)
    }

    /// Pass decoder array to [`decode_u128`](super::decode_u128())
    #[inline]
    pub fn decode_u128(&self, a: &[u8]) -> Result<u128, DecodeError> {
        decode_u128(self.dec, a)
    }

    /// Pass decoder array to [`decode`](super::decode()), and remove padding as needed
    #[inline]
    pub fn decode(&self, a: &[u8]) -> Result<Vec<u8>, DecodeError> {
        if let Some(pad) = self.pad {
            decode(self.dec, rem_pad(a, pad))
        } else {
            decode(self.dec, a)
        }
    }

    /// Pass decoder array to [`decode_into`](super::decode_into()), and remove padding as needed
    #[inline]
    pub fn decode_into(&self, a: &[u8], b: &mut Vec<u8>) -> Result<(), DecodeError> {
        if let Some(pad) = self.pad {
            decode_into(self.dec, rem_pad(a, pad), b)
        } else {
            decode_into(self.dec, a, b)
        }
    }

    /// Pass string as bytes and decoder array to [`decode_u64`](super::decode_u64())
    #[inline]
    pub fn decode_u64_str(&self, a: impl AsRef<str>) -> Result<u64, DecodeError> {
        let a = a.as_ref().as_bytes();
        decode_u64(self.dec, a)
    }

    /// Pass string as bytes and decoder array to [`decode_u128`](super::decode_u128())
    #[inline]
    pub fn decode_u128_str(&self, a: impl AsRef<str>) -> Result<u128, DecodeError> {
        let a = a.as_ref().as_bytes();
        decode_u128(self.dec, a)
    }

    /// Pass string as bytes and decoder array to [`decode`](super::decode()), and remove padding as needed
    #[inline]
    pub fn decode_str(&self, a: impl AsRef<str>) -> Result<Vec<u8>, DecodeError> {
        let a = a.as_ref().as_bytes();
        if let Some(pad) = self.pad {
            decode(self.dec, rem_pad(a, pad))
        } else {
            decode(self.dec, a)
        }
    }

    /// Pass string as bytes and decoder array to [`decode_uuid`](super::decode_uuid())
    #[cfg(feature = "uuid")]
    #[inline]
    pub fn decode_uuid_str(&self, a: impl AsRef<str>) -> Result<Uuid, DecodeError> {
        let a = a.as_ref().as_bytes();
        decode_uuid(self.dec, a)
    }

    /// Pass decoder array to [`decode_uuid`](super::decode_uuid())
    #[cfg(feature = "uuid")]
    #[inline]
    pub fn decode_uuid(&self, a: &[u8]) -> Result<Uuid, DecodeError> {
        decode_uuid(self.dec, a)
    }

    /// Pass encoder array to [`encode_uuid`](super::encode_uuid())
    #[cfg(feature = "uuid")]
    #[inline]
    pub fn encode_uuid(&self, n: Uuid) -> String {
        encode_uuid(self.enc, n)
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::INVALID_BYTE;

    use super::*;

    #[test]
    fn confirm_crockford_decode() {
        const INVALID_CHAR: char = ' ';
        let mut s = String::new();
        for i in 0..=255u8 {
            let c = i as char;
            #[rustfmt::skip]
            let c = match c {
                '0' | 'o' | 'O' => '0',
                '1' | 'i' | 'I' | 'l' | 'L' => '1',
                '2' ..= '9' => c,
                'u' | 'U' => INVALID_CHAR,
                'A' ..= 'Z' => c,
                'a' ..= 'z' => c.to_ascii_uppercase(),
                _ => INVALID_CHAR,
            };
            s.push(c);
        }

        let dec_chars = DEC_CROCKFORD
            .map(|b| {
                if b == INVALID_BYTE {
                    INVALID_CHAR
                } else {
                    ENC_CROCKFORD[b as usize] as char
                }
            })
            .iter()
            .collect::<String>();

        assert_eq!(dec_chars, s);
    }
}
