/// Used for marking indices in decoder arrays that are invalid characters
///
/// Only useful externally when testing alphabets like in
/// [tests/alphabet.rs](https://github.com/rogusdev/fast32/blob/main/tests/alphabets.rs)
pub const INVALID_BYTE: u8 = u8::MAX;

pub const U8_MASK_BOT_5: u8 = 0b00011111;
pub const U8_MASK_BOT_4: u8 = 0b00001111;
pub const U8_MASK_TOP_4: u8 = 0b11110000;
pub const U8_MASK_TOP_3: u8 = 0b11100000;
pub const U8_MASK_BOT_2: u8 = 0b00000011;
pub const U8_MASK_TOP_1: u8 = 0b10000000;
pub const U8_MASK_BOT_1: u8 = 0b00000001;
pub const U8_MASK_TOP_2: u8 = 0b11000000;
pub const U8_MASK_BOT_3: u8 = 0b00000111;
pub const U8_MASK_TOP_5: u8 = 0b11111000;
pub const U8_MASK_BOT_6: u8 = 0b00111111;
pub const U8_MASK_TOP_6: u8 = 0b11111100;

pub fn bits_or_err_u8(dec: &[u8; 256], a: &[u8], i: usize) -> Result<u8, DecodeError> {
    let c = a[i];
    let o = dec[c as usize];
    if o == INVALID_BYTE {
        Err(DecodeError::InvalidChar {
            char: c as char,
            index: i,
        })
    } else {
        Ok(o)
    }
}

pub fn bits_or_err_u64(dec: &[u8; 256], a: &[u8], i: usize) -> Result<u64, DecodeError> {
    let c = a[i];
    let o = dec[c as usize];
    if o == INVALID_BYTE {
        Err(DecodeError::InvalidChar {
            char: c as char,
            index: i,
        })
    } else {
        Ok(o as u64)
    }
}

pub fn bits_or_err_u128(dec: &[u8; 256], a: &[u8], i: usize) -> Result<u128, DecodeError> {
    let c = a[i];
    let o = dec[c as usize];
    if o == INVALID_BYTE {
        Err(DecodeError::InvalidChar {
            char: c as char,
            index: i,
        })
    } else {
        Ok(o as u128)
    }
}

/// Error generated when decoding invalid data
///
/// `InvalidChar`: a character to be decoded is either:
///
/// 1. not in the decoding list of characters at all or
/// 1. positioned at the end of a sequence such that this char is impossible
/// (contains bits in positions, at the end, that make this not actually properly encoded data)
///
/// `InvalidLength`: The remainder of the length of the sequence to be decoded
/// is an impossible length for valid decodable sequences
/// (e.g. official base64 can *never* generate unpadded encoded sequences of strings
/// with a length remainder of 1, when divided by 4 -- only 2, 3, or 4)
///
/// Note that for decoding integers, invalid length is simply that
/// the length is too long for that size of integer.
#[derive(Debug, Clone, PartialEq)]
pub enum DecodeError {
    InvalidChar { char: char, index: usize },
    InvalidLength { length: usize },
    // FIXME: adding this decreases decode_u64 performance by >50%!
    // InvalidBits { byte: u8, index: usize },
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidChar { char, index } => {
                write!(f, "Invalid char of '{char}' at position {index}")
            }
            Self::InvalidLength { length } => {
                write!(f, "Invalid length of {length}")
            } // Self::InvalidBits { byte, index } => {
              //     write!(f, "Invalid bits in {byte} at position {index}")
              // }
        }
    }
}

impl std::error::Error for DecodeError {}

/// Build a new decoder array for the given bits (encoder array size), exactly of the encoder array
///
/// **You should use [`fast32::make_base32_alpha`](crate::make_base32_alpha)
/// or [`fast32::make_base64_alpha`](crate::make_base64_alpha) instead of this!**
/// (Those macros invoke this function as appropriate.)
///
/// Example:
/// ```
/// use fast32::decoder_map_simple;
/// use fast32::base32::Alphabet32Nopad;
///
/// const ENC_RFC4648_LOWER: &'static [u8; 32] = b"abcdefghijklmnopqrstuvwxyz234567";
/// const DEC_RFC4648_LOWER: [u8; 256] = decoder_map_simple(ENC_RFC4648_LOWER);
/// pub const RFC4648_LOWER_NOPAD: Alphabet32Nopad = Alphabet32Nopad::new(ENC_RFC4648_LOWER, &DEC_RFC4648_LOWER);
///
/// assert_eq!(RFC4648_LOWER_NOPAD.encode_u64(31), "7");
/// assert_eq!(RFC4648_LOWER_NOPAD.decode_u64_str("7").unwrap(), 31);
/// ```
///
/// See more in [tests/alphabet.rs](https://github.com/rogusdev/fast32/blob/main/tests/alphabets.rs)
pub const fn decoder_map_simple<const B: usize>(enc: &[u8; B]) -> [u8; 256] {
    let mut dec = [INVALID_BYTE; 256];
    let mut i = 0;
    while i < B {
        dec[enc[i] as usize] = i as u8;
        i += 1;
    }
    dec
}

const fn enc_index<const B: usize>(enc: &[u8; B], c: u8) -> u8 {
    let mut j = 0;
    while j < B {
        if enc[j] == c {
            return j as u8;
        }
        j += 1;
    }
    INVALID_BYTE
}

/// Build a new decoder array for the given bits (encoder array size), with additional character translations
///
/// **You should use [`fast32::make_base32_alpha`](crate::make_base32_alpha)
/// or [`fast32::make_base64_alpha`](crate::make_base64_alpha) instead of this!**
/// (Those macros invoke this function as appropriate.)
///
/// Example:
/// ```
/// use fast32::decoder_map;
/// use fast32::base32::Alphabet32Nopad;
///
/// const ENC_CROCKFORD_LOWER: &'static [u8; 32] = b"0123456789abcdefghjkmnpqrstvwxyz";
/// const DEC_CROCKFORD_LOWER: [u8; 256] = decoder_map(
///     ENC_CROCKFORD_LOWER,
///     b"iloABCDEFGHIJKLMNOPQRSTVWXYZ",
///     b"110abcdefgh1jk1mn0pqrstvwxyz",
/// );
/// pub const CROCKFORD_LOWER: Alphabet32Nopad = Alphabet32Nopad::new(ENC_CROCKFORD_LOWER, &DEC_CROCKFORD_LOWER);
///
/// assert_eq!(CROCKFORD_LOWER.encode_u64(31), "z");
/// assert_eq!(CROCKFORD_LOWER.decode_u64_str("z").unwrap(), 31);
/// ```
///
/// See more in [tests/alphabet.rs](https://github.com/rogusdev/fast32/blob/main/tests/alphabets.rs)
pub const fn decoder_map<const B: usize, const N: usize>(
    enc: &[u8; B],
    from: &[u8; N],
    to: &[u8; N],
) -> [u8; 256] {
    let mut dec = decoder_map_simple(enc);

    let mut i = 0;
    while i < N {
        if enc_index(enc, from[i]) != INVALID_BYTE {
            panic!("Re-mapping a char from encoder!")
        }
        let j = enc_index(enc, to[i]);
        if j == INVALID_BYTE {
            panic!("Missing mapped char in encoder!")
        }
        dec[from[i] as usize] = j;
        i += 1;
    }
    dec
}
