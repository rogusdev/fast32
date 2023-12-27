use core::ptr::write;

#[cfg(feature = "uuid")]
use uuid::Uuid;

use crate::DecodeError;

use super::decode_bytes::{capacity_decode, decode, decode_into};
use super::decode_u128::decode_u128;
use super::decode_u64::decode_u64;
use super::encode_bytes::{capacity_encode, encode, encode_into};
use super::encode_u128::{capacity_encode_u128, encode_u128, encode_u128_into};
use super::encode_u64::{capacity_encode_u64, encode_u64, encode_u64_into};

#[cfg(feature = "uuid")]
use super::uuid::{decode_uuid, encode_uuid, encode_uuid_into};

/// Creates an appropriate base32 alphabet (nopad, padded, simple, or with from:to mapping)
///
/// Note that the second identifier is to make the decoder a constant static variable, for const fn usage.
///
/// See examples in [tests/alphabet.rs](https://github.com/rogusdev/fast32/blob/main/tests/alphabets.rs)
#[macro_export]
macro_rules! make_base32_alpha {
    ( $n:ident, $dec:ident, $enc:expr ) => {
        pub const $dec: [u8; 256] = $crate::decoder_map_simple($enc);
        pub const $n: $crate::base32::Alphabet32Nopad =
            $crate::base32::Alphabet32Nopad::new($enc, &$dec);
    };
    ( $n:ident, $dec:ident, $enc:expr, $pad:literal ) => {
        pub const $dec: [u8; 256] = $crate::decoder_map_simple($enc);
        pub const $n: $crate::base32::Alphabet32Padded =
            $crate::base32::Alphabet32Padded::new($enc, &$dec, $pad);
    };
    ( $n:ident, $dec:ident, $enc:expr, $fr:literal, $to:literal ) => {
        pub const $dec: [u8; 256] = $crate::decoder_map($enc, $fr, $to);
        pub const $n: $crate::base32::Alphabet32Nopad =
            $crate::base32::Alphabet32Nopad::new($enc, &$dec);
    };
    ( $n:ident, $dec:ident, $enc:expr, $fr:literal, $to:literal, $pad:literal ) => {
        pub const $dec: [u8; 256] = $crate::decoder_map($enc, $fr, $to);
        pub const $n: $crate::base32::Alphabet32Padded =
            $crate::base32::Alphabet32Padded::new($enc, &$dec, $pad);
    };
}

const ENC_RFC4648: &'static [u8; BITS] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
make_base32_alpha!(RFC4648, DEC_RFC4648, ENC_RFC4648, '=');
make_base32_alpha!(RFC4648_NOPAD, DEC_RFC4648_2, ENC_RFC4648);

const ENC_RFC4648_HEX: &'static [u8; BITS] = b"0123456789ABCDEFGHIJKLMNOPQRSTUV";
make_base32_alpha!(RFC4648_HEX, DEC_RFC4648_HEX, ENC_RFC4648_HEX, '=');
make_base32_alpha!(RFC4648_HEX_NOPAD, DEC_RFC4648_HEX_2, ENC_RFC4648_HEX);

// only a separate const to be used in the test below
const ENC_CROCKFORD: &'static [u8; BITS] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";
make_base32_alpha!(
    CROCKFORD,
    DEC_CROCKFORD,
    ENC_CROCKFORD,
    b"ILOabcdefghijklmnopqrstvwxyz",
    b"110ABCDEFGH1JK1MN0PQRSTVWXYZ"
);

make_base32_alpha!(
    CROCKFORD_LOWER,
    DEC_CROCKFORD_LOWER,
    b"0123456789abcdefghjkmnpqrstvwxyz",
    b"iloABCDEFGHIJKLMNOPQRSTVWXYZ",
    b"110abcdefgh1jk1mn0pqrstvwxyz"
);

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

#[inline]
const fn pad_len(len: usize) -> usize {
    // 0 len needs to go to 0 padding, not WIDTH_ENC
    (WIDTH_ENC - (len % WIDTH_ENC)) % WIDTH_ENC
}

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

