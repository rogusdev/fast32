use std::fmt;

use crate::ENC_CROCKFORD_UPPER;
use crate::{INVALID_BYTE, INVALID_CHAR};

pub const DEC_CROCKFORD_UPPER: [u8; 256] = decoder_map(ENC_CROCKFORD_UPPER, b"................................................0123456789.......ABCDEFGH1JK1MN0PQRST.VWXYZ......ABCDEFGH1JK1MN0PQRST.VWXYZ.....");

const fn decoder_char_from_enc(enc: &[u8; 32], c: u8) -> u8 {
    if c == INVALID_CHAR as u8 {
        INVALID_BYTE
    } else if enc[0] == c {
        0
    } else if enc[1] == c {
        1
    } else if enc[2] == c {
        2
    } else if enc[3] == c {
        3
    } else if enc[4] == c {
        4
    } else if enc[5] == c {
        5
    } else if enc[6] == c {
        6
    } else if enc[7] == c {
        7
    } else if enc[8] == c {
        8
    } else if enc[9] == c {
        9
    } else if enc[10] == c {
        10
    } else if enc[11] == c {
        11
    } else if enc[12] == c {
        12
    } else if enc[13] == c {
        13
    } else if enc[14] == c {
        14
    } else if enc[15] == c {
        15
    } else if enc[16] == c {
        16
    } else if enc[17] == c {
        17
    } else if enc[18] == c {
        18
    } else if enc[19] == c {
        19
    } else if enc[20] == c {
        20
    } else if enc[21] == c {
        21
    } else if enc[22] == c {
        22
    } else if enc[23] == c {
        23
    } else if enc[24] == c {
        24
    } else if enc[25] == c {
        25
    } else if enc[26] == c {
        26
    } else if enc[27] == c {
        27
    } else if enc[28] == c {
        28
    } else if enc[29] == c {
        29
    } else if enc[30] == c {
        30
    } else if enc[31] == c {
        31
    } else {
        panic!("Decoder has chars not present in encoder!")
    }
}

