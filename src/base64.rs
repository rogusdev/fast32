mod alphabet;
mod decode_bytes;
mod decode_u128;
mod decode_u64;
mod encode_bytes;
mod encode_u128;
mod encode_u64;

#[cfg(feature = "uuid")]
mod uuid;

pub use self::decode_bytes::{decode_bytes, decode_bytes_into};
pub use self::decode_u128::decode_u128;
pub use self::decode_u64::decode_u64;
pub use self::encode_bytes::{encode_bytes, encode_bytes_into};
pub use self::encode_u128::{capacity_u128, encode_u128, encode_u128_into};
pub use self::encode_u64::{capacity_u64, encode_u64, encode_u64_into};

pub use self::alphabet::{decoder_map, decoder_map_simple, Alphabet};
pub use self::alphabet::RFC4648;
pub use self::alphabet::RFC4648_NOPAD;
pub use self::alphabet::RFC4648_URL;
pub use self::alphabet::RFC4648_URL_NOPAD;

#[cfg(feature = "uuid")]
pub use self::uuid::{decode_uuid, decode_uuid_str, encode_uuid};
