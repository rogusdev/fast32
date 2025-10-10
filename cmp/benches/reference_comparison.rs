use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

// Current implementation
use fast32::base32::CROCKFORD as CURRENT_CROCKFORD;

// Reference implementation
use fast32_reference::base32::CROCKFORD as REFERENCE_CROCKFORD;

// ============================================================================
// Grouped Comparison Benchmarks
// ============================================================================

fn u64_encode_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("u64_encode_comparison");

    // Test different values
    let test_values = [
        ("small_5111", 5111u64),
        ("medium_10238", 10238u64),
        ("large_1066193093600", 1066193093600u64),
        ("very_large_u64_big", 1311768467294899695u64),
        ("max_u64", u64::MAX),
    ];

    for (name, value) in test_values.iter() {
        // Reference implementation (baseline)
        group.bench_with_input(BenchmarkId::new("reference", name), value, |b, &n| {
            b.iter(|| REFERENCE_CROCKFORD.encode_u64(black_box(n)))
        });

        // Current implementation (to compare against reference)
        group.bench_with_input(BenchmarkId::new("current", name), value, |b, &n| {
            b.iter(|| CURRENT_CROCKFORD.encode_u64(black_box(n)))
        });
    }

    group.finish();
}

fn u64_decode_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("u64_decode_comparison");

    // Test different encoded values (corresponding to the encode test values)
    let test_values = [
        ("small_5111", &b"4ZQ"[..]),
        ("medium_10238", &b"9ZY"[..]),
        ("large_1066193093600", &b"Z0Z0Z0Z0"[..]),
        ("very_large_u64_big", &b"14D2PF28AQKFF"[..]),
        ("max_u64", &b"FZZZZZZZZZZZZ"[..]),
    ];

    for (name, data) in test_values.iter() {
        // Reference implementation (baseline)
        group.bench_with_input(BenchmarkId::new("reference", name), data, |b, data| {
            b.iter(|| REFERENCE_CROCKFORD.decode_u64(black_box(*data)))
        });

        // Current implementation (to compare against reference)
        group.bench_with_input(BenchmarkId::new("current", name), data, |b, data| {
            b.iter(|| CURRENT_CROCKFORD.decode_u64(black_box(*data)))
        });
    }

    group.finish();
}

fn u128_encode_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("u128_encode_comparison");

    // Test different u128 values
    let test_values = [
        ("small_u64_max_as_u128", u64::MAX as u128),
        ("large_u128_big", 24197857200151252728969465429440056815u128),
        ("max_u128", u128::MAX),
    ];

    for (name, value) in test_values.iter() {
        // Reference implementation (baseline)
        group.bench_with_input(BenchmarkId::new("reference", name), value, |b, &n| {
            b.iter(|| REFERENCE_CROCKFORD.encode_u128(black_box(n)))
        });

        // Current implementation (to compare against reference)
        group.bench_with_input(BenchmarkId::new("current", name), value, |b, &n| {
            b.iter(|| CURRENT_CROCKFORD.encode_u128(black_box(n)))
        });
    }

    group.finish();
}

fn u128_decode_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("u128_decode_comparison");

    // Test different encoded u128 values (corresponding to the encode test values)
    let test_values = [
        ("small_u64_max_as_u128", &b"FZZZZZZZZZZZZ"[..]),
        ("large_u128_big", &b"J6HB7H45BSQQH4D2PF28AQKFF"[..]),
        ("max_u128", &b"7ZZZZZZZZZZZZZZZZZZZZZZZZZ"[..]),
    ];

    for (name, data) in test_values.iter() {
        // Reference implementation (baseline)
        group.bench_with_input(BenchmarkId::new("reference", name), data, |b, data| {
            b.iter(|| REFERENCE_CROCKFORD.decode_u128(black_box(*data)))
        });

        // Current implementation (to compare against reference)
        group.bench_with_input(BenchmarkId::new("current", name), data, |b, data| {
            b.iter(|| CURRENT_CROCKFORD.decode_u128(black_box(*data)))
        });
    }

    group.finish();
}

fn bytes_encode_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("bytes_encode_comparison");

    // Test different byte array sizes for encoding
    let test_data = [
        (
            "fox_41_bytes",
            &b"The quick brown fox jumps over the lazy d"[..],
        ),
        (
            "fox_42_bytes",
            &b"The quick brown fox jumps over the lazy do"[..],
        ),
        (
            "fox_43_bytes",
            &b"The quick brown fox jumps over the lazy dog"[..],
        ),
        (
            "fox_44_bytes",
            &b"The quick brown fox jumps over the lazy dog."[..],
        ),
    ];

    for (name, data) in test_data.iter() {
        // Reference implementation (baseline)
        group.bench_with_input(BenchmarkId::new("reference", name), data, |b, data| {
            b.iter(|| REFERENCE_CROCKFORD.encode(black_box(*data)))
        });

        // Current implementation (to compare against reference)
        group.bench_with_input(BenchmarkId::new("current", name), data, |b, data| {
            b.iter(|| CURRENT_CROCKFORD.encode(black_box(*data)))
        });
    }

    group.finish();
}

fn bytes_decode_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("bytes_decode_comparison");

    // Test different encoded byte array sizes for decoding
    let test_data = [
        (
            "fox_41_bytes",
            &b"AHM6A83HENMP6TS0C9S6YXVE41K6YY10D9TPTW3K41QQCSBJ41T6GS90DHGQMY90CG"[..],
        ),
        (
            "fox_42_bytes",
            &b"AHM6A83HENMP6TS0C9S6YXVE41K6YY10D9TPTW3K41QQCSBJ41T6GS90DHGQMY90CHQG"[..],
        ),
        (
            "fox_43_bytes",
            &b"AHM6A83HENMP6TS0C9S6YXVE41K6YY10D9TPTW3K41QQCSBJ41T6GS90DHGQMY90CHQPE"[..],
        ),
        (
            "fox_44_bytes",
            &b"AHM6A83HENMP6TS0C9S6YXVE41K6YY10D9TPTW3K41QQCSBJ41T6GS90DHGQMY90CHQPEBG"[..],
        ),
    ];

    for (name, data) in test_data.iter() {
        // Reference implementation (baseline)
        group.bench_with_input(BenchmarkId::new("reference", name), data, |b, data| {
            b.iter(|| REFERENCE_CROCKFORD.decode(black_box(*data)))
        });

        // Current implementation (to compare against reference)
        group.bench_with_input(BenchmarkId::new("current", name), data, |b, data| {
            b.iter(|| CURRENT_CROCKFORD.decode(black_box(*data)))
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    u64_encode_comparison,
    u64_decode_comparison,
    u128_encode_comparison,
    u128_decode_comparison,
    bytes_encode_comparison,
    bytes_decode_comparison,
);

criterion_main!(benches);
