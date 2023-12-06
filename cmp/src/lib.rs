#[cfg(test)]
mod tests {
    const CROCKFORD32: data_encoding::Encoding = data_encoding_macro::new_encoding!{
        symbols: "0123456789ABCDEFGHJKMNPQRSTVWXYZ",
        translate_from: "abcdefghjkmnpqrstvwxyzLIOlio",
        translate_to: "ABCDEFGHJKMNPQRSTVWXYZ110110",
    };

    #[test]
    fn compare_crockford_crate_u64() {
        let rs = [
            (u64::MAX - 100000)..=u64::MAX,
            ((1 << 60) - 10000)..=((1 << 60) + 10000),
            ((1 << 55) - 10000)..=((1 << 55) + 10000),
            ((1 << 50) - 10000)..=((1 << 50) + 10000),
            ((1 << 45) - 10000)..=((1 << 45) + 10000),
            ((1 << 40) - 10000)..=((1 << 40) + 10000),
            ((1 << 35) - 10000)..=((1 << 35) + 10000),
            ((1 << 30) - 10000)..=((1 << 30) + 10000),
            ((1 << 25) - 10000)..=((1 << 25) + 10000),
            ((1 << 20) - 10000)..=((1 << 20) + 10000),
            0..=((1 << 16) + 10000),  // first 4 chars
        ];

        for r in rs {
            for n in r {
                let c = crockford::encode(n);
                let e = fast32::base32::CROCKFORD.encode_u64(n);
                assert_eq!(c, e, "mismatch encode for {n}: {c} vs {e}");

                let c = crockford::decode(c).unwrap();
                let e = fast32::base32::CROCKFORD.decode_u64(e.as_bytes()).unwrap();
                assert_eq!(c, e, "mismatch decode for {n}: {c} vs {e}");
            }
        }
    }

    #[test]
    fn compare_base32_bytes_crockford() {
        {
            // rem 4
            let n = b"The quick brown fox jumps over the lazy dog.";
            let x = "AHM6A83HENMP6TS0C9S6YXVE41K6YY10D9TPTW3K41QQCSBJ41T6GS90DHGQMY90CHQPEBG";

            let s = base32::encode(base32::Alphabet::Crockford, n);
            assert_eq!(s, x);

            let s = fast32::base32::CROCKFORD.encode_bytes(n);
            assert_eq!(s, x);
            assert_eq!(fast32::base32::CROCKFORD.decode_bytes(s.as_bytes()).unwrap(), n);

            let s = CROCKFORD32.encode(n);
            assert_eq!(s, x);
            assert_eq!(CROCKFORD32.decode(s.as_bytes()).unwrap(), n);
        }

        {
            // rem 3
            let n = b"The quick brown fox jumps over the lazy dog";
            let x = "AHM6A83HENMP6TS0C9S6YXVE41K6YY10D9TPTW3K41QQCSBJ41T6GS90DHGQMY90CHQPE";

            let s = base32::encode(base32::Alphabet::Crockford, n);
            assert_eq!(s, x);

            let s = fast32::base32::CROCKFORD.encode_bytes(n);
            assert_eq!(s, x);
            assert_eq!(fast32::base32::CROCKFORD.decode_bytes(s.as_bytes()).unwrap(), n);

            let s = CROCKFORD32.encode(n);
            assert_eq!(s, x);
            assert_eq!(CROCKFORD32.decode(s.as_bytes()).unwrap(), n);
        }

        {
            // rem 2
            let n = b"The quick brown fox jumps over the lazy do";
            let x = "AHM6A83HENMP6TS0C9S6YXVE41K6YY10D9TPTW3K41QQCSBJ41T6GS90DHGQMY90CHQG";

            let s = base32::encode(base32::Alphabet::Crockford, n);
            assert_eq!(s, x);

            let s = fast32::base32::CROCKFORD.encode_bytes(n);
            assert_eq!(s, x);
            assert_eq!(fast32::base32::CROCKFORD.decode_bytes(s.as_bytes()).unwrap(), n);

            let s = CROCKFORD32.encode(n);
            assert_eq!(s, x);
            assert_eq!(CROCKFORD32.decode(s.as_bytes()).unwrap(), n);
        }

        {
            // rem 1
            let n = b"The quick brown fox jumps over the lazy d";
            let x = "AHM6A83HENMP6TS0C9S6YXVE41K6YY10D9TPTW3K41QQCSBJ41T6GS90DHGQMY90CG";

            let s = base32::encode(base32::Alphabet::Crockford, n);
            assert_eq!(s, x);

            let s = fast32::base32::CROCKFORD.encode_bytes(n);
            assert_eq!(s, x);
            assert_eq!(fast32::base32::CROCKFORD.decode_bytes(s.as_bytes()).unwrap(), n);

            let s = CROCKFORD32.encode(n);
            assert_eq!(s, x);
            assert_eq!(CROCKFORD32.decode(s.as_bytes()).unwrap(), n);
        }

        let rs = [
            (u64::MAX - 100000)..=u64::MAX,
            ((1 << 60) - 10000)..=((1 << 60) + 10000),
            ((1 << 55) - 10000)..=((1 << 55) + 10000),
            ((1 << 50) - 10000)..=((1 << 50) + 10000),
            ((1 << 45) - 10000)..=((1 << 45) + 10000),
            ((1 << 40) - 10000)..=((1 << 40) + 10000),
            ((1 << 35) - 10000)..=((1 << 35) + 10000),
            ((1 << 30) - 10000)..=((1 << 30) + 10000),
            ((1 << 25) - 10000)..=((1 << 25) + 10000),
            ((1 << 20) - 10000)..=((1 << 20) + 10000),
            0..=((1 << 16) + 10000),  // first 4 chars
        ];

        for r in rs {
            for n in r {
                let b = &n.to_be_bytes();
                let c = base32::encode(base32::Alphabet::Crockford, b);
                let e = fast32::base32::CROCKFORD.encode_bytes(b);
                assert_eq!(c, e, "mismatch encode for {n}: {c} vs {e}");
                let c = CROCKFORD32.encode(b);
                assert_eq!(c, e, "mismatch encode for {n}: {c} vs {e}");
                let d = fast32::base32::CROCKFORD.decode_bytes(e.as_bytes()).unwrap();
                assert_eq!(b.to_vec(), d, "mismatch decode for {n}: {b:?} vs {d:?}");
            }
        }
    }

