mod alphabet;
mod decode_bytes;
mod decode_u128;
mod decode_u64;
mod encode_bytes;
mod encode_u128;
mod encode_u64;

#[cfg(feature = "uuid")]
mod uuid;

pub use self::decode_bytes::{capacity_decode, decode, decode_into};
pub use self::decode_u128::decode_u128;
pub use self::decode_u64::decode_u64;
pub use self::encode_bytes::{capacity_encode, encode, encode_into};
pub use self::encode_u128::{capacity_encode_u128, encode_u128, encode_u128_into};
pub use self::encode_u64::{capacity_encode_u64, encode_u64, encode_u64_into};

#[cfg(feature = "uuid")]
pub use self::uuid::{decode_uuid, encode_uuid, encode_uuid_into};

pub use self::alphabet::{Alphabet32Nopad, Alphabet32Padded};

/// Crockford Base32 (no padding)
///
/// [https://www.crockford.com/base32.html](https://www.crockford.com/base32.html)
pub use self::alphabet::CROCKFORD;

/// Crockford Base32, lowercase (no padding)
///
/// Same as [upper version](self::CROCKFORD), just lowercase
pub use self::alphabet::CROCKFORD_LOWER;

/// RFC 4648 Base32 normal, with padding
///
/// `"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567"` and `'='`
///
/// [https://datatracker.ietf.org/doc/html/rfc4648#section-6](https://datatracker.ietf.org/doc/html/rfc4648#section-6)
pub use self::alphabet::RFC4648;

/// RFC 4648 Base32 normal, no padding
///
/// `"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567"`
///
/// [https://datatracker.ietf.org/doc/html/rfc4648#section-6](https://datatracker.ietf.org/doc/html/rfc4648#section-6)
pub use self::alphabet::RFC4648_NOPAD;

/// RFC 4648 Base32 "hex" form, with padding
///
/// `"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567"` and `'='`
///
/// [https://datatracker.ietf.org/doc/html/rfc4648#section-7](https://datatracker.ietf.org/doc/html/rfc4648#section-7)
pub use self::alphabet::RFC4648_HEX;

/// RFC 4648 Base32 "hex" form, no padding
///
/// `"0123456789ABCDEFGHIJKLMNOPQRSTUV"`
///
/// [https://datatracker.ietf.org/doc/html/rfc4648#section-7](https://datatracker.ietf.org/doc/html/rfc4648#section-7)
pub use self::alphabet::RFC4648_HEX_NOPAD;
