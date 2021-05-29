use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("generate permutations", |b| {
        b.iter(|| {
            reccak::permutations(
                b"qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890!@#%^-_=+([{<)]}>",
                5,
            )
            .take(1000000)
            .collect::<Vec<_>>()
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main! {benches}
