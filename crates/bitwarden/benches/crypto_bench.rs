use bitwarden::platform::performance_test::{DecryptPerformanceRequest, decrypt_performance, EncryptPerformanceRequest, encrypt_performance};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let key = "UY4B5N4DA4UisCNClgZtRr6VLy9ZF5BXXC7cDZRqourKi4ghEMgISbCsubvgCkHf5DZctQjVot11/vVvN9NNHQ==".to_owned();
    let decrypt_request = DecryptPerformanceRequest {
        cipher_text: "2.wdCOHqz8UoAezZBcBaXilQ==|/HNKsVacSuL0uh2FoSIl2w==|zL4gnsP+zU3rG0bF9SQ5uphhy5HDTH26GNGzMyYVK1o=".to_owned(), // "test" encrypted by default key
        key: key.clone(),
        num_operations: 1,
    };
    let encrypt_request = EncryptPerformanceRequest {
        key: key.clone(),
        num_operations: 1,
    };

    c.bench_function("decrypt 1", |b| b.iter(|| decrypt_performance(black_box(&decrypt_request))));
    c.bench_function("encrypt 1", |b| b.iter(|| encrypt_performance(black_box(&encrypt_request))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
