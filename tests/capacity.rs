mod base32 {
    use fast32::base32::{
        capacity_decode, capacity_encode, capacity_encode_u128, capacity_encode_u64,
    };

    #[test]
    fn test_capacity_encode() {
        assert_eq!(capacity_encode(&vec![0u8; 0]), 0);

        assert_eq!(capacity_encode(&vec![0u8; 1]), 2);
        assert_eq!(capacity_encode(&vec![0u8; 2]), 4);
        assert_eq!(capacity_encode(&vec![0u8; 3]), 5);
        assert_eq!(capacity_encode(&vec![0u8; 4]), 7);
        assert_eq!(capacity_encode(&vec![0u8; 5]), 8);

        assert_eq!(capacity_encode(&vec![0u8; 6]), 10);
        assert_eq!(capacity_encode(&vec![0u8; 7]), 12);
        assert_eq!(capacity_encode(&vec![0u8; 8]), 13);
        assert_eq!(capacity_encode(&vec![0u8; 9]), 15);
        assert_eq!(capacity_encode(&vec![0u8; 10]), 16);
    }

    #[test]
    fn test_capacity_decode() {
        assert_eq!(capacity_decode(&vec![0u8; 0]), 0);

        // Note that a 1 byte array would not be valid encoded base32, and similarly for other lengths

        assert_eq!(capacity_decode(&vec![0u8; 2]), 1);
        assert_eq!(capacity_decode(&vec![0u8; 4]), 2);
        assert_eq!(capacity_decode(&vec![0u8; 5]), 3);
        assert_eq!(capacity_decode(&vec![0u8; 7]), 4);
        assert_eq!(capacity_decode(&vec![0u8; 8]), 5);

        assert_eq!(capacity_decode(&vec![0u8; 10]), 6);
        assert_eq!(capacity_decode(&vec![0u8; 12]), 7);
        assert_eq!(capacity_decode(&vec![0u8; 13]), 8);
        assert_eq!(capacity_decode(&vec![0u8; 15]), 9);
        assert_eq!(capacity_decode(&vec![0u8; 16]), 10);
    }

    #[test]
    fn test_capacity_encode_u64() {
        assert_eq!(capacity_encode_u64(0), 1);
        assert_eq!(capacity_encode_u64(1), 1);
        assert_eq!(capacity_encode_u64(2), 1);

        assert_eq!(capacity_encode_u64(31), 1);
        assert_eq!(capacity_encode_u64(32), 2);
        assert_eq!(capacity_encode_u64(33), 2);

        assert_eq!(capacity_encode_u64(63), 2);
        assert_eq!(capacity_encode_u64(64), 2);
        assert_eq!(capacity_encode_u64(65), 2);

        assert_eq!(capacity_encode_u64(32 * 32 - 1), 2);
        assert_eq!(capacity_encode_u64(32 * 32), 3);
        assert_eq!(capacity_encode_u64(32 * 32 + 1), 3);

        assert_eq!(capacity_encode_u64(64 * 64 - 1), 3);
        assert_eq!(capacity_encode_u64(64 * 64), 3);
        assert_eq!(capacity_encode_u64(64 * 64 + 1), 3);

        // 32^6 == 64^5
        assert_eq!(capacity_encode_u64(32 * 32 * 32 * 32 * 32 * 32 - 1), 6);
        assert_eq!(capacity_encode_u64(32 * 32 * 32 * 32 * 32 * 32), 7);
        assert_eq!(capacity_encode_u64(32 * 32 * 32 * 32 * 32 * 32 + 1), 7);

        assert_eq!(capacity_encode_u64(u64::MAX), 13);
    }

    #[test]
    fn test_capacity_encode_u128() {
        assert_eq!(capacity_encode_u128(0), 1);
        assert_eq!(capacity_encode_u128(1), 1);
        assert_eq!(capacity_encode_u128(2), 1);

        assert_eq!(capacity_encode_u128(31), 1);
        assert_eq!(capacity_encode_u128(32), 2);
        assert_eq!(capacity_encode_u128(33), 2);

        assert_eq!(capacity_encode_u128(63), 2);
        assert_eq!(capacity_encode_u128(64), 2);
        assert_eq!(capacity_encode_u128(65), 2);

        assert_eq!(capacity_encode_u128(32 * 32 - 1), 2);
        assert_eq!(capacity_encode_u128(32 * 32), 3);
        assert_eq!(capacity_encode_u128(32 * 32 + 1), 3);

        assert_eq!(capacity_encode_u128(64 * 64 - 1), 3);
        assert_eq!(capacity_encode_u128(64 * 64), 3);
        assert_eq!(capacity_encode_u128(64 * 64 + 1), 3);

        // 32^6 == 64^5
        assert_eq!(capacity_encode_u128(32 * 32 * 32 * 32 * 32 * 32 - 1), 6);
        assert_eq!(capacity_encode_u128(32 * 32 * 32 * 32 * 32 * 32), 7);
        assert_eq!(capacity_encode_u128(32 * 32 * 32 * 32 * 32 * 32 + 1), 7);

        assert_eq!(capacity_encode_u128(u128::MAX), 26);
    }
}

