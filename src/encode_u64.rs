use crate::ENC_CROCKFORD_UPPER;

const B32_U64_C12_MSK: u64 = 0b1111000000000000000000000000000000000000000000000000000000000000;
const B32_U64_C12_MIN: u64 = 0b0001000000000000000000000000000000000000000000000000000000000000;
const B32_U64_C11_MSK: u64 = 0b0000111110000000000000000000000000000000000000000000000000000000;
const B32_U64_C11_MIN: u64 = 0b0000000010000000000000000000000000000000000000000000000000000000;
const B32_U64_C10_MSK: u64 = 0b0000000001111100000000000000000000000000000000000000000000000000;
const B32_U64_C10_MIN: u64 = 0b0000000000000100000000000000000000000000000000000000000000000000;
const B32_U64_C09_MSK: u64 = 0b0000000000000011111000000000000000000000000000000000000000000000;
const B32_U64_C09_MIN: u64 = 0b0000000000000000001000000000000000000000000000000000000000000000;
const B32_U64_C08_MSK: u64 = 0b0000000000000000000111110000000000000000000000000000000000000000;
const B32_U64_C08_MIN: u64 = 0b0000000000000000000000010000000000000000000000000000000000000000;
const B32_U64_C07_MSK: u64 = 0b0000000000000000000000001111100000000000000000000000000000000000;
const B32_U64_C07_MIN: u64 = 0b0000000000000000000000000000100000000000000000000000000000000000;
const B32_U64_C06_MSK: u64 = 0b0000000000000000000000000000011111000000000000000000000000000000;
const B32_U64_C06_MIN: u64 = 0b0000000000000000000000000000000001000000000000000000000000000000;
const B32_U64_C05_MSK: u64 = 0b0000000000000000000000000000000000111110000000000000000000000000;
const B32_U64_C05_MIN: u64 = 0b0000000000000000000000000000000000000010000000000000000000000000;
const B32_U64_C04_MSK: u64 = 0b0000000000000000000000000000000000000001111100000000000000000000;
const B32_U64_C04_MIN: u64 = 0b0000000000000000000000000000000000000000000100000000000000000000;
const B32_U64_C03_MSK: u64 = 0b0000000000000000000000000000000000000000000011111000000000000000;
const B32_U64_C03_MIN: u64 = 0b0000000000000000000000000000000000000000000000001000000000000000;
const B32_U64_C02_MSK: u64 = 0b0000000000000000000000000000000000000000000000000111110000000000;
const B32_U64_C02_MIN: u64 = 0b0000000000000000000000000000000000000000000000000000010000000000;
const B32_U64_C01_MSK: u64 = 0b0000000000000000000000000000000000000000000000000000001111100000;
const B32_U64_C01_MIN: u64 = 0b0000000000000000000000000000000000000000000000000000000000100000;
const B32_U64_C00_MSK: u64 = 0b0000000000000000000000000000000000000000000000000000000000011111;

