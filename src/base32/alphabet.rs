#[cfg(feature = "uuid")]
use uuid::Uuid;

use crate::shared::{INVALID_BYTE, INVALID_CHAR};
use crate::DecodeError;

use super::decode_bytes::{decode_bytes, decode_bytes_str};
use super::decode_u128::{decode_u128, decode_u128_str};
use super::decode_u64::{decode_u64, decode_u64_str};
use super::encode_bytes::{encode_bytes, encode_bytes_str};
use super::encode_u128::{encode_u128, encode_u128_into};
use super::encode_u64::{encode_u64, encode_u64_into};

#[cfg(feature = "uuid")]
use super::uuid::{decode_uuid, decode_uuid_str, encode_uuid};


const ENC_CROCKFORD: &'static [u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";
const DEC_CROCKFORD: [u8; 256] = decoder_map(ENC_CROCKFORD,
    b"               0123456789       ABCDEFGH1JK1MN0PQRST VWXYZ      ABCDEFGH1JK1MN0PQRST VWXYZ    "
);
pub const CROCKFORD: Alphabet = Alphabet::new(ENC_CROCKFORD, &DEC_CROCKFORD, None);

const ENC_RFC4648: &'static [u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
const DEC_RFC4648: [u8; 256] = decoder_map_simple(ENC_RFC4648);
pub const RFC4648: Alphabet = Alphabet::new(ENC_RFC4648, &DEC_RFC4648, Some('='));
pub const RFC4648_NOPAD: Alphabet = Alphabet::new(ENC_RFC4648, &DEC_RFC4648, None);

const ENC_RFC4648_HEX: &'static [u8; 32] = b"0123456789ABCDEFGHIJKLMNOPQRSTUV";
const DEC_RFC4648_HEX: [u8; 256] = decoder_map_simple(ENC_RFC4648_HEX);
pub const RFC4648_HEX: Alphabet = Alphabet::new(ENC_RFC4648_HEX, &DEC_RFC4648_HEX, Some('='));
pub const RFC4648_HEX_NOPAD: Alphabet = Alphabet::new(ENC_RFC4648_HEX, &DEC_RFC4648_HEX, None);


pub const BITS: usize = 32;

pub const WIDTH_IN: usize = 5;
pub const WIDTH_OUT: usize = 8;

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

pub struct Alphabet {
    enc: &'static [u8; BITS],
    dec: &'static [u8; 256],
    pad: Option<char>,
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
            let s = encode_bytes(self.enc, a);
            match s.len() % 8 {
                2 => format!("{s}{pad}{pad}{pad}{pad}{pad}{pad}"),
                4 => format!("{s}{pad}{pad}{pad}{pad}"),
                5 => format!("{s}{pad}{pad}{pad}"),
                7 => format!("{s}{pad}"),
                _ => s,
            }
        } else {
            encode_bytes(self.enc, a)
        }
    }

    #[inline]
    pub fn encode_bytes_str(&self, a: impl AsRef<str>) -> String {
        if let Some(pad) = self.pad {
            let s = encode_bytes_str(self.enc, a);
            match s.len() % 8 {
                2 => format!("{s}{pad}{pad}{pad}{pad}{pad}{pad}"),
                4 => format!("{s}{pad}{pad}{pad}{pad}"),
                5 => format!("{s}{pad}{pad}{pad}"),
                7 => format!("{s}{pad}"),
                _ => s,
            }
        } else {
            encode_bytes_str(self.enc, a)
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
            let len = a.len();
            let pad = pad as u8;
            if len == 0 {
                decode_bytes(self.dec, a)
            } else if a[len - 6] == pad {
                decode_bytes(self.dec, &a[..len - 6])
            } else if a[len - 4] == pad {
                decode_bytes(self.dec, &a[..len - 4])
            } else if a[len - 3] == pad {
                decode_bytes(self.dec, &a[..len - 3])
            } else if a[len - 1] == pad {
                decode_bytes(self.dec, &a[..len - 1])
            } else {
                decode_bytes(self.dec, a)
            }
        } else {
            decode_bytes(self.dec, a)
        }
    }

    #[inline]
    pub fn decode_u64_str(&self, a: impl AsRef<str>) -> Result<u64, DecodeError> {
        decode_u64_str(self.dec, a)
    }

    #[inline]
    pub fn decode_u128_str(&self, a: impl AsRef<str>) -> Result<u128, DecodeError> {
        decode_u128_str(self.dec, a)
    }

    #[inline]
    pub fn decode_bytes_str(&self, a: impl AsRef<str>) -> Result<Vec<u8>, DecodeError> {
        if let Some(pad) = self.pad {
            // note that with pad this skips _str version and goes direct
            let a = a.as_ref().as_bytes();
            let len = a.len();
            let pad = pad as u8;
            if len == 0 {
                decode_bytes(self.dec, a)
            } else if a[len - 6] == pad {
                decode_bytes(self.dec, &a[..len - 6])
            } else if a[len - 4] == pad {
                decode_bytes(self.dec, &a[..len - 4])
            } else if a[len - 3] == pad {
                decode_bytes(self.dec, &a[..len - 3])
            } else if a[len - 1] == pad {
                decode_bytes(self.dec, &a[..len - 1])
            } else {
                decode_bytes(self.dec, a)
            }
        } else {
            decode_bytes_str(self.dec, a)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn confirm_crockford_decode() {
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
