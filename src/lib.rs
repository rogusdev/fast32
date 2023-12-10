//! Base32 and base64 encoding in Rust. Primarily for integer (u64, u128)
//! and UUID identifiers (behind feature `uuid`), as well as arbitrary bytes.
//!
//! And do it all very quickly
//! (more on this in the [README](https://github.com/rogusdev/fast32#speed)).
//!
//! Examples:
//! ```
//! use uuid::Uuid;
//! use fast32::base32::CROCKFORD;
//!
//! assert_eq!(CROCKFORD.encode_u64(31), "Z");
//! assert_eq!(CROCKFORD.decode_u64(b"Z").unwrap(), 31);
//! assert_eq!(CROCKFORD.decode_u64_str("Z").unwrap(), 31);
//!
//! // &[0x1F] is 31
//! assert_eq!(CROCKFORD.encode_bytes(&[0x1F]), "3W");
//! assert_eq!(CROCKFORD.decode_bytes(b"3W").unwrap(), &[0x1F]);
//!
//! let u = Uuid::nil();
//! assert_eq!(CROCKFORD.encode_uuid(u), "0");
//! assert_eq!(CROCKFORD.decode_uuid(b"0").unwrap(), u);
//! let u = Uuid::max();
//! assert_eq!(CROCKFORD.encode_uuid(u), "7ZZZZZZZZZZZZZZZZZZZZZZZZZ");
//! assert_eq!(CROCKFORD.decode_uuid_str("7ZZZZZZZZZZZZZZZZZZZZZZZZZ").unwrap(), u);
//! ```
//!
//! With padding:
//! ```
//! use fast32::base32::RFC4648;
//! assert_eq!(RFC4648.encode_bytes(&[0x1F]), "D4======");
//! assert_eq!(RFC4648.decode_bytes(b"D4======").unwrap(), &[0x1F]);
//! ```
//!
//! Base64, without padding (needed for u64 + u128 encode/decode) and with padding:
//! ```
//! use fast32::base64::{RFC4648, RFC4648_NOPAD};
//! assert_eq!(RFC4648_NOPAD.encode_u64(31), "f");
//! assert_eq!(RFC4648_NOPAD.decode_u64(b"f").unwrap(), 31);
//!
//! assert_eq!(RFC4648.encode_bytes(&[0x1F]), "Hw==");
//! assert_eq!(RFC4648.decode_bytes(b"Hw==").unwrap(), &[0x1F]);
//! ```
//!
//! Also directly writing bytes into existing arrays:
//! ```
//! use fast32::base32::CROCKFORD;
//!
//! let n = &[
//!     0x00, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB,
//!     0xCD,
//! ];
//! let x = "00938NKRJ2NWVVRJ6HB7H45BSM";
//!
//! let mut b = Vec::<u8>::with_capacity(26);
//! CROCKFORD.encode_bytes_into(n, &mut b);
//! assert_eq!(&b[0..26], x.as_bytes());
//! assert_eq!(CROCKFORD.decode_bytes(&b[0..26]).unwrap(), n);
//!
//! let mut b = Vec::<u8>::with_capacity(27);
//! b.push(201);
//! CROCKFORD.encode_bytes_into(n, &mut b);
//! b.push(202);
//! assert_eq!(String::from_utf8(b[1..27].to_vec()).unwrap(), x);
//! assert_eq!(&b[1..27], x.as_bytes());
//! assert_eq!(CROCKFORD.decode_bytes(&b[1..27]).unwrap(), n);
//!
//! let mut b = Vec::<u8>::with_capacity(17);
//! b.push(201);
//! CROCKFORD.decode_bytes_into(x.as_bytes(), &mut b).unwrap();
//! b.push(202);
//! assert_eq!(&b[1..17], n);
//! ```
//!
//! The [tests](https://github.com/rogusdev/fast32/tree/main/tests)
//! have numerous examples that might help further.
//!
//! ### Encoding integers
//!
//! Note that by default, encoding an integer into base32 or base64 via normal algorithms
//! does not "look like" a number -- notably the rightmost character usually looks off,
//! and there are sometimes more characters than there needs to be.
//! This might be a plus for obfuscation, barely, but it makes them hard to reason about quickly,
//! and it's also more efficient to process them as integers rather than arbitrary arrays of bytes
//! (because we know upfront that integers are always a small size).
//!
//! For example, the normal/base10 integer `31` processed normally, as bytes, into official
//! [RFC 4648](https://datatracker.ietf.org/doc/html/rfc4648) base32 hex, without padding,
//! will come out as `"D4"`. In contrast, processing it as an integer, as this library can,
//! will come out as `"Z"` (in Crockford's base32 alphabet) which is more intuitively
//! one less than `32` at `"10"`, as one might hope (vs `"EA"` in base32 hex -- note
//! that's an `A` not a `4` so the string changed nonintuitively for an increment of 1).
//! This is helpful with "nice looking" urls of base32 encodings of identifiers, etc.
//!
//! ### Summary
//!
//! In short, this crate should do everything you want for base32 and base64 encoding
//! (please [raise an issue](https://github.com/rogusdev/fast32/issues) if it doesn't!)
//! while doing all of it very quickly and conveniently.

/// Base32 functions and constants for specific encodings
pub mod base32;
/// Base64 functions and constants for specific encodings
pub mod base64;

mod shared;

pub use crate::shared::{decoder_map, decoder_map_simple, DecodeError, INVALID_BYTE};
