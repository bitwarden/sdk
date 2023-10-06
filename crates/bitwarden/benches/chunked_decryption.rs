use std::io::Write;

use bitwarden::crypto::{encrypt_aes256_hmac, ChunkedDecryptor, SymmetricCryptoKey};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::RngCore;

struct SizeFmt(usize);
impl std::fmt::Display for SizeFmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const SUFFIXES: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
        let mut size = self.0 as f64;
        for suffix in SUFFIXES {
            if size < 1024.0 {
                return write!(f, "{:.1}{}", size, suffix);
            }
            size /= 1024.0;
        }
        write!(f, "{}", self.0)
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("decryption");

    for size in [100 * 1024, 15 * 1024 * 1024, 200 * 1024 * 1024] {
        group.throughput(criterion::Throughput::Bytes(size as u64));
        if size > 1024 * 1024 {
            group.sample_size(20);
        }

        let mut initial_buf = Vec::with_capacity(size);
        initial_buf.resize(size, 0);
        rand::thread_rng().fill_bytes(&mut initial_buf[..size]);
        let key: SymmetricCryptoKey = SymmetricCryptoKey::generate("test");
        let enc_str = encrypt_aes256_hmac(&initial_buf, key.mac_key.unwrap(), key.key).unwrap();
        let enc_buf = enc_str.to_buffer().unwrap();

        group.bench_with_input(
            BenchmarkId::new("decrypt_with_key", SizeFmt(size)),
            &size,
            |b, _size| b.iter(|| black_box(enc_str.decrypt_with_key(&key).unwrap())),
        );

        for chunk_size in [64, 2048, 8192] {
            group.bench_with_input(
                BenchmarkId::new(format!("ChunkedDecryptor[{chunk_size}]"), SizeFmt(size)),
                &size,
                |b, _size| {
                    b.iter(|| {
                        let mut decrypted_buf = Vec::with_capacity(size);
                        let mut cd = ChunkedDecryptor::new(&key, &mut decrypted_buf);

                        for chunk in enc_buf.chunks(chunk_size) {
                            cd.write_all(chunk).unwrap();
                        }
                        cd.finalize().unwrap();

                        //
                    })
                },
            );
        }
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
