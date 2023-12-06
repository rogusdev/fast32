use paste::paste;

use crate::DecodeError;
use crate::shared::{INVALID_BYTE, INVALID_CHAR};

use super::decode_u64::decode_u64;
use super::decode_u128::decode_u128;
use super::decode_u64::decode_u64_str;
use super::decode_u128::decode_u128_str;
use super::encode_u64::encode_u64;
use super::encode_u128::encode_u128;
use super::decode_bytes::decode_bytes;
use super::decode_bytes::decode_bytes_str;
use super::encode_bytes::encode_bytes;
use super::encode_bytes::encode_bytes_str;

pub const BITS: usize = 64;

#[macro_export]
macro_rules! make_base64_alpha_simple {
    ( $n:ident, $e:literal ) => {
        paste! {
const [<ENC_ $n>]: &'static [u8; BITS] = $e;
const [<DEC_ $n>]: [u8; 256] = decoder_map_simple([<ENC_ $n>]);
pub const $n: Alphabet = Alphabet::new([<ENC_ $n>], &[<DEC_ $n>], None);
        }
    };
    ( $n:ident, $e:literal, $p:literal ) => {
        paste! {
const [<ENC_ $n>]: &'static [u8; BITS] = $e;
const [<DEC_ $n>]: [u8; 256] = decoder_map_simple([<ENC_ $n>]);
pub const $n: Alphabet = Alphabet::new([<ENC_ $n>], &[<DEC_ $n>], Some($p));
        }
    };
}

#[macro_export]
macro_rules! make_base64_alpha_mapped {
    ( $n:ident, $e:literal, $d:literal ) => {
        paste! {
const [<ENC_ $n>]: &'static [u8; BITS] = $e;
const [<DEC_ $n>]: [u8; 256] = decoder_map([<ENC_ $n>], $d);
pub const $n: Alphabet = Alphabet::new([<ENC_ $n>], &[<DEC_ $n>], None);
        }
    };
    ( $n:ident, $e:literal, $d:literal, $p:literal ) => {
        paste! {
const [<ENC_ $n>]: &'static [u8; BITS] = $e;
const [<DEC_ $n>]: [u8; 256] = decoder_map([<ENC_ $n>], $d);
pub const $n: Alphabet = Alphabet::new([<ENC_ $n>], &[<DEC_ $n>], Some($p));
        }
    };
}

make_base64_alpha_simple!(RFC4648, b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/", '=');
make_base64_alpha_simple!(RFC4648_URL, b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_", '=');
make_base64_alpha_simple!(RFC4648_NOPAD, b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
make_base64_alpha_simple!(RFC4648_URL_NOPAD, b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_");

pub struct Alphabet {
    enc: &'static [u8; BITS],
    dec: &'static [u8; 256],
    pad: Option<char>,
}

impl Alphabet {
    pub const fn new(
        enc: &'static [u8; BITS],
        dec: &'static [u8; 256],
        pad: Option<char>,
    ) -> Self {
        Self {
            enc,
            dec,
            pad,
        }
    }

    pub fn encode_u64(&self, n: u64) -> String {
        encode_u64(self.enc, n)
    }

    pub fn encode_u128(&self, n: u128) -> String {
        encode_u128(self.enc, n)
    }

    pub fn encode_bytes(&self, a: &[u8]) -> String {
        if let Some(pad) = self.pad {
            let s = encode_bytes(self.enc, a);
            match s.len() % 4 {
                2 => format!("{s}{pad}{pad}"),
                3 => format!("{s}{pad}"),
                _ => s
            }
        } else {
            encode_bytes(self.enc, a)
        }
    }

    pub fn encode_bytes_str(&self, a: impl AsRef<str>) -> String {
        if let Some(pad) = self.pad {
            let s = encode_bytes_str(self.enc, a);
            match s.len() % 4 {
                2 => format!("{s}{pad}{pad}"),
                3 => format!("{s}{pad}"),
                _ => s
            }
        } else {
            encode_bytes_str(self.enc, a)
        }
    }

    pub fn decode_u64(&self, a: &[u8]) -> Result<u64, DecodeError> {
        decode_u64(self.dec, a)
    }

    pub fn decode_u128(&self, a: &[u8]) -> Result<u128, DecodeError> {
        decode_u128(self.dec, a)
    }

    pub fn decode_bytes(&self, a: &[u8]) -> Result<Vec<u8>, DecodeError> {
        if let Some(pad) = self.pad {
            let len = a.len();
            let pad = pad as u8;
            if len < 4 {
                decode_bytes(self.dec, a)
            } else if a[len-3] == pad {
                decode_bytes(self.dec, &a[..len-3])
            } else if a[len-2] == pad {
                decode_bytes(self.dec, &a[..len-2])
            } else {
                decode_bytes(self.dec, a)
            }
        } else {
            decode_bytes(self.dec, a)
        }
    }

    pub fn decode_u64_str(&self, a: impl AsRef<str>) -> Result<u64, DecodeError> {
        decode_u64_str(self.dec, a)
    }

    pub fn decode_u128_str(&self, a: impl AsRef<str>) -> Result<u128, DecodeError> {
        decode_u128_str(self.dec, a)
    }

    pub fn decode_bytes_str(&self, a: impl AsRef<str>) -> Result<Vec<u8>, DecodeError> {
        if let Some(pad) = self.pad {
            // note that with pad this skips _str version and goes direct
            let a = a.as_ref().as_bytes();
            let len = a.len();
            let pad = pad as u8;
            if len < 4 {
                decode_bytes(self.dec, a)
            } else if a[len-3] == pad {
                decode_bytes(self.dec, &a[..len-3])
            } else if a[len-2] == pad {
                decode_bytes(self.dec, &a[..len-2])
            } else {
                decode_bytes(self.dec, a)
            }
        } else {
            decode_bytes_str(self.dec, a)
        }
    }
}

const fn decoder_map_simple(enc: &[u8; BITS]) -> [u8; 256] {
    let mut dec = [INVALID_BYTE; 256];
    let mut i = 0;
    while i < BITS {
        dec[enc[i] as usize] = i as u8;
        i += 1;
    }
    dec
}

const fn decoder_char_from_enc(enc: &[u8; BITS], dec: &[u8; 128], i: usize) -> u8 {
    let c = dec[i];
    if c == INVALID_CHAR as u8 {
        return INVALID_BYTE
    }

    let mut j = 0;
    while j < BITS {
        if enc[j] == c {
            return j as u8
        }
        j += 1;
    }

    panic!("Decoder has char not present in encoder!")
}

// TODO: confirm that the macro above will actually work from a separate crate...
#[allow(dead_code)]
const fn decoder_map(enc: &[u8; BITS], map: &[u8; 128]) -> [u8; 256] {
    let mut dec = [INVALID_BYTE; 256];
    let mut i = 0;
    while i < 128 {
        dec[i] = decoder_char_from_enc(enc, &map, i);
        i += 1;
    }
    dec
}
