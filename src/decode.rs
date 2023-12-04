use crate::encode::ENC_CROCKFORD_UPPER;
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
    } else if enc[32] == c {
        32
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
fn bits_or_err(i: u8) -> Result<u64, ()> {
    let o = DEC_CROCKFORD_UPPER[i as usize];
    if o == INVALID_BYTE {
        Err(())?
    } else {
        Ok(o as u64)
    }
}

pub fn decode_u64(a: &[u8]) -> Result<u64, ()> {
    #[rustfmt::skip]
    let n = match a.len() {
        13 => {
            bits_or_err(a[ 0])? << 60 |
            bits_or_err(a[ 1])? << 55 |
            bits_or_err(a[ 2])? << 50 |
            bits_or_err(a[ 3])? << 45 |
            bits_or_err(a[ 4])? << 40 |
            bits_or_err(a[ 5])? << 35 |
            bits_or_err(a[ 6])? << 30 |
            bits_or_err(a[ 7])? << 25 |
            bits_or_err(a[ 8])? << 20 |
            bits_or_err(a[ 9])? << 15 |
            bits_or_err(a[10])? << 10 |
            bits_or_err(a[11])? <<  5 |
            bits_or_err(a[12])?
        }
        12 => {
            bits_or_err(a[ 0])? << 55 |
            bits_or_err(a[ 1])? << 50 |
            bits_or_err(a[ 2])? << 45 |
            bits_or_err(a[ 3])? << 40 |
            bits_or_err(a[ 4])? << 35 |
            bits_or_err(a[ 5])? << 30 |
            bits_or_err(a[ 6])? << 25 |
            bits_or_err(a[ 7])? << 20 |
            bits_or_err(a[ 8])? << 15 |
            bits_or_err(a[ 9])? << 10 |
            bits_or_err(a[10])? <<  5 |
            bits_or_err(a[11])?
        }
        11 => {
            bits_or_err(a[ 0])? << 50 |
            bits_or_err(a[ 1])? << 45 |
            bits_or_err(a[ 2])? << 40 |
            bits_or_err(a[ 3])? << 35 |
            bits_or_err(a[ 4])? << 30 |
            bits_or_err(a[ 5])? << 25 |
            bits_or_err(a[ 6])? << 20 |
            bits_or_err(a[ 7])? << 15 |
            bits_or_err(a[ 8])? << 10 |
            bits_or_err(a[ 9])? <<  5 |
            bits_or_err(a[10])?
        }
        10 => {
            bits_or_err(a[ 0])? << 45 |
            bits_or_err(a[ 1])? << 40 |
            bits_or_err(a[ 2])? << 35 |
            bits_or_err(a[ 3])? << 30 |
            bits_or_err(a[ 4])? << 25 |
            bits_or_err(a[ 5])? << 20 |
            bits_or_err(a[ 6])? << 15 |
            bits_or_err(a[ 7])? << 10 |
            bits_or_err(a[ 8])? <<  5 |
            bits_or_err(a[ 9])?
        }
        9 => {
            bits_or_err(a[ 0])? << 40 |
            bits_or_err(a[ 1])? << 35 |
            bits_or_err(a[ 2])? << 30 |
            bits_or_err(a[ 3])? << 25 |
            bits_or_err(a[ 4])? << 20 |
            bits_or_err(a[ 5])? << 15 |
            bits_or_err(a[ 6])? << 10 |
            bits_or_err(a[ 7])? <<  5 |
            bits_or_err(a[ 8])?
        }
        8 => {
            bits_or_err(a[ 0])? << 35 |
            bits_or_err(a[ 1])? << 30 |
            bits_or_err(a[ 2])? << 25 |
            bits_or_err(a[ 3])? << 20 |
            bits_or_err(a[ 4])? << 15 |
            bits_or_err(a[ 5])? << 10 |
            bits_or_err(a[ 6])? <<  5 |
            bits_or_err(a[ 7])?
        }
        7 => {
            bits_or_err(a[ 0])? << 30 |
            bits_or_err(a[ 1])? << 25 |
            bits_or_err(a[ 2])? << 20 |
            bits_or_err(a[ 3])? << 15 |
            bits_or_err(a[ 4])? << 10 |
            bits_or_err(a[ 5])? <<  5 |
            bits_or_err(a[ 6])?
        }
        6 => {
            bits_or_err(a[ 0])? << 25 |
            bits_or_err(a[ 1])? << 20 |
            bits_or_err(a[ 2])? << 15 |
            bits_or_err(a[ 3])? << 10 |
            bits_or_err(a[ 4])? <<  5 |
            bits_or_err(a[ 5])?
        }
        5 => {
            bits_or_err(a[ 0])? << 20 |
            bits_or_err(a[ 1])? << 15 |
            bits_or_err(a[ 2])? << 10 |
            bits_or_err(a[ 3])? <<  5 |
            bits_or_err(a[ 4])?
        }
        4 => {
            bits_or_err(a[ 0])? << 15 |
            bits_or_err(a[ 1])? << 10 |
            bits_or_err(a[ 2])? <<  5 |
            bits_or_err(a[ 3])?
        }
        3 => {
            bits_or_err(a[ 0])? << 10 |
            bits_or_err(a[ 1])? <<  5 |
            bits_or_err(a[ 2])?
        }
        2 => {
            bits_or_err(a[ 0])? <<  5 |
            bits_or_err(a[ 1])?
        }
        1 => {
            #[allow(unused_parens)]
            bits_or_err(a[ 0])?
        }
        0 => 0,
        _ => Err(())?,
    };
    Ok(n)
}

#[allow(dead_code)]
pub fn decode_u64_unsafe(a: &[u8]) -> Result<u64, ()> {
    #[rustfmt::skip]
    let n = match a.len() {
        13 => {
            (DEC_CROCKFORD_UPPER[a[ 0] as usize] as u64) << 60 |
            (DEC_CROCKFORD_UPPER[a[ 1] as usize] as u64) << 55 |
            (DEC_CROCKFORD_UPPER[a[ 2] as usize] as u64) << 50 |
            (DEC_CROCKFORD_UPPER[a[ 3] as usize] as u64) << 45 |
            (DEC_CROCKFORD_UPPER[a[ 4] as usize] as u64) << 40 |
            (DEC_CROCKFORD_UPPER[a[ 5] as usize] as u64) << 35 |
            (DEC_CROCKFORD_UPPER[a[ 6] as usize] as u64) << 30 |
            (DEC_CROCKFORD_UPPER[a[ 7] as usize] as u64) << 25 |
            (DEC_CROCKFORD_UPPER[a[ 8] as usize] as u64) << 20 |
            (DEC_CROCKFORD_UPPER[a[ 9] as usize] as u64) << 15 |
            (DEC_CROCKFORD_UPPER[a[10] as usize] as u64) << 10 |
            (DEC_CROCKFORD_UPPER[a[11] as usize] as u64) <<  5 |
            (DEC_CROCKFORD_UPPER[a[12] as usize] as u64)
        }
        12 => {
            (DEC_CROCKFORD_UPPER[a[ 0] as usize] as u64) << 55 |
            (DEC_CROCKFORD_UPPER[a[ 1] as usize] as u64) << 50 |
            (DEC_CROCKFORD_UPPER[a[ 2] as usize] as u64) << 45 |
            (DEC_CROCKFORD_UPPER[a[ 3] as usize] as u64) << 40 |
            (DEC_CROCKFORD_UPPER[a[ 4] as usize] as u64) << 35 |
            (DEC_CROCKFORD_UPPER[a[ 5] as usize] as u64) << 30 |
            (DEC_CROCKFORD_UPPER[a[ 6] as usize] as u64) << 25 |
            (DEC_CROCKFORD_UPPER[a[ 7] as usize] as u64) << 20 |
            (DEC_CROCKFORD_UPPER[a[ 8] as usize] as u64) << 15 |
            (DEC_CROCKFORD_UPPER[a[ 9] as usize] as u64) << 10 |
            (DEC_CROCKFORD_UPPER[a[10] as usize] as u64) <<  5 |
            (DEC_CROCKFORD_UPPER[a[11] as usize] as u64)
        }
        11 => {
            (DEC_CROCKFORD_UPPER[a[ 0] as usize] as u64) << 50 |
            (DEC_CROCKFORD_UPPER[a[ 1] as usize] as u64) << 45 |
            (DEC_CROCKFORD_UPPER[a[ 2] as usize] as u64) << 40 |
            (DEC_CROCKFORD_UPPER[a[ 3] as usize] as u64) << 35 |
            (DEC_CROCKFORD_UPPER[a[ 4] as usize] as u64) << 30 |
            (DEC_CROCKFORD_UPPER[a[ 5] as usize] as u64) << 25 |
            (DEC_CROCKFORD_UPPER[a[ 6] as usize] as u64) << 20 |
            (DEC_CROCKFORD_UPPER[a[ 7] as usize] as u64) << 15 |
            (DEC_CROCKFORD_UPPER[a[ 8] as usize] as u64) << 10 |
            (DEC_CROCKFORD_UPPER[a[ 9] as usize] as u64) <<  5 |
            (DEC_CROCKFORD_UPPER[a[10] as usize] as u64)
        }
        10 => {
            (DEC_CROCKFORD_UPPER[a[ 0] as usize] as u64) << 45 |
            (DEC_CROCKFORD_UPPER[a[ 1] as usize] as u64) << 40 |
            (DEC_CROCKFORD_UPPER[a[ 2] as usize] as u64) << 35 |
            (DEC_CROCKFORD_UPPER[a[ 3] as usize] as u64) << 30 |
            (DEC_CROCKFORD_UPPER[a[ 4] as usize] as u64) << 25 |
            (DEC_CROCKFORD_UPPER[a[ 5] as usize] as u64) << 20 |
            (DEC_CROCKFORD_UPPER[a[ 6] as usize] as u64) << 15 |
            (DEC_CROCKFORD_UPPER[a[ 7] as usize] as u64) << 10 |
            (DEC_CROCKFORD_UPPER[a[ 8] as usize] as u64) <<  5 |
            (DEC_CROCKFORD_UPPER[a[ 9] as usize] as u64)
        }
        9 => {
            (DEC_CROCKFORD_UPPER[a[ 0] as usize] as u64) << 40 |
            (DEC_CROCKFORD_UPPER[a[ 1] as usize] as u64) << 35 |
            (DEC_CROCKFORD_UPPER[a[ 2] as usize] as u64) << 30 |
            (DEC_CROCKFORD_UPPER[a[ 3] as usize] as u64) << 25 |
            (DEC_CROCKFORD_UPPER[a[ 4] as usize] as u64) << 20 |
            (DEC_CROCKFORD_UPPER[a[ 5] as usize] as u64) << 15 |
            (DEC_CROCKFORD_UPPER[a[ 6] as usize] as u64) << 10 |
            (DEC_CROCKFORD_UPPER[a[ 7] as usize] as u64) <<  5 |
            (DEC_CROCKFORD_UPPER[a[ 8] as usize] as u64)
        }
        8 => {
            (DEC_CROCKFORD_UPPER[a[ 0] as usize] as u64) << 35 |
            (DEC_CROCKFORD_UPPER[a[ 1] as usize] as u64) << 30 |
            (DEC_CROCKFORD_UPPER[a[ 2] as usize] as u64) << 25 |
            (DEC_CROCKFORD_UPPER[a[ 3] as usize] as u64) << 20 |
            (DEC_CROCKFORD_UPPER[a[ 4] as usize] as u64) << 15 |
            (DEC_CROCKFORD_UPPER[a[ 5] as usize] as u64) << 10 |
            (DEC_CROCKFORD_UPPER[a[ 6] as usize] as u64) <<  5 |
            (DEC_CROCKFORD_UPPER[a[ 7] as usize] as u64)
        }
        7 => {
            (DEC_CROCKFORD_UPPER[a[ 0] as usize] as u64) << 30 |
            (DEC_CROCKFORD_UPPER[a[ 1] as usize] as u64) << 25 |
            (DEC_CROCKFORD_UPPER[a[ 2] as usize] as u64) << 20 |
            (DEC_CROCKFORD_UPPER[a[ 3] as usize] as u64) << 15 |
            (DEC_CROCKFORD_UPPER[a[ 4] as usize] as u64) << 10 |
            (DEC_CROCKFORD_UPPER[a[ 5] as usize] as u64) <<  5 |
            (DEC_CROCKFORD_UPPER[a[ 6] as usize] as u64)
        }
        6 => {
            (DEC_CROCKFORD_UPPER[a[ 0] as usize] as u64) << 25 |
            (DEC_CROCKFORD_UPPER[a[ 1] as usize] as u64) << 20 |
            (DEC_CROCKFORD_UPPER[a[ 2] as usize] as u64) << 15 |
            (DEC_CROCKFORD_UPPER[a[ 3] as usize] as u64) << 10 |
            (DEC_CROCKFORD_UPPER[a[ 4] as usize] as u64) <<  5 |
            (DEC_CROCKFORD_UPPER[a[ 5] as usize] as u64)
        }
        5 => {
            (DEC_CROCKFORD_UPPER[a[ 0] as usize] as u64) << 20 |
            (DEC_CROCKFORD_UPPER[a[ 1] as usize] as u64) << 15 |
            (DEC_CROCKFORD_UPPER[a[ 2] as usize] as u64) << 10 |
            (DEC_CROCKFORD_UPPER[a[ 3] as usize] as u64) <<  5 |
            (DEC_CROCKFORD_UPPER[a[ 4] as usize] as u64)
        }
        4 => {
            (DEC_CROCKFORD_UPPER[a[ 0] as usize] as u64) << 15 |
            (DEC_CROCKFORD_UPPER[a[ 1] as usize] as u64) << 10 |
            (DEC_CROCKFORD_UPPER[a[ 2] as usize] as u64) <<  5 |
            (DEC_CROCKFORD_UPPER[a[ 3] as usize] as u64)
        }
        3 => {
            (DEC_CROCKFORD_UPPER[a[ 0] as usize] as u64) << 10 |
            (DEC_CROCKFORD_UPPER[a[ 1] as usize] as u64) <<  5 |
            (DEC_CROCKFORD_UPPER[a[ 2] as usize] as u64)
        }
        2 => {
            (DEC_CROCKFORD_UPPER[a[ 0] as usize] as u64) <<  5 |
            (DEC_CROCKFORD_UPPER[a[ 1] as usize] as u64)
        }
        1 => {
            #[allow(unused_parens)]
            (DEC_CROCKFORD_UPPER[a[ 0] as usize] as u64)
        }
        0 => 0,
        _ => Err(())?,
    };
    Ok(n)
}
