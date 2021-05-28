use criterion::{black_box, criterion_group, criterion_main, Criterion};


fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("hash data", |b| b.iter(|| reccak::hash(b"message")));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!{benches}
