use std::sync::Mutex;

use bitwarden::platform::performance_test::{DecryptPerformanceRequest, EncryptPerformanceRequest};
use bitwarden_json::{client::Client, command::{Command, PerformanceCommand}};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use criterion::async_executor::FuturesExecutor;

pub fn criterion_benchmark(c: &mut Criterion) {
    let key = "UY4B5N4DA4UisCNClgZtRr6VLy9ZF5BXXC7cDZRqourKi4ghEMgISbCsubvgCkHf5DZctQjVot11/vVvN9NNHQ==".to_owned();
    let decrypt_request = DecryptPerformanceRequest {
        cipher_text: "2.wdCOHqz8UoAezZBcBaXilQ==|/HNKsVacSuL0uh2FoSIl2w==|zL4gnsP+zU3rG0bF9SQ5uphhy5HDTH26GNGzMyYVK1o=".to_owned(), // "test" encrypted by default key
        key: key.clone(),
        num_operations: 1000,
    };
    let encrypt_request = EncryptPerformanceRequest {
        key: key.clone(),
        num_operations: 1000,
    };

    let decrypt_command = Command::Performance(PerformanceCommand::Decrypt(decrypt_request));
    let encrypt_command = Command::Performance(PerformanceCommand::Encrypt(encrypt_request));
    
    let decrypt_json = serde_json::to_string(&decrypt_command).unwrap();
    let encrypt_json = serde_json::to_string(&encrypt_command).unwrap();

    let client = Mutex::new(Client::new(None));
    
    c.bench_function("json decrypt 1k ops", |b| b.to_async(FuturesExecutor).iter(|| async {
        client.lock().unwrap().run_command(black_box(&decrypt_json)).await
    }));
    c.bench_function("json encrypt 1k ops", |b| b.to_async(FuturesExecutor).iter(|| async {
        client.lock().unwrap().run_command(black_box(&encrypt_json)).await
    }));
}

criterion_group!(json_benches, criterion_benchmark);
criterion_main!(json_benches);