mod base64 {
    use fast32::base64::{
        capacity_decode, capacity_encode, capacity_encode_u128, capacity_encode_u64,
    };

    #[test]
    fn test_capacity_encode() {
        assert_eq!(capacity_encode(&vec![0u8; 0]), 0);

        assert_eq!(capacity_encode(&vec![0u8; 1]), 2);
        assert_eq!(capacity_encode(&vec![0u8; 2]), 3);
        assert_eq!(capacity_encode(&vec![0u8; 3]), 4);

        assert_eq!(capacity_encode(&vec![0u8; 4]), 6);
        assert_eq!(capacity_encode(&vec![0u8; 5]), 7);
        assert_eq!(capacity_encode(&vec![0u8; 6]), 8);

        assert_eq!(capacity_encode(&vec![0u8; 7]), 10);
        assert_eq!(capacity_encode(&vec![0u8; 8]), 11);
        assert_eq!(capacity_encode(&vec![0u8; 9]), 12);
    }

    #[test]
    fn test_capacity_decode() {
        assert_eq!(capacity_decode(&vec![0u8; 0]), 0);

        // Note that a 1 byte array would not be valid encoded base64, and similarly for other lengths

        assert_eq!(capacity_decode(&vec![0u8; 2]), 1);
        assert_eq!(capacity_decode(&vec![0u8; 3]), 2);
        assert_eq!(capacity_decode(&vec![0u8; 4]), 3);

        assert_eq!(capacity_decode(&vec![0u8; 6]), 4);
        assert_eq!(capacity_decode(&vec![0u8; 7]), 5);
        assert_eq!(capacity_decode(&vec![0u8; 8]), 6);

        assert_eq!(capacity_decode(&vec![0u8; 10]), 7);
        assert_eq!(capacity_decode(&vec![0u8; 11]), 8);
        assert_eq!(capacity_decode(&vec![0u8; 12]), 9);
    }

    #[test]
    fn test_capacity_encode_u64() {
        assert_eq!(capacity_encode_u64(0), 1);
        assert_eq!(capacity_encode_u64(1), 1);
        assert_eq!(capacity_encode_u64(2), 1);

        assert_eq!(capacity_encode_u64(31), 1);
        assert_eq!(capacity_encode_u64(32), 1);
        assert_eq!(capacity_encode_u64(33), 1);

        assert_eq!(capacity_encode_u64(63), 1);
        assert_eq!(capacity_encode_u64(64), 2);
        assert_eq!(capacity_encode_u64(65), 2);

        assert_eq!(capacity_encode_u64(32 * 32 - 1), 2);
        assert_eq!(capacity_encode_u64(32 * 32), 2);
        assert_eq!(capacity_encode_u64(32 * 32 + 1), 2);

        assert_eq!(capacity_encode_u64(64 * 64 - 1), 2);
        assert_eq!(capacity_encode_u64(64 * 64), 3);
        assert_eq!(capacity_encode_u64(64 * 64 + 1), 3);

        // 32^6 == 64^5
        assert_eq!(capacity_encode_u64(64 * 64 * 64 * 64 * 64 - 1), 5);
        assert_eq!(capacity_encode_u64(64 * 64 * 64 * 64 * 64), 6);
        assert_eq!(capacity_encode_u64(64 * 64 * 64 * 64 * 64 + 1), 6);

        assert_eq!(capacity_encode_u64(u64::MAX), 11);
    }

    #[test]
    fn test_capacity_encode_u128() {
        assert_eq!(capacity_encode_u128(0), 1);
        assert_eq!(capacity_encode_u128(1), 1);
        assert_eq!(capacity_encode_u128(2), 1);

        assert_eq!(capacity_encode_u128(31), 1);
        assert_eq!(capacity_encode_u128(32), 1);
        assert_eq!(capacity_encode_u128(33), 1);

        assert_eq!(capacity_encode_u128(63), 1);
        assert_eq!(capacity_encode_u128(64), 2);
        assert_eq!(capacity_encode_u128(65), 2);

        assert_eq!(capacity_encode_u128(32 * 32 - 1), 2);
        assert_eq!(capacity_encode_u128(32 * 32), 2);
        assert_eq!(capacity_encode_u128(32 * 32 + 1), 2);

        assert_eq!(capacity_encode_u128(64 * 64 - 1), 2);
        assert_eq!(capacity_encode_u128(64 * 64), 3);
        assert_eq!(capacity_encode_u128(64 * 64 + 1), 3);

        // 32^6 == 64^5
        assert_eq!(capacity_encode_u128(64 * 64 * 64 * 64 * 64 - 1), 5);
        assert_eq!(capacity_encode_u128(64 * 64 * 64 * 64 * 64), 6);
        assert_eq!(capacity_encode_u128(64 * 64 * 64 * 64 * 64 + 1), 6);

        assert_eq!(capacity_encode_u128(u128::MAX), 22);
    }
}