const fn decoder_map(enc: &[u8; 32], dec: &[u8; 128]) -> [u8; 256] {
    [
        decoder_char_from_enc(enc, dec[0]),
        decoder_char_from_enc(enc, dec[1]),
        decoder_char_from_enc(enc, dec[2]),
        decoder_char_from_enc(enc, dec[3]),
        decoder_char_from_enc(enc, dec[4]),
        decoder_char_from_enc(enc, dec[5]),
        decoder_char_from_enc(enc, dec[6]),
        decoder_char_from_enc(enc, dec[7]),
        decoder_char_from_enc(enc, dec[8]),
        decoder_char_from_enc(enc, dec[9]),
        decoder_char_from_enc(enc, dec[10]),
        decoder_char_from_enc(enc, dec[11]),
        decoder_char_from_enc(enc, dec[12]),
        decoder_char_from_enc(enc, dec[13]),
        decoder_char_from_enc(enc, dec[14]),
        decoder_char_from_enc(enc, dec[15]),
        decoder_char_from_enc(enc, dec[16]),
        decoder_char_from_enc(enc, dec[17]),
        decoder_char_from_enc(enc, dec[18]),
        decoder_char_from_enc(enc, dec[19]),
        decoder_char_from_enc(enc, dec[20]),
        decoder_char_from_enc(enc, dec[21]),
        decoder_char_from_enc(enc, dec[22]),
        decoder_char_from_enc(enc, dec[23]),
        decoder_char_from_enc(enc, dec[24]),
        decoder_char_from_enc(enc, dec[25]),
        decoder_char_from_enc(enc, dec[26]),
        decoder_char_from_enc(enc, dec[27]),
        decoder_char_from_enc(enc, dec[28]),
        decoder_char_from_enc(enc, dec[29]),
        decoder_char_from_enc(enc, dec[30]),
        decoder_char_from_enc(enc, dec[31]),
        decoder_char_from_enc(enc, dec[32]),
        decoder_char_from_enc(enc, dec[33]),
        decoder_char_from_enc(enc, dec[34]),
        decoder_char_from_enc(enc, dec[35]),
        decoder_char_from_enc(enc, dec[36]),
        decoder_char_from_enc(enc, dec[37]),
        decoder_char_from_enc(enc, dec[38]),
        decoder_char_from_enc(enc, dec[39]),
        decoder_char_from_enc(enc, dec[40]),
        decoder_char_from_enc(enc, dec[41]),
        decoder_char_from_enc(enc, dec[42]),
        decoder_char_from_enc(enc, dec[43]),
        decoder_char_from_enc(enc, dec[44]),
        decoder_char_from_enc(enc, dec[45]),
        decoder_char_from_enc(enc, dec[46]),
        decoder_char_from_enc(enc, dec[47]),
        decoder_char_from_enc(enc, dec[48]),
        decoder_char_from_enc(enc, dec[49]),
        decoder_char_from_enc(enc, dec[50]),
        decoder_char_from_enc(enc, dec[51]),
        decoder_char_from_enc(enc, dec[52]),
        decoder_char_from_enc(enc, dec[53]),
        decoder_char_from_enc(enc, dec[54]),
        decoder_char_from_enc(enc, dec[55]),
        decoder_char_from_enc(enc, dec[56]),
        decoder_char_from_enc(enc, dec[57]),
        decoder_char_from_enc(enc, dec[58]),
        decoder_char_from_enc(enc, dec[59]),
        decoder_char_from_enc(enc, dec[60]),
        decoder_char_from_enc(enc, dec[61]),
        decoder_char_from_enc(enc, dec[62]),
        decoder_char_from_enc(enc, dec[63]),
        decoder_char_from_enc(enc, dec[64]),
        decoder_char_from_enc(enc, dec[65]),
        decoder_char_from_enc(enc, dec[66]),
        decoder_char_from_enc(enc, dec[67]),
        decoder_char_from_enc(enc, dec[68]),
        decoder_char_from_enc(enc, dec[69]),
        decoder_char_from_enc(enc, dec[70]),
        decoder_char_from_enc(enc, dec[71]),
        decoder_char_from_enc(enc, dec[72]),
        decoder_char_from_enc(enc, dec[73]),
        decoder_char_from_enc(enc, dec[74]),
        decoder_char_from_enc(enc, dec[75]),
        decoder_char_from_enc(enc, dec[76]),
        decoder_char_from_enc(enc, dec[77]),
        decoder_char_from_enc(enc, dec[78]),
        decoder_char_from_enc(enc, dec[79]),
        decoder_char_from_enc(enc, dec[80]),
        decoder_char_from_enc(enc, dec[81]),
        decoder_char_from_enc(enc, dec[82]),
        decoder_char_from_enc(enc, dec[83]),
        decoder_char_from_enc(enc, dec[84]),
        decoder_char_from_enc(enc, dec[85]),
        decoder_char_from_enc(enc, dec[86]),
        decoder_char_from_enc(enc, dec[87]),
        decoder_char_from_enc(enc, dec[88]),
        decoder_char_from_enc(enc, dec[89]),
        decoder_char_from_enc(enc, dec[90]),
        decoder_char_from_enc(enc, dec[91]),
        decoder_char_from_enc(enc, dec[92]),
        decoder_char_from_enc(enc, dec[93]),
        decoder_char_from_enc(enc, dec[94]),
        decoder_char_from_enc(enc, dec[95]),
        decoder_char_from_enc(enc, dec[96]),
        decoder_char_from_enc(enc, dec[97]),
        decoder_char_from_enc(enc, dec[98]),
        decoder_char_from_enc(enc, dec[99]),
        decoder_char_from_enc(enc, dec[100]),
        decoder_char_from_enc(enc, dec[101]),
        decoder_char_from_enc(enc, dec[102]),
        decoder_char_from_enc(enc, dec[103]),
        decoder_char_from_enc(enc, dec[104]),
        decoder_char_from_enc(enc, dec[105]),
        decoder_char_from_enc(enc, dec[106]),
        decoder_char_from_enc(enc, dec[107]),
        decoder_char_from_enc(enc, dec[108]),
        decoder_char_from_enc(enc, dec[109]),
        decoder_char_from_enc(enc, dec[110]),
        decoder_char_from_enc(enc, dec[111]),
        decoder_char_from_enc(enc, dec[112]),
        decoder_char_from_enc(enc, dec[113]),
        decoder_char_from_enc(enc, dec[114]),
        decoder_char_from_enc(enc, dec[115]),
        decoder_char_from_enc(enc, dec[116]),
        decoder_char_from_enc(enc, dec[117]),
        decoder_char_from_enc(enc, dec[118]),
        decoder_char_from_enc(enc, dec[119]),
        decoder_char_from_enc(enc, dec[120]),
        decoder_char_from_enc(enc, dec[121]),
        decoder_char_from_enc(enc, dec[122]),
        decoder_char_from_enc(enc, dec[123]),
        decoder_char_from_enc(enc, dec[124]),
        decoder_char_from_enc(enc, dec[125]),
        decoder_char_from_enc(enc, dec[126]),
        decoder_char_from_enc(enc, dec[127]),
        // the top 128 spots in u8, above ascii, we can just leave as always invalid
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
        INVALID_BYTE,
    ]
}

#[inline]
pub fn bits_or_err(c: u8) -> Result<u64, DecodeError> {
    let o = DEC_CROCKFORD_UPPER[c as usize];
    if o == INVALID_BYTE {
        Err(DecodeError::InvalidChar { char: c as char })
    } else {
        Ok(o as u64)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DecodeError {
    InvalidChar { char: char },
    InvalidLength { length: usize },
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DecodeError::InvalidChar { char } => {
                write!(f, "Invalid char of '{char}'")
            }
            DecodeError::InvalidLength { length } => {
                write!(f, "Invalid length of {length}")
            }
        }
    }
}
