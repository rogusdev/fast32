use crate::INVALID_BYTE;

#[inline]
pub fn bits_or_err(dec: &[u8; 256], a: &[u8], i: usize) -> Result<u64, DecodeError> {
    let c = a[i];
    let o = dec[c as usize];
    if o == INVALID_BYTE {
        Err(DecodeError::InvalidChar { char: c as char, index: i })
    } else {
        Ok(o as u64)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DecodeError {
    InvalidChar { char: char, index: usize },
    InvalidLength { length: usize },
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
        }
    }
}
