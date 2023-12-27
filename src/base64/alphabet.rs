use core::ptr::write;

#[cfg(feature = "uuid")]
use uuid::Uuid;

use crate::DecodeError;

use super::decode_bytes::{decode, decode_into};
use super::decode_u128::decode_u128;
use super::decode_u64::decode_u64;
use super::encode_bytes::{encode, encode_into};
use super::encode_u128::{encode_u128, encode_u128_into};
use super::encode_u64::{encode_u64, encode_u64_into};

#[cfg(feature = "uuid")]
use super::uuid::{decode_uuid, encode_uuid, encode_uuid_into};

/// Creates an appropriate base64 alphabet (nopad, padded, simple, or with from:to mapping)
///
/// Note that the second identifier is to make the decoder a constant static variable, for const fn usage.
///
/// See examples in [tests/alphabet.rs](https://github.com/rogusdev/fast32/blob/main/tests/alphabets.rs)
#[macro_export]
macro_rules! make_base64_alpha {
    ( $n:ident, $dec:ident, $enc:expr ) => {
        pub const $dec: [u8; 256] = $crate::decoder_map_simple($enc);
        pub const $n: $crate::base64::Alphabet64Nopad =
            $crate::base64::Alphabet64Nopad::new($enc, &$dec);
    };
    ( $n:ident, $dec:ident, $enc:expr, $pad:literal ) => {
        pub const $dec: [u8; 256] = $crate::decoder_map_simple($enc);
        pub const $n: $crate::base64::Alphabet64Padded =
            $crate::base64::Alphabet64Padded::new($enc, &$dec, $pad);
    };
    ( $n:ident, $dec:ident, $enc:expr, $fr:literal, $to:literal ) => {
        pub const $dec: [u8; 256] = $crate::decoder_map($enc, $fr, $to);
        pub const $n: $crate::base64::Alphabet64Nopad =
            $crate::base64::Alphabet64Nopad::new($enc, &$dec);
    };
    ( $n:ident, $dec:ident, $enc:expr, $fr:literal, $to:literal, $pad:literal ) => {
        pub const $dec: [u8; 256] = $crate::decoder_map($enc, $fr, $to);
        pub const $n: $crate::base64::Alphabet64Padded =
            $crate::base64::Alphabet64Padded::new($enc, &$dec, $pad);
    };
}

const ENC_RFC4648: &'static [u8; BITS] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
make_base64_alpha!(RFC4648, DEC_RFC4648, ENC_RFC4648, '=');
make_base64_alpha!(RFC4648_NOPAD, DEC_RFC4648_2, ENC_RFC4648);

const ENC_RFC4648_URL: &'static [u8; BITS] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
make_base64_alpha!(RFC4648_URL, DEC_RFC4648_URL, ENC_RFC4648_URL, '=');
make_base64_alpha!(RFC4648_URL_NOPAD, DEC_RFC4648_URL_2, ENC_RFC4648_URL);

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

/// Hold a specific padded base64 encoding and decoding map pair, with functions to do encoding + decoding
///
/// Padded alphabets cannot encode or decode integers (u64, u128, uuid)
pub struct Alphabet64Padded {
    enc: &'static [u8; BITS],
    dec: &'static [u8; 256],
    pad: char,
}

/// Hold a specific no padding base64 encoding and decoding map pair, with functions to do encoding + decoding
///
/// Non-padded alphabets can encode and decode integers (u64, u128, uuid)
pub struct Alphabet64Nopad {
    enc: &'static [u8; BITS],
    dec: &'static [u8; 256],
}

impl Alphabet64Padded {
    /// Instantiate with encoder and decoder maps, plus padding char
    pub const fn new(enc: &'static [u8; BITS], dec: &'static [u8; 256], pad: char) -> Self {
        Self { enc, dec, pad }
    }

    /// Pass encoder array to [`encode`](super::encode()), and add padding as needed
    #[inline]
    pub fn encode(&self, a: &[u8]) -> String {
        let mut s = encode(self.enc, a);
        unsafe {
            let b = s.as_mut_vec();
            // make space for max possible padding
            b.reserve(2);
            add_pad(b, self.pad);
        }
        s
    }

    /// Pass encoder array to [`encode_into`](super::encode_into()), and add padding as needed
    #[inline]
    pub fn encode_into(&self, a: &[u8], b: &mut Vec<u8>) {
        encode_into(self.enc, a, b);
        unsafe { add_pad(b, self.pad) }
    }

    /// Pass decoder array to [`decode`](super::decode()), and remove padding as needed
    #[inline]
    pub fn decode(&self, a: &[u8]) -> Result<Vec<u8>, DecodeError> {
        if a.len() % WIDTH_ENC != 0 {
            Err(DecodeError::InvalidLength { length: a.len() })
        } else {
            decode(self.dec, rem_pad(a, self.pad))
        }
    }

    /// Pass decoder array to [`decode_into`](super::decode_into()), and remove padding as needed
    #[inline]
    pub fn decode_into(&self, a: &[u8], b: &mut Vec<u8>) -> Result<(), DecodeError> {
        if a.len() % WIDTH_ENC != 0 {
            Err(DecodeError::InvalidLength { length: a.len() })
        } else {
            decode_into(self.dec, rem_pad(a, self.pad), b)
        }
    }

    /// Pass string as bytes and decoder array to [`decode`](super::decode()), and remove padding as needed
    #[inline]
    pub fn decode_str(&self, a: impl AsRef<str>) -> Result<Vec<u8>, DecodeError> {
        let a = a.as_ref().as_bytes();
        if a.len() % WIDTH_ENC != 0 {
            Err(DecodeError::InvalidLength { length: a.len() })
        } else {
            decode(self.dec, rem_pad(a, self.pad))
        }
    }
}

impl Alphabet64Nopad {
    /// Instantiate with encoder and decoder maps (no padding)
    pub const fn new(enc: &'static [u8; BITS], dec: &'static [u8; 256]) -> Self {
        Self { enc, dec }
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

    /// Pass encoder array to [`encode`](super::encode())
    #[inline]
    pub fn encode(&self, a: &[u8]) -> String {
        encode(self.enc, a)
    }

    /// Pass encoder array to [`encode_into`](super::encode_into())
    #[inline]
    pub fn encode_into(&self, a: &[u8], b: &mut Vec<u8>) {
        encode_into(self.enc, a, b)
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

    /// Pass decoder array to [`decode`](super::decode())
    #[inline]
    pub fn decode(&self, a: &[u8]) -> Result<Vec<u8>, DecodeError> {
        decode(self.dec, a)
    }

    /// Pass decoder array to [`decode_into`](super::decode_into())
    #[inline]
    pub fn decode_into(&self, a: &[u8], b: &mut Vec<u8>) -> Result<(), DecodeError> {
        decode_into(self.dec, a, b)
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

    /// Pass string as bytes and decoder array to [`decode`](super::decode())
    #[inline]
    pub fn decode_str(&self, a: impl AsRef<str>) -> Result<Vec<u8>, DecodeError> {
        let a = a.as_ref().as_bytes();
        decode(self.dec, a)
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

    /// Pass encoder array to [`encode_uuid_into`](super::encode_uuid_into())
    #[cfg(feature = "uuid")]
    #[inline]
    pub fn encode_uuid_into(&self, n: Uuid, b: &mut Vec<u8>) {
        encode_uuid_into(self.enc, n, b)
    }
}