/// Hold a specific padded base32 encoding and decoding map pair, with functions to do encoding + decoding
///
/// Padded alphabets cannot encode or decode integers (u64, u128, uuid)
pub struct Alphabet32Padded {
    enc: &'static [u8; BITS],
    dec: &'static [u8; 256],
    pad: char,
}

/// Hold a specific no padding base32 encoding and decoding map pair, with functions to do encoding + decoding
///
/// Non-padded alphabets can encode and decode integers (u64, u128, uuid)
pub struct Alphabet32Nopad {
    enc: &'static [u8; BITS],
    dec: &'static [u8; 256],
}

impl Alphabet32Padded {
    /// Instantiate with encoder and decoder maps, plus padding char
    pub const fn new(enc: &'static [u8; BITS], dec: &'static [u8; 256], pad: char) -> Self {
        Self { enc, dec, pad }
    }

    /// Capacity needed in dest `Vec<u8>` to encode this byte array -- with padding!
    #[inline]
    pub const fn capacity_encode(&self, a: &[u8]) -> usize {
        let len = capacity_encode(a);
        len + pad_len(len)
    }

    /// Capacity needed in dest `Vec<u8>` to decode this byte array -- with padding!
    pub fn capacity_decode(&self, a: &[u8]) -> usize {
        capacity_decode(rem_pad(a, self.pad))
    }

    /// Pass encoder array to [`encode`](super::encode()), and add padding as needed
    #[inline]
    pub fn encode(&self, a: &[u8]) -> String {
        let mut s = encode(self.enc, a);
        unsafe {
            let b = s.as_mut_vec();
            b.reserve(pad_len(b.len()));
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
    ///
    /// Also returns [`DecodeError`] if input is an invalid length for padding (i.e. not a multiple of 8)
    #[inline]
    pub fn decode(&self, a: &[u8]) -> Result<Vec<u8>, DecodeError> {
        if a.len() % WIDTH_ENC != 0 {
            Err(DecodeError::InvalidLength { length: a.len() })
        } else {
            decode(self.dec, rem_pad(a, self.pad))
        }
    }

    /// Pass decoder array to [`decode_into`](super::decode_into()), and remove padding as needed
    ///
    /// Also returns [`DecodeError`] if input is an invalid length for padding (i.e. not a multiple of 8)
    #[inline]
    pub fn decode_into(&self, a: &[u8], b: &mut Vec<u8>) -> Result<(), DecodeError> {
        if a.len() % WIDTH_ENC != 0 {
            Err(DecodeError::InvalidLength { length: a.len() })
        } else {
            decode_into(self.dec, rem_pad(a, self.pad), b)
        }
    }

    /// Pass string as bytes and decoder array to [`decode`](super::decode()), and remove padding as needed
    ///
    /// Also returns [`DecodeError`] if input is an invalid length for padding (i.e. not a multiple of 8)
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

impl Alphabet32Nopad {
    /// Instantiate with encoder and decoder maps (no padding)
    pub const fn new(enc: &'static [u8; BITS], dec: &'static [u8; 256]) -> Self {
        Self { enc, dec }
    }

    /// Pass input bytes to [`capacity_encode`](super::capacity_encode())
    #[inline]
    pub const fn capacity_encode(&self, a: &[u8]) -> usize {
        capacity_encode(a)
    }

    /// Pass input u64 to [`capacity_encode_u64`](super::capacity_encode_u64())
    #[inline]
    pub const fn capacity_encode_u64(&self, n: u64) -> usize {
        capacity_encode_u64(n)
    }

    /// Pass input u128 to [`capacity_encode_u128`](super::capacity_encode_u128())
    #[inline]
    pub const fn capacity_encode_u128(&self, n: u128) -> usize {
        capacity_encode_u128(n)
    }

    /// Pass input bytes to [`capacity_decode`](super::capacity_decode())
    #[inline]
    pub const fn capacity_decode(&self, a: &[u8]) -> usize {
        capacity_decode(a)
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
