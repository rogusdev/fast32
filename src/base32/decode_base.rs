use super::alphabet::INVALID_BYTE;

pub fn bits_or_err_u8(dec: &[u8; 256], a: &[u8], i: usize) -> Result<u8, DecodeError> {
    let c = a[i];
    let o = dec[c as usize];
    if o == INVALID_BYTE {
        Err(DecodeError::InvalidChar { char: c as char, index: i })
    } else {
        Ok(o)
    }
}

pub fn bits_or_err_u64(dec: &[u8; 256], a: &[u8], i: usize) -> Result<u64, DecodeError> {
    let c = a[i];
    let o = dec[c as usize];
    if o == INVALID_BYTE {
        Err(DecodeError::InvalidChar { char: c as char, index: i })
    } else {
        Ok(o as u64)
    }
}

pub fn bits_or_err_u128(dec: &[u8; 256], a: &[u8], i: usize) -> Result<u128, DecodeError> {
    let c = a[i];
    let o = dec[c as usize];
    if o == INVALID_BYTE {
        Err(DecodeError::InvalidChar { char: c as char, index: i })
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
            }
            // DecodeError::InvalidBits { byte, index } => {
            //     write!(f, "Invalid bits in {byte} at position {index}")
            // }
        }
    }
}
