#[cfg(feature = "uniffi")]
uniffi::setup_scaffolding!();
#[cfg(feature = "uniffi")]
mod uniffi_support;

#[cfg(feature = "internal")]
pub mod admin_console;
pub mod auth;
pub mod client;
mod error;
pub use error::{validate_only_whitespaces, Error, MissingFieldError, VaultLocked};
#[cfg(feature = "internal")]
pub mod mobile;
#[cfg(feature = "internal")]
pub mod platform;
#[cfg(feature = "secrets")]
pub mod secrets_manager;
mod util;

pub use bitwarden_crypto::ZeroizingAllocator;
pub use client::{Client, ClientSettings, DeviceType};

#[allow(warnings)]
#[cfg(test)]
mod testcrypto {

    // TEST IMPL OF THE NEW ENCRYPTABLE/DECRYPTABLE TRAITS
    // Note that these never touch the keys at all, they just use the context and key references to
    // encrypt/decrypt

    use bitwarden_crypto::{
        key_refs, service::*, AsymmetricCryptoKey, CryptoError, EncString, KeyEncryptable,
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

    #[derive(Clone)]
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

    impl Encryptable<MySymmKeyRef, MyAsymmKeyRef, MySymmKeyRef, Cipher> for CipherView {
        fn encrypt(
            &self,
            ctx: &mut CryptoServiceContext<MySymmKeyRef, MyAsymmKeyRef>,
            key: MySymmKeyRef,
        ) -> Result<Cipher, bitwarden_crypto::CryptoError> {
            let cipher_key = match &self.key {
                Some(cipher_key) => {
                    ctx.decrypt_and_store_symmetric_key(key, CIPHER_KEY, cipher_key)?
                }
                None => key,
            };

            Ok(Cipher {
                key: self.key.clone(),
                name: self.name.as_str().encrypt(ctx, cipher_key)?,
            })
        }
    }

    impl Decryptable<MySymmKeyRef, MyAsymmKeyRef, MySymmKeyRef, CipherView> for Cipher {
        fn decrypt(
            &self,
            ctx: &mut CryptoServiceContext<MySymmKeyRef, MyAsymmKeyRef>,
            key: MySymmKeyRef,
        ) -> Result<CipherView, CryptoError> {
            let cipher_key = match &self.key {
                Some(cipher_key) => {
                    ctx.decrypt_and_store_symmetric_key(key, CIPHER_KEY, cipher_key)?
                }
                None => key,
            };

            Ok(CipherView {
                key: self.key.clone(),
                name: self.name.decrypt(ctx, cipher_key)?,
            })
        }
    }

    #[test]
    fn test_cipher() {
        let user_key = SymmetricCryptoKey::generate(rand::thread_rng());

        let org_id = uuid::Uuid::parse_str("91b000b6-81ce-47f4-9802-3390e0b895ed").unwrap();
        let org_key = SymmetricCryptoKey::generate(rand::thread_rng());

        let cipher_key = SymmetricCryptoKey::generate(rand::thread_rng());
        let cipher_key_user_enc = cipher_key.to_vec().encrypt_with_key(&user_key).unwrap();
        let cipher_view = CipherView {
            key: Some(cipher_key_user_enc.clone()),
            name: "test".to_string(),
        };

        let service: CryptoService<MySymmKeyRef, MyAsymmKeyRef> = CryptoService::new();
        // Ideally we'd decrypt the keys directly into the service, but that's not implemented yet
        #[allow(deprecated)]
        {
            service.insert_symmetric_key(MySymmKeyRef::User, user_key);
            service.insert_symmetric_key(MySymmKeyRef::Organization(org_id), org_key);
        }

        let cipher_enc2 = service.encrypt(cipher_view.clone()).unwrap();

        let cipher_view2 = service.decrypt(&cipher_enc2).unwrap();

        assert_eq!(cipher_view.name, cipher_view2.name);

        // We can also decrypt a value by tagging it with the key
        let text = String::from("test!");

        let text_enc = service
            .encrypt(text.as_str().using_key(MySymmKeyRef::User))
            .unwrap();

        // And decrypt values in parallel
        let mut data = Vec::with_capacity(10_000_000);
        for _ in 0..data.capacity() {
            data.push("hello world, this is an encryption test!".using_key(MySymmKeyRef::User));
        }
        let now = std::time::Instant::now();
        let _ = service.encrypt_list(&data).unwrap();
        println!("Batch encrypting took {:?}", now.elapsed());

        let now = std::time::Instant::now();
        for d in data {
            let _ = service.encrypt(d).unwrap();
        }
        println!("Individual encrypting took {:?}", now.elapsed());

        panic!("DONE")
    }
}
