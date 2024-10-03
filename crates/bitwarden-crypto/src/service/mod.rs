use std::sync::{Arc, RwLock};

use crate::{AsymmetricCryptoKey, SymmetricCryptoKey};

mod context;
mod encryptable;
pub mod key_ref;
mod key_store;

pub use context::CryptoServiceContext;
pub use encryptable::{Decryptable, Encryptable, UsesKey, UsingKey, UsingKeyExt};
use key_ref::{AsymmetricKeyRef, KeyRef, SymmetricKeyRef};
pub use key_store::create_key_store;
use key_store::KeyStore;

#[derive(Clone)]
pub struct CryptoService<SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef> {
    // We use an Arc<> to make it easier to pass this service around, as we can
    // clone it instead of passing references
    key_stores: Arc<RwLock<Keys<SymmKeyRef, AsymmKeyRef>>>,
}

impl<SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef> std::fmt::Debug
    for CryptoService<SymmKeyRef, AsymmKeyRef>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CryptoService").finish()
    }
}

// This is just a wrapper around the keys so we only deal with one RwLock
pub(crate) struct Keys<SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef>
{
    symmetric_keys: Box<dyn KeyStore<SymmKeyRef>>,
    asymmetric_keys: Box<dyn KeyStore<AsymmKeyRef>>,
}

impl<SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef>
    CryptoService<SymmKeyRef, AsymmKeyRef>
{
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            key_stores: Arc::new(RwLock::new(Keys {
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

    pub fn retain_symmetric_keys(&self, f: fn(SymmKeyRef) -> bool) {
        self.key_stores
            .write()
            .expect("RwLock is poisoned")
            .symmetric_keys
            .retain(f);
    }

    pub fn retain_asymmetric_keys(&self, f: fn(AsymmKeyRef) -> bool) {
        self.key_stores
            .write()
            .expect("RwLock is poisoned")
            .asymmetric_keys
            .retain(f);
    }

    #[deprecated(note = "We should be generating/decrypting the keys into the service directly")]
    pub fn insert_symmetric_key(&self, key_ref: SymmKeyRef, key: SymmetricCryptoKey) {
        self.key_stores
            .write()
            .expect("RwLock is poisoned")
            .symmetric_keys
            .insert(key_ref, key);
    }

    #[deprecated(note = "We should be generating/decrypting the keys into the service directly")]
    pub fn insert_asymmetric_key(&self, key_ref: AsymmKeyRef, key: AsymmetricCryptoKey) {
        self.key_stores
            .write()
            .expect("RwLock is poisoned")
            .asymmetric_keys
            .insert(key_ref, key);
    }

    /// Initiate an encryption/decryption context. This is an advanced API, use with care.
    /// Prefer to instead use `encrypt`/`decrypt`/`encrypt_list`/`decrypt_list` methods.
    pub fn context(&'_ self) -> CryptoServiceContext<'_, SymmKeyRef, AsymmKeyRef> {
        CryptoServiceContext {
            global_keys: self.key_stores.read().expect("RwLock is poisoned"),
            local_symmetric_keys: create_key_store(),
            local_asymmetric_keys: create_key_store(),
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

    pub fn decrypt_list<
        Key: KeyRef,
        Data: Decryptable<SymmKeyRef, AsymmKeyRef, Key, Output> + UsesKey<Key> + Send + Sync,
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
                    result.push(item.decrypt(&mut context, key));
                    context.clear();
                }

                result
            })
            .flatten()
            .collect();

        res
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
