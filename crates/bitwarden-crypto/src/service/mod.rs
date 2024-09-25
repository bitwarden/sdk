use std::sync::{Arc, RwLock};

use crate::{EncString, SymmetricCryptoKey};

mod context;
mod encryptable;
pub mod key_ref;
mod key_store;

use context::RustCryptoServiceContext;
pub use encryptable::{Decryptable, Encryptable, KeyProvided, KeyProvidedExt, UsesKey};
use key_ref::{AsymmetricKeyRef, KeyRef, SymmetricKeyRef};
pub use key_store::create_key_store;
use key_store::KeyStore;

#[derive(Clone)]
pub struct CryptoService<SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef> {
    // We use an Arc<> to make it easier to pass this service around, as we can
    // clone it instead of passing references
    key_stores: Arc<RwLock<RustCryptoServiceKeys<SymmKeyRef, AsymmKeyRef>>>,
}

// This is just a wrapper around the keys so we only deal with one RwLock
struct RustCryptoServiceKeys<SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef> {
    symmetric_keys: Box<dyn KeyStore<SymmKeyRef>>,
    asymmetric_keys: Box<dyn KeyStore<AsymmKeyRef>>,
}

impl<SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef>
    CryptoService<SymmKeyRef, AsymmKeyRef>
{
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            key_stores: Arc::new(RwLock::new(RustCryptoServiceKeys {
                symmetric_keys: create_key_store(),
                asymmetric_keys: create_key_store(),
            })),
        }
    }

    pub fn clear(&self) {
        let mut keys = self.key_stores.write().expect("RwLock is poisoned");
        keys.symmetric_keys.clear();
        keys.asymmetric_keys.clear();
    }

    #[deprecated(note = "We should be generating/decrypting the keys into the service directly")]
    pub fn insert_symmetric_key(&self, key_ref: SymmKeyRef, key: SymmetricCryptoKey) {
        self.key_stores
            .write()
            .expect("RwLock is poisoned")
            .symmetric_keys
            .insert(key_ref, key);
    }

    // TODO: Do we want this to be public?
    pub(crate) fn context(&'_ self) -> CryptoServiceContext<'_, SymmKeyRef, AsymmKeyRef> {
        CryptoServiceContext {
            // TODO: Cache these?, or maybe initialize them lazily? or both?
            engine: RustCryptoServiceContext {
                global_keys: self.key_stores.read().expect("RwLock is poisoned"),
                local_symmetric_keys: create_key_store(),
                local_asymmetric_keys: create_key_store(),
            },
        }
    }

    // These are just convenience methods to avoid having to call `context` every time
    pub fn decrypt<
        Key: KeyRef,
        Data: Decryptable<SymmKeyRef, AsymmKeyRef, Key, Output> + UsesKey<Key>,
        Output,
    >(
        &self,
        data: &Data,
    ) -> Result<Output, crate::CryptoError> {
        let key = data.uses_key();
        data.decrypt(&mut self.context(), key)
    }

    pub fn encrypt<
        Key: KeyRef,
        Data: Encryptable<SymmKeyRef, AsymmKeyRef, Key, Output> + UsesKey<Key>,
        Output,
    >(
        &self,
        data: Data,
    ) -> Result<Output, crate::CryptoError> {
        let key = data.uses_key();
        data.encrypt(&mut self.context(), key)
    }

    pub fn encrypt_list<
        Key: KeyRef,
        Data: Encryptable<SymmKeyRef, AsymmKeyRef, Key, Output> + UsesKey<Key> + Send + Sync,
        Output: Send + Sync,
    >(
        &self,
        data: &[Data],
    ) -> Result<Vec<Output>, crate::CryptoError> {
        use rayon::prelude::*;

        // We want to split all the data between available threads, but at the
        // same time we don't want to split it too much if the amount of data is small.

        // In this case, the minimum chunk size is 50.
        let chunk_size = usize::max(1 + data.len() / rayon::current_num_threads(), 50);

        let res: Result<Vec<_>, _> = data
            .par_chunks(chunk_size)
            .map(|chunk| {
                let mut context = self.context();

                let mut result = Vec::with_capacity(chunk.len());

                for item in chunk {
                    let key = item.uses_key();
                    result.push(item.encrypt(&mut context, key));
                    context.clear();
                }

                result
            })
            .flatten()
            .collect();

        res
    }
}

pub struct CryptoServiceContext<'a, SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef> {
    engine: RustCryptoServiceContext<'a, SymmKeyRef, AsymmKeyRef>,
}

impl<'a, SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef>
    CryptoServiceContext<'a, SymmKeyRef, AsymmKeyRef>
{
    /// Decrypt a key and store it in the local key store
    pub fn decrypt_and_store_symmetric_key(
        &mut self,
        encryption_key: SymmKeyRef,
        new_key_ref: SymmKeyRef,
        encrypted_key: &EncString,
    ) -> Result<SymmKeyRef, crate::CryptoError> {
        self.engine
            .decrypt_and_store_symmetric_key(encryption_key, new_key_ref, encrypted_key)?;
        // This is returned for convenience
        Ok(new_key_ref)
    }

    pub fn clear(&mut self) {
        self.engine.clear();
    }
}
