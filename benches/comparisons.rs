use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_crockford_encode_5111(c: &mut Criterion) {
    c.bench_function("crockford encode 5111", |b| {
        b.iter(|| crockford::encode(black_box(5111)))
    });
}

fn bench_base32_encode_5111(c: &mut Criterion) {
    c.bench_function("base32 encode 5111", |b| {
        b.iter(|| base32::encode(base32::Alphabet::Crockford, black_box(&[0x13, 0xF7])))
    });
}

fn bench_crate_encode_5111(c: &mut Criterion) {
    c.bench_function("easy 32 encode 5111", |b| {
        b.iter(|| easy32::encode_u64(black_box(5111)))
    });
}

fn bench_crockford_encode_1066193093600(c: &mut Criterion) {
    c.bench_function("crockford encode 1066193093600", |b| {
        b.iter(|| crockford::encode(black_box(1066193093600)))
    });
}

fn bench_base32_encode_1066193093600(c: &mut Criterion) {
    c.bench_function("base32 encode 1066193093600", |b| {
        b.iter(|| {
            base32::encode(
                base32::Alphabet::Crockford,
                black_box(&[0xF8, 0x3E, 0x0F, 0x83, 0xE0]),
            )
        })
    });
}

fn bench_crate_encode_1066193093600(c: &mut Criterion) {
    c.bench_function("easy 32 encode 1066193093600", |b| {
        b.iter(|| easy32::encode_u64(black_box(1066193093600)))
    });
}

fn bench_crockford_encode_10239(c: &mut Criterion) {
    c.bench_function("crockford encode 10239", |b| {
        b.iter(|| crockford::encode(black_box(10239)))
    });
}

fn bench_base32_encode_10239(c: &mut Criterion) {
    c.bench_function("base32 encode 10239", |b| {
        b.iter(|| base32::encode(base32::Alphabet::Crockford, black_box(&[0x27, 0xFF])))
    });
}

fn bench_crate_encode_10239(c: &mut Criterion) {
    c.bench_function("easy 32 encode 10239", |b| {
        b.iter(|| easy32::encode_u64(black_box(10239)))
    });
}

fn bench_crockford_decode_5111(c: &mut Criterion) {
    c.bench_function("crockford decode 5111", |b| {
        b.iter(|| crockford::decode(black_box("4ZQ")))
    });
}

fn bench_base32_decode_5111(c: &mut Criterion) {
    c.bench_function("base32 decode 5111", |b| {
        b.iter(|| base32::decode(base32::Alphabet::Crockford, black_box("4ZQ")))
    });
}

fn bench_crate_decode_5111(c: &mut Criterion) {
    c.bench_function("easy 32 decode 5111", |b| {
        b.iter(|| easy32::decode_u64(black_box("4ZQ")))
    });
}

fn bench_crockford_decode_1066193093600(c: &mut Criterion) {
    c.bench_function("crockford decode 1066193093600", |b| {
        b.iter(|| crockford::decode(black_box("Z0Z0Z0Z0")))
    });
}

fn bench_base32_decode_1066193093600(c: &mut Criterion) {
    c.bench_function("base32 decode 1066193093600", |b| {
        b.iter(|| base32::decode(base32::Alphabet::Crockford, black_box("Z0Z0Z0Z0")))
    });
}

fn bench_crate_decode_1066193093600(c: &mut Criterion) {
    c.bench_function("easy 32 decode 1066193093600", |b| {
        b.iter(|| easy32::decode_u64(black_box("Z0Z0Z0Z0")))
    });
}

fn bench_crockford_decode_10239(c: &mut Criterion) {
    c.bench_function("crockford decode 10239", |b| {
        b.iter(|| crockford::decode(black_box("9ZZ")))
    });
}

fn bench_base32_decode_10239(c: &mut Criterion) {
    c.bench_function("base32 decode 10239", |b| {
        b.iter(|| base32::decode(base32::Alphabet::Crockford, black_box("9ZZ")))
    });
}

fn bench_crate_decode_10239(c: &mut Criterion) {
    c.bench_function("easy 32 decode 10239", |b| {
        b.iter(|| easy32::decode_u64(black_box("9ZZ")))
    });
}

criterion_group!(
    benches,
    bench_crockford_encode_5111,
    bench_base32_encode_5111,
    bench_crate_encode_5111,
    bench_crockford_encode_1066193093600,
    bench_base32_encode_1066193093600,
    bench_crate_encode_1066193093600,
    bench_crockford_encode_10239,
    bench_base32_encode_10239,
    bench_crate_encode_10239,
    bench_crockford_decode_5111,
    bench_base32_decode_5111,
    bench_crate_decode_5111,
    bench_crockford_decode_1066193093600,
    bench_base32_decode_1066193093600,
    bench_crate_decode_1066193093600,
    bench_crockford_decode_10239,
    bench_base32_decode_10239,
    bench_crate_decode_10239,
);
criterion_main!(benches);
