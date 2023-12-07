pub const INVALID_CHAR: char = '.';
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
            DecodeError::InvalidChar { char, index } => {
                write!(f, "Invalid char of '{char}' at position {index}")
            }
            DecodeError::InvalidLength { length } => {
                write!(f, "Invalid length of {length}")
            } // DecodeError::InvalidBits { byte, index } => {
              //     write!(f, "Invalid bits in {byte} at position {index}")
              // }
        }
    }
}
