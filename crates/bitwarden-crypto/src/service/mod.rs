use std::sync::{Arc, RwLock};

use crate::{AsymmetricKeyRef, Decryptable, Encryptable, KeyRef, SymmetricKeyRef, UsesKey};

mod context;

mod key_store;

use context::ReadWriteGlobal;
pub use context::{CryptoServiceContext, ReadOnlyGlobal};
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

pub struct Keys<SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef> {
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

    /// Initiate an encryption/decryption context. This context will have read only access to the
    /// global keys, and will have its own local key stores with read/write access. This
    /// context-local store will be cleared up when the context is dropped.
    ///
    /// This is an advanced API, use with care. Prefer to instead use
    /// `encrypt`/`decrypt`/`encrypt_list`/`decrypt_list` methods.
    ///
    /// One of the pitfalls of the current implementations is that keys stored in the context-local
    /// store only get cleared automatically when dropped, and not between operations. This
    /// means that if you are using the same context for multiple operations, you may want to
    /// clear it manually between them.
    pub fn context(&'_ self) -> CryptoServiceContext<'_, SymmKeyRef, AsymmKeyRef> {
        CryptoServiceContext {
            global: ReadOnlyGlobal(self.key_stores.read().expect("RwLock is poisoned")),
            local_symmetric_keys: create_key_store(),
            local_asymmetric_keys: create_key_store(),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Initiate an encryption/decryption context. This context will have MUTABLE access to the
    /// global keys, and will have its own local key stores with read/write access. This
    /// context-local store will be cleared up when the context is dropped.
    ///
    /// This is an advanced API, use with care and ONLY when needing to modify the global keys.
    ///
    /// The same pitfalls as `context` apply here, but with the added risk of accidentally
    /// modifying the global keys and leaving the service in an inconsistent state.
    ///
    /// TODO: We should work towards making this pub(crate)
    pub fn context_mut(
        &'_ self,
    ) -> CryptoServiceContext<
        '_,
        SymmKeyRef,
        AsymmKeyRef,
        ReadWriteGlobal<'_, SymmKeyRef, AsymmKeyRef>,
    > {
        CryptoServiceContext {
            global: ReadWriteGlobal(self.key_stores.write().expect("RwLock is poisoned")),
            local_symmetric_keys: create_key_store(),
            local_asymmetric_keys: create_key_store(),
            _phantom: std::marker::PhantomData,
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
                let mut ctx = self.context();

                let mut result = Vec::with_capacity(chunk.len());

                for item in chunk {
                    let key = item.uses_key();
                    result.push(item.decrypt(&mut ctx, key));
                    ctx.clear();
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
                let mut ctx = self.context();

                let mut result = Vec::with_capacity(chunk.len());

                for item in chunk {
                    let key = item.uses_key();
                    result.push(item.encrypt(&mut ctx, key));
                    ctx.clear();
                }

                result
            })
            .flatten()
            .collect();

        res
    }
}
