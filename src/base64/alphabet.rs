use core::ptr::write;

#[cfg(feature = "uuid")]
use uuid::Uuid;

use crate::shared::{INVALID_BYTE, INVALID_CHAR};
use crate::DecodeError;

use super::decode_bytes::{decode_bytes, decode_bytes_into};
use super::decode_u128::decode_u128;
use super::decode_u64::decode_u64;
use super::encode_bytes::{encode_bytes, encode_bytes_into};
use super::encode_u128::{encode_u128, encode_u128_into};
use super::encode_u64::{encode_u64, encode_u64_into};

#[cfg(feature = "uuid")]
use super::uuid::{decode_uuid, decode_uuid_str, encode_uuid};


const ENC_RFC4648: &'static [u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const DEC_RFC4648: [u8; 256] = decoder_map_simple(ENC_RFC4648);
pub const RFC4648: Alphabet = Alphabet::new(ENC_RFC4648, &DEC_RFC4648, Some('='));
pub const RFC4648_NOPAD: Alphabet = Alphabet::new(ENC_RFC4648, &DEC_RFC4648, None);

const ENC_RFC4648_URL: &'static [u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
const DEC_RFC4648_URL: [u8; 256] = decoder_map_simple(ENC_RFC4648_URL);
pub const RFC4648_URL: Alphabet = Alphabet::new(ENC_RFC4648_URL, &DEC_RFC4648_URL, Some('='));
pub const RFC4648_URL_NOPAD: Alphabet = Alphabet::new(ENC_RFC4648_URL, &DEC_RFC4648_URL, None);


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

pub struct Alphabet {
    enc: &'static [u8; BITS],
    dec: &'static [u8; 256],
    pad: Option<char>,
}

unsafe fn add_pad(b: &mut Vec<u8>, pad: char) {
    let len = b.len();
    match len % WIDTH_ENC {
        2 => {
            assert!(b.capacity() >= len + 2, "Missing capacity for padding");
            let end = b.as_mut_ptr().add(len);

            write(end       , pad as u8);
            write(end.add(1), pad as u8);

            b.set_len(len + 2);
        }
        3 => {
            assert!(b.capacity() >= len + 1, "Missing capacity for padding");
            let end = b.as_mut_ptr().add(len);

            write(end       , pad as u8);

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

impl Alphabet {
    pub const fn new(enc: &'static [u8; BITS], dec: &'static [u8; 256], pad: Option<char>) -> Self {
        Self { enc, dec, pad }
    }

    #[inline]
    pub fn encode_u64(&self, n: u64) -> String {
        encode_u64(self.enc, n)
    }

    #[inline]
    pub fn encode_u128(&self, n: u128) -> String {
        encode_u128(self.enc, n)
    }

    #[inline]
    pub fn encode_u64_into(&self, n: u64, b: &mut Vec<u8>) {
        encode_u64_into(self.enc, n, b)
    }

    #[inline]
    pub fn encode_u128_into(&self, n: u128, b: &mut Vec<u8>) {
        encode_u128_into(self.enc, n, b)
    }

    #[inline]
    pub fn encode_bytes(&self, a: &[u8]) -> String {
        if let Some(pad) = self.pad {
            let mut s = encode_bytes(self.enc, a);
            unsafe {
                let b = s.as_mut_vec();
                // make space for max possible padding
                b.reserve(2);
                add_pad(b, pad);
            }
            s
        } else {
            encode_bytes(self.enc, a)
        }
    }

    #[inline]
    pub fn encode_bytes_into(&self, a: &[u8], b: &mut Vec<u8>) {
        if let Some(pad) = self.pad {
            encode_bytes_into(self.enc, a, b);
            unsafe { add_pad(b, pad) }
        } else {
            encode_bytes_into(self.enc, a, b)
        }
    }

    #[inline]
    pub fn encode_bytes_str(&self, a: impl AsRef<str>) -> String {
        let a = a.as_ref().as_bytes();
        if let Some(pad) = self.pad {
            let mut s = encode_bytes(self.enc, a);
            unsafe {
                let b = s.as_mut_vec();
                // make space for max possible padding
                b.reserve(2);
                add_pad(b, pad);
            }
            s
        } else {
            encode_bytes(self.enc, a)
        }
    }

    #[inline]
    pub fn decode_u64(&self, a: &[u8]) -> Result<u64, DecodeError> {
        decode_u64(self.dec, a)
    }

    #[inline]
    pub fn decode_u128(&self, a: &[u8]) -> Result<u128, DecodeError> {
        decode_u128(self.dec, a)
    }

    #[inline]
    pub fn decode_bytes(&self, a: &[u8]) -> Result<Vec<u8>, DecodeError> {
        if let Some(pad) = self.pad {
            decode_bytes(self.dec, rem_pad(a, pad))
        } else {
            decode_bytes(self.dec, a)
        }
    }

    #[inline]
    pub fn decode_bytes_into(&self, a: &[u8], b: &mut Vec<u8>) -> Result<(), DecodeError> {
        if let Some(pad) = self.pad {
            decode_bytes_into(self.dec, rem_pad(a, pad), b)
        } else {
            decode_bytes_into(self.dec, a, b)
        }
    }

    #[inline]
    pub fn decode_u64_str(&self, a: impl AsRef<str>) -> Result<u64, DecodeError> {
        let a = a.as_ref().as_bytes();
        decode_u64(self.dec, a)
    }

    #[inline]
    pub fn decode_u128_str(&self, a: impl AsRef<str>) -> Result<u128, DecodeError> {
        let a = a.as_ref().as_bytes();
        decode_u128(self.dec, a)
    }

    #[inline]
    pub fn decode_bytes_str(&self, a: impl AsRef<str>) -> Result<Vec<u8>, DecodeError> {
        let a = a.as_ref().as_bytes();
        if let Some(pad) = self.pad {
            decode_bytes(self.dec, rem_pad(a, pad))
        } else {
            decode_bytes(self.dec, a)
        }
    }

    #[cfg(feature = "uuid")]
    #[inline]
    pub fn decode_uuid_str(&self, a: impl AsRef<str>) -> Result<Uuid, DecodeError> {
        decode_uuid_str(self.dec, a)
    }

    #[cfg(feature = "uuid")]
    #[inline]
    pub fn decode_uuid(&self, a: &[u8]) -> Result<Uuid, DecodeError> {
        decode_uuid(self.dec, a)
    }

    #[cfg(feature = "uuid")]
    #[inline]
    pub fn encode_uuid(&self, n: Uuid) -> String {
        encode_uuid(self.enc, n)
    }
}

pub const fn decoder_map_simple(enc: &[u8; BITS]) -> [u8; 256] {
    let mut dec = [INVALID_BYTE; 256];
    let mut i = 0;
    while i < BITS {
        dec[enc[i] as usize] = i as u8;
        i += 1;
    }
    dec
}

const fn decoder_char_from_enc(enc: &[u8; BITS], map: &[u8; 94], i: usize) -> u8 {
    let c = map[i];
    if c == INVALID_CHAR as u8 {
        return INVALID_BYTE;
    }

    let mut j = 0;
    while j < BITS {
        if enc[j] == c {
            return j as u8;
        }
        j += 1;
    }

    panic!("Decoder has char not present in encoder!")
}

pub const fn decoder_map(enc: &[u8; BITS], map: &[u8; 94]) -> [u8; 256] {
    let mut dec = [INVALID_BYTE; 256];
    let mut i = 33;
    while i < 127 {
        dec[i] = decoder_char_from_enc(enc, &map, i - 33);
        i += 1;
    }
    dec
}