    #[test]
    fn compare_base32_bytes_rfc4648pad() {
        {
            // rem 4
            let n = b"The quick brown fox jumps over the lazy dog.";
            let x = "KRUGKIDROVUWG2ZAMJZG653OEBTG66BANJ2W24DTEBXXMZLSEB2GQZJANRQXU6JAMRXWOLQ=";

            let s = base32::encode(base32::Alphabet::RFC4648 { padding: true }, n);
            assert_eq!(s, x);

            let s = fast32::base32::RFC4648.encode_bytes(n);
            assert_eq!(s, x);
            assert_eq!(fast32::base32::RFC4648.decode_bytes(s.as_bytes()).unwrap(), n);

            let s = data_encoding::BASE32.encode(n);
            assert_eq!(s, x);
            assert_eq!(data_encoding::BASE32.decode(s.as_bytes()).unwrap(), n);
        }

        {
            // rem 3
            let n = b"The quick brown fox jumps over the lazy dog";
            let x = "KRUGKIDROVUWG2ZAMJZG653OEBTG66BANJ2W24DTEBXXMZLSEB2GQZJANRQXU6JAMRXWO===";

            let s = base32::encode(base32::Alphabet::RFC4648 { padding: true }, n);
            assert_eq!(s, x);

            let s = fast32::base32::RFC4648.encode_bytes(n);
            assert_eq!(s, x);
            assert_eq!(fast32::base32::RFC4648.decode_bytes(s.as_bytes()).unwrap(), n);

            let s = data_encoding::BASE32.encode(n);
            assert_eq!(s, x);
            assert_eq!(data_encoding::BASE32.decode(s.as_bytes()).unwrap(), n);
        }

        {
            // rem 2
            let n = b"The quick brown fox jumps over the lazy do";
            let x = "KRUGKIDROVUWG2ZAMJZG653OEBTG66BANJ2W24DTEBXXMZLSEB2GQZJANRQXU6JAMRXQ====";

            let s = base32::encode(base32::Alphabet::RFC4648 { padding: true }, n);
            assert_eq!(s, x);

            let s = fast32::base32::RFC4648.encode_bytes(n);
            assert_eq!(s, x);
            assert_eq!(fast32::base32::RFC4648.decode_bytes(s.as_bytes()).unwrap(), n);

            let s = data_encoding::BASE32.encode(n);
            assert_eq!(s, x);
            assert_eq!(data_encoding::BASE32.decode(s.as_bytes()).unwrap(), n);
        }

        {
            // rem 1
            let n = b"The quick brown fox jumps over the lazy d";
            let x = "KRUGKIDROVUWG2ZAMJZG653OEBTG66BANJ2W24DTEBXXMZLSEB2GQZJANRQXU6JAMQ======";

            let s = base32::encode(base32::Alphabet::RFC4648 { padding: true }, n);
            assert_eq!(s, x);

            let s = fast32::base32::RFC4648.encode_bytes(n);
            assert_eq!(s, x);
            assert_eq!(fast32::base32::RFC4648.decode_bytes(s.as_bytes()).unwrap(), n);

            let s = data_encoding::BASE32.encode(n);
            assert_eq!(s, x);
            assert_eq!(data_encoding::BASE32.decode(s.as_bytes()).unwrap(), n);
        }

        let rs = [
            (u64::MAX - 100000)..=u64::MAX,
            ((1 << 60) - 10000)..=((1 << 60) + 10000),
            ((1 << 55) - 10000)..=((1 << 55) + 10000),
            ((1 << 50) - 10000)..=((1 << 50) + 10000),
            ((1 << 45) - 10000)..=((1 << 45) + 10000),
            ((1 << 40) - 10000)..=((1 << 40) + 10000),
            ((1 << 35) - 10000)..=((1 << 35) + 10000),
            ((1 << 30) - 10000)..=((1 << 30) + 10000),
            ((1 << 25) - 10000)..=((1 << 25) + 10000),
            ((1 << 20) - 10000)..=((1 << 20) + 10000),
            0..=((1 << 16) + 10000),  // first 4 chars
        ];

        for r in rs {
            for n in r {
                let b = &n.to_be_bytes();
                let c = base32::encode(base32::Alphabet::RFC4648 { padding: true }, b);
                let e = fast32::base32::RFC4648.encode_bytes(b);
                assert_eq!(c, e, "mismatch encode for {n}: {c} vs {e}");
                let c = data_encoding::BASE32.encode(b);
                assert_eq!(c, e, "mismatch encode for {n}: {c} vs {e}");
                let d = fast32::base32::RFC4648.decode_bytes(e.as_bytes()).unwrap();
                assert_eq!(b.to_vec(), d, "mismatch decode for {n}: {b:?} vs {d:?}");
            }
        }
    }
}
