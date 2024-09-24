use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

pub fn criterion_benchmark(c: &mut Criterion) {
    let user_key = SymmetricCryptoKey::generate(rand::thread_rng());

    let org_id = uuid::Uuid::parse_str("91b000b6-81ce-47f4-9802-3390e0b895ed").expect("");
    let org_key = SymmetricCryptoKey::generate(rand::thread_rng());

    let cipher_key = SymmetricCryptoKey::generate(rand::thread_rng());
    let cipher_key_user_enc = cipher_key.to_vec().encrypt_with_key(&user_key).expect("");
    let cipher_view = CipherView {
        key: Some(cipher_key_user_enc.clone()),
        name: "test".to_string(),
    };

    let service: CryptoService<MySymmKeyRef, MyAsymmKeyRef> = CryptoService::new();
    #[allow(deprecated)]
    {
        service.insert_symmetric_key(MySymmKeyRef::User, user_key.clone());
        service.insert_symmetric_key(MySymmKeyRef::Organization(org_id), org_key.clone());
    }

    let cipher_views = vec![cipher_view.clone(); 10_000];

    {
        let mut group = c.benchmark_group("New encryptable");

        for size in [1, 10, 100, 1_000, 10_000].iter() {
            group.throughput(Throughput::Elements(*size as u64));

            group.bench_with_input(
                BenchmarkId::new("encrypt X ciphers individually", size),
                size,
                |b, &size| {
                    b.iter(|| {
                        for c in cipher_views.iter().take(size).cloned() {
                            service.encrypt(black_box(c)).expect("");
                        }
                    });
                },
            );

            group.bench_with_input(
                BenchmarkId::new("encrypt X ciphers batch", size),
                size,
                |b, &size| {
                    b.iter(|| service.encrypt_list(black_box(&cipher_views[..size])));
                },
            );
        }

        group.finish();
    }

    {
        let mut group = c.benchmark_group("Old encryptable");

        for size in [1, 10, 100, 1_000, 10_000].iter() {
            group.throughput(Throughput::Elements(*size as u64));

            group.bench_with_input(
                BenchmarkId::new("encrypt X ciphers individually", size),
                size,
                |b, &size| {
                    b.iter(|| {
                        for c in cipher_views.iter().take(size).cloned() {
                            black_box(c).encrypt_with_key(&user_key).expect("");
                        }
                    });
                },
            );

            group.bench_with_input(
                BenchmarkId::new("encrypt X ciphers batch", size),
                size,
                |b, &size| {
                    b.iter(|| {
                        black_box(cipher_views[0..size].to_vec())
                            .encrypt_with_key(&user_key)
                            .expect("");
                    });
                },
            );
        }

        group.finish();
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

use bitwarden_crypto::{
    key_refs, service::*, CryptoError, EncString, KeyDecryptable, KeyEncryptable,
    SymmetricCryptoKey,
};

key_refs! {
    #[symmetric]
    pub enum MySymmKeyRef {
        User,
        Organization(uuid::Uuid),
        #[local]
        Local(&'static str),
    }

    #[asymmetric]
    pub enum MyAsymmKeyRef {
        UserPrivateKey,
        #[local]
        Local(&'static str),
    }
}

#[allow(unused)]
struct Cipher {
    key: Option<EncString>,
    name: EncString,
}

#[derive(Clone)]
struct CipherView {
    key: Option<EncString>,
    name: String,
}

impl UsesKey<MySymmKeyRef> for Cipher {
    fn uses_key(&self) -> MySymmKeyRef {
        MySymmKeyRef::User
    }
}
impl UsesKey<MySymmKeyRef> for CipherView {
    fn uses_key(&self) -> MySymmKeyRef {
        MySymmKeyRef::User
    }
}

const CIPHER_KEY: MySymmKeyRef = MySymmKeyRef::Local("cipher_key");

// New encryptable implementations

impl Encryptable<MySymmKeyRef, MyAsymmKeyRef, MySymmKeyRef, Cipher> for CipherView {
    fn encrypt(
        &self,
        ctx: &mut CryptoServiceContext<MySymmKeyRef, MyAsymmKeyRef>,
        key: MySymmKeyRef,
    ) -> Result<Cipher, bitwarden_crypto::CryptoError> {
        let cipher_key = match &self.key {
            Some(cipher_key) => ctx.decrypt_and_store_symmetric_key(key, CIPHER_KEY, cipher_key)?,
            None => key,
        };

        Ok(Cipher {
            key: self.key.clone(),
            name: self.name.as_str().encrypt(ctx, cipher_key)?,
        })
    }
}

// Old encryptable implementations

impl KeyEncryptable<SymmetricCryptoKey, Cipher> for CipherView {
    fn encrypt_with_key(self, key: &SymmetricCryptoKey) -> Result<Cipher, CryptoError> {
        let cipher_key = self
            .key
            .as_ref()
            .map(|k| {
                let mut kk: Vec<u8> = k.decrypt_with_key(key)?;
                SymmetricCryptoKey::try_from(kk.as_mut_slice())
            })
            .transpose()?;

        let key = cipher_key.as_ref().unwrap_or(key);

        Ok(Cipher {
            key: self.key.clone(),
            name: self.name.encrypt_with_key(key)?,
        })
    }
}
