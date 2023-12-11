use core::ptr::write;

#[cfg(feature = "uuid")]
use uuid::Uuid;

use crate::shared::decoder_map_simple;
use crate::DecodeError;

use super::decode_bytes::{decode, decode_into};
use super::decode_u128::decode_u128;
use super::decode_u64::decode_u64;
use super::encode_bytes::{encode, encode_into};
use super::encode_u128::{encode_u128, encode_u128_into};
use super::encode_u64::{encode_u64, encode_u64_into};

#[cfg(feature = "uuid")]
use super::uuid::{decode_uuid, encode_uuid};

const ENC_RFC4648: &'static [u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const DEC_RFC4648: [u8; 256] = decoder_map_simple(ENC_RFC4648);
/// RFC 4648 Base64 normal, with padding
///
/// `"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"` and `'='`
///
/// [https://datatracker.ietf.org/doc/html/rfc4648#section-4](https://datatracker.ietf.org/doc/html/rfc4648#section-4)
pub const RFC4648: Alphabet64 = Alphabet64::new(ENC_RFC4648, &DEC_RFC4648, Some('='));
/// RFC 4648 Base64 normal, no padding
///
/// `"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"`
///
/// [https://datatracker.ietf.org/doc/html/rfc4648#section-4](https://datatracker.ietf.org/doc/html/rfc4648#section-4)
pub const RFC4648_NOPAD: Alphabet64 = Alphabet64::new(ENC_RFC4648, &DEC_RFC4648, None);

const ENC_RFC4648_URL: &'static [u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
const DEC_RFC4648_URL: [u8; 256] = decoder_map_simple(ENC_RFC4648_URL);
/// RFC 4648 Base64 "url safe" form, with padding
///
/// `"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_"` and `'='`
///
/// [https://datatracker.ietf.org/doc/html/rfc4648#section-5](https://datatracker.ietf.org/doc/html/rfc4648#section-5)
pub const RFC4648_URL: Alphabet64 = Alphabet64::new(ENC_RFC4648_URL, &DEC_RFC4648_URL, Some('='));
/// RFC 4648 Base64 "url safe" form, no padding
///
/// `"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_"`
///
/// [https://datatracker.ietf.org/doc/html/rfc4648#section-5](https://datatracker.ietf.org/doc/html/rfc4648#section-5)
pub const RFC4648_URL_NOPAD: Alphabet64 = Alphabet64::new(ENC_RFC4648_URL, &DEC_RFC4648_URL, None);

pub const BITS: usize = 64;

pub const WIDTH_DEC: usize = 3;
pub const WIDTH_ENC: usize = 4;

pub const WIDTH_1: usize = 6;
pub const WIDTH_2: usize = 12;
pub const WIDTH_3: usize = 18;
pub const WIDTH_4: usize = 24;
pub const WIDTH_5: usize = 30;
pub const WIDTH_6: usize = 36;
pub const WIDTH_7: usize = 42;
pub const WIDTH_8: usize = 48;
pub const WIDTH_9: usize = 54;
pub const WIDTH_10: usize = 60;
pub const WIDTH_11: usize = 66;
pub const WIDTH_12: usize = 72;
pub const WIDTH_13: usize = 78;
pub const WIDTH_14: usize = 84;
pub const WIDTH_15: usize = 90;
pub const WIDTH_16: usize = 96;
pub const WIDTH_17: usize = 102;
pub const WIDTH_18: usize = 108;
pub const WIDTH_19: usize = 114;
pub const WIDTH_20: usize = 120;
pub const WIDTH_21: usize = 126;

unsafe fn add_pad(b: &mut Vec<u8>, pad: char) {
    let len = b.len();
    match len % WIDTH_ENC {
        2 => {
            assert!(b.capacity() >= len + 2, "Missing capacity for padding");
            let end = b.as_mut_ptr().add(len);

            write(end, pad as u8);
            write(end.add(1), pad as u8);

            b.set_len(len + 2);
        }
        3 => {
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
    } else if a[len - 2] == pad {
        &a[..len - 2]
    } else if a[len - 1] == pad {
        &a[..len - 1]
    } else {
        a
    }
}

/// Hold a specific base64 encoding and decoding map pair, with functions to do encoding + decoding
pub struct Alphabet64 {
    enc: &'static [u8; BITS],
    dec: &'static [u8; 256],
    pad: Option<char>,
}

impl Alphabet64 {
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
                b.reserve(2);
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