#[rustfmt::skip]
#[inline]
pub fn encode_u64(n: u64) -> String {
    let b = {
        if n >= B32_U64_C12_MIN {
            [
                ENC_CROCKFORD_UPPER[((n & B32_U64_C12_MSK) >> 60) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C11_MSK) >> 55) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C10_MSK) >> 50) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C09_MSK) >> 45) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C08_MSK) >> 40) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C07_MSK) >> 35) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C06_MSK) >> 30) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C05_MSK) >> 25) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C04_MSK) >> 20) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C03_MSK) >> 15) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C02_MSK) >> 10) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C01_MSK) >>  5) as usize],
                ENC_CROCKFORD_UPPER[( n & B32_U64_C00_MSK       ) as usize],
            ]
            .to_vec()
        } else if n >= B32_U64_C11_MIN {
            [
                ENC_CROCKFORD_UPPER[((n & B32_U64_C11_MSK) >> 55) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C10_MSK) >> 50) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C09_MSK) >> 45) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C08_MSK) >> 40) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C07_MSK) >> 35) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C06_MSK) >> 30) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C05_MSK) >> 25) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C04_MSK) >> 20) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C03_MSK) >> 15) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C02_MSK) >> 10) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C01_MSK) >>  5) as usize],
                ENC_CROCKFORD_UPPER[( n & B32_U64_C00_MSK       ) as usize],
            ]
            .to_vec()
        } else if n >= B32_U64_C10_MIN {
            [
                ENC_CROCKFORD_UPPER[((n & B32_U64_C10_MSK) >> 50) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C09_MSK) >> 45) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C08_MSK) >> 40) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C07_MSK) >> 35) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C06_MSK) >> 30) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C05_MSK) >> 25) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C04_MSK) >> 20) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C03_MSK) >> 15) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C02_MSK) >> 10) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C01_MSK) >>  5) as usize],
                ENC_CROCKFORD_UPPER[( n & B32_U64_C00_MSK       ) as usize],
            ]
            .to_vec()
        } else if n >= B32_U64_C09_MIN {
            [
                ENC_CROCKFORD_UPPER[((n & B32_U64_C09_MSK) >> 45) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C08_MSK) >> 40) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C07_MSK) >> 35) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C06_MSK) >> 30) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C05_MSK) >> 25) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C04_MSK) >> 20) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C03_MSK) >> 15) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C02_MSK) >> 10) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C01_MSK) >>  5) as usize],
                ENC_CROCKFORD_UPPER[( n & B32_U64_C00_MSK       ) as usize],
            ]
            .to_vec()
        } else if n >= B32_U64_C08_MIN {
            [
                ENC_CROCKFORD_UPPER[((n & B32_U64_C08_MSK) >> 40) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C07_MSK) >> 35) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C06_MSK) >> 30) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C05_MSK) >> 25) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C04_MSK) >> 20) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C03_MSK) >> 15) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C02_MSK) >> 10) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C01_MSK) >>  5) as usize],
                ENC_CROCKFORD_UPPER[( n & B32_U64_C00_MSK       ) as usize],
            ]
            .to_vec()
        } else if n >= B32_U64_C07_MIN {
            [
                ENC_CROCKFORD_UPPER[((n & B32_U64_C07_MSK) >> 35) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C06_MSK) >> 30) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C05_MSK) >> 25) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C04_MSK) >> 20) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C03_MSK) >> 15) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C02_MSK) >> 10) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C01_MSK) >>  5) as usize],
                ENC_CROCKFORD_UPPER[( n & B32_U64_C00_MSK       ) as usize],
            ]
            .to_vec()
        } else if n >= B32_U64_C06_MIN {
            [
                ENC_CROCKFORD_UPPER[((n & B32_U64_C06_MSK) >> 30) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C05_MSK) >> 25) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C04_MSK) >> 20) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C03_MSK) >> 15) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C02_MSK) >> 10) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C01_MSK) >>  5) as usize],
                ENC_CROCKFORD_UPPER[( n & B32_U64_C00_MSK       ) as usize],
            ]
            .to_vec()
        } else if n >= B32_U64_C05_MIN {
            [
                ENC_CROCKFORD_UPPER[((n & B32_U64_C05_MSK) >> 25) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C04_MSK) >> 20) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C03_MSK) >> 15) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C02_MSK) >> 10) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C01_MSK) >>  5) as usize],
                ENC_CROCKFORD_UPPER[( n & B32_U64_C00_MSK       ) as usize],
            ]
            .to_vec()
        } else if n >= B32_U64_C04_MIN {
            [
                ENC_CROCKFORD_UPPER[((n & B32_U64_C04_MSK) >> 20) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C03_MSK) >> 15) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C02_MSK) >> 10) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C01_MSK) >>  5) as usize],
                ENC_CROCKFORD_UPPER[( n & B32_U64_C00_MSK       ) as usize],
            ]
            .to_vec()
        } else if n >= B32_U64_C03_MIN {
            [
                ENC_CROCKFORD_UPPER[((n & B32_U64_C03_MSK) >> 15) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C02_MSK) >> 10) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C01_MSK) >>  5) as usize],
                ENC_CROCKFORD_UPPER[( n & B32_U64_C00_MSK       ) as usize],
            ]
            .to_vec()
        } else if n >= B32_U64_C02_MIN {
            [
                ENC_CROCKFORD_UPPER[((n & B32_U64_C02_MSK) >> 10) as usize],
                ENC_CROCKFORD_UPPER[((n & B32_U64_C01_MSK) >>  5) as usize],
                ENC_CROCKFORD_UPPER[( n & B32_U64_C00_MSK       ) as usize],
            ]
            .to_vec()
        } else if n >= B32_U64_C01_MIN {
            [
                ENC_CROCKFORD_UPPER[((n & B32_U64_C01_MSK) >>  5) as usize],
                ENC_CROCKFORD_UPPER[( n & B32_U64_C00_MSK       ) as usize],
            ]
            .to_vec()
        } else {
            [
                ENC_CROCKFORD_UPPER[( n & B32_U64_C00_MSK       ) as usize],
            ]
            .to_vec()
        }
    };
    unsafe { String::from_utf8_unchecked(b) }
}
