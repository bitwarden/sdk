use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn allocate_string(s: &str) -> String {
    s.to_owned()
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("string abc", |b| {
        b.iter(|| allocate_string(black_box("abc")))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
