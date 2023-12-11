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

pub use self::alphabet::{Alphabet64Nopad, Alphabet64Padded};

/// RFC 4648 Base64 normal, with padding
///
/// `"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"` and `'='`
///
/// [https://datatracker.ietf.org/doc/html/rfc4648#section-4](https://datatracker.ietf.org/doc/html/rfc4648#section-4)
pub use self::alphabet::RFC4648;

/// RFC 4648 Base64 normal, no padding
///
/// `"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"`
///
/// [https://datatracker.ietf.org/doc/html/rfc4648#section-4](https://datatracker.ietf.org/doc/html/rfc4648#section-4)
pub use self::alphabet::RFC4648_NOPAD;

/// RFC 4648 Base64 "url safe" form, with padding
///
/// `"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_"` and `'='`
///
/// [https://datatracker.ietf.org/doc/html/rfc4648#section-5](https://datatracker.ietf.org/doc/html/rfc4648#section-5)
pub use self::alphabet::RFC4648_URL;

/// RFC 4648 Base64 "url safe" form, no padding
///
/// `"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_"`
///
/// [https://datatracker.ietf.org/doc/html/rfc4648#section-5](https://datatracker.ietf.org/doc/html/rfc4648#section-5)
pub use self::alphabet::RFC4648_URL_NOPAD;
