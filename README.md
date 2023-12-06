# fast32

Base32 and base64 encoding in Rust. Primarily for integer (u64, u128) and UUID identifiers (behind feature `uuid`), as well as arbitrary bytes. And do it all very quickly (more on this below).

Note that by default, encoding an integer into base32 or base64 via normal algorithms does not "look like" a number -- notably the rightmost character usually looks off, and there are sometimes more characters than there needs to be. This might be a plus for obfuscation, barely, but it makes them hard to reason about quickly, and it's also more efficient (slightly) to process them as integers rather than arbitrary arrays of bytes (because we know upfront that integers are always a small size).

For example, the integer `31` processed normally, as bytes, into official [RFC 4648](https://datatracker.ietf.org/doc/html/rfc4648) base32 hex, without padding, will come out as `"D4"`. In contrast, processing it as an integer, as this library can, will come out as `"Z"` which is more intuitively one less than `32` at `"10"`, as one might hope (vs `"EA"` in base32 hex -- note that's an `A` not a `4` so string changed nonintuitively for an increment of 1). This is helpful with "nice looking" urls of base32 encodings of identifiers, etc.

### Speed

This is intended to be as fast as basically possible, while still keeping an intuitive interface. It is, per the bench comparisons in this repo, on my machine, about 55-100%+ faster than the closest [alternative](https://github.com/archer884/crockford) for decoding u64s, and 20-60% faster for encoding u64s. It is also 26-28% faster on decoding raw bytes and 37-40% faster on encoding raw bytes than the closest [other alternative](https://github.com/ia0/data-encoding) (that is significantly more mature but does not offer this crate's integer encoding/decoding, on top of the slower performance). There is an earlier and [seemingly long abandoned](https://github.com/andreasots/base32) alternative that is vastly slower than all of these, but still somewhat popular.

### Summary

In short, this crate should do everything you want for base32 and base64 encoding (please [raise an issue](https://github.com/rogusdev/fast32/issues) if it doesn't!) while doing all of it very quickly and conveniently.
