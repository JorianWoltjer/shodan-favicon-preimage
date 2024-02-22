use b64::ToBase64;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use shodan_favicon_preimage::{murmur3::murmur3_32, BASE64_CONFIG};

fn criterion_benchmark(c: &mut Criterion) {
    let n: u32 = 0xdeadbeef;
    let state = 31337;
    let processed = 42;

    c.bench_function("murmur3_32() with Base64 from number", |b| {
        b.iter(|| {
            let guess = black_box(n).to_le_bytes().to_base64(BASE64_CONFIG) + "\n";

            let hash = murmur3_32(guess.as_bytes(), state, processed);

            hash == 1337
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
