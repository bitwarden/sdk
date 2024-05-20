use bitwarden_crypto::ZeroizingAllocator;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[global_allocator]
static ALLOC: ZeroizingAllocator<std::alloc::System> = ZeroizingAllocator(std::alloc::System);

fn allocate_string(s: &str) -> String {
    s.to_owned()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("string abc", |b| {
        b.iter(|| allocate_string(black_box("abc")))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
