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

pub use self::alphabet::Alphabet64;
pub use self::alphabet::RFC4648;
pub use self::alphabet::RFC4648_NOPAD;
pub use self::alphabet::RFC4648_URL;
pub use self::alphabet::RFC4648_URL_NOPAD;

#[cfg(feature = "uuid")]
pub use self::uuid::{decode_uuid, encode_uuid};
