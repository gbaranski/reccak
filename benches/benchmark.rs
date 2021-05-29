use criterion::{criterion_group, criterion_main, Criterion};
use smallvec::smallvec;

fn criterion_benchmark(c: &mut Criterion) {
    let message: reccak::Input = smallvec![1, 2, 3, 4, 5];
    c.bench_function("hash data", |b| {
        b.iter(|| reccak::hash(message.clone().into()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main! {benches}
