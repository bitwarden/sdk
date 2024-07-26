use std::sync::Arc;

use crate::{EncString, SymmetricCryptoKey};

mod crypto_engine;
mod encryptable;
pub mod key_ref;
mod key_store;

use crypto_engine::{CryptoEngine, CryptoEngineContext, RustCryptoEngine};
pub use encryptable::{Decryptable, Encryptable};
use key_ref::{AsymmetricKeyRef, KeyRef, SymmetricKeyRef};

#[derive(Clone)]
pub struct CryptoService<SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef> {
    // We use an Arc<> to make it easier to pass this service around, as we can
    // clone it instead of passing references
    engine: Arc<dyn CryptoEngine<SymmKeyRef, AsymmKeyRef>>,
}

impl<SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef>
    CryptoService<SymmKeyRef, AsymmKeyRef>
{
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            engine: Arc::new(RustCryptoEngine::new()),
        }
    }

    pub fn clear(&self) {
        self.engine.clear();
    }

    #[deprecated(note = "We should be generating/decrypting the keys into the service directly")]
    pub fn insert_symmetric_key(&self, key_ref: SymmKeyRef, key: SymmetricCryptoKey) {
        #[allow(deprecated)]
        self.engine.insert_symmetric_key(key_ref, key);
    }

    pub fn context(&'_ self) -> CryptoServiceContext<'_, SymmKeyRef, AsymmKeyRef> {
        CryptoServiceContext {
            engine: self.engine.context(),
        }
    }

    // These are just convenience methods to avoid having to call `context` every time
    pub fn decrypt<Key: KeyRef, Data: Decryptable<SymmKeyRef, AsymmKeyRef, Key, Output>, Output>(
        &self,
        key: Key,
        data: &Data,
    ) -> Result<Output, crate::CryptoError> {
        data.decrypt(&mut self.context(), key)
    }

    pub fn encrypt<Key: KeyRef, Data: Encryptable<SymmKeyRef, AsymmKeyRef, Key, Output>, Output>(
        &self,
        key: Key,
        data: Data,
    ) -> Result<Output, crate::CryptoError> {
        data.encrypt(&mut self.context(), key)
    }
}

pub struct CryptoServiceContext<'a, SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef> {
    engine: Box<dyn CryptoEngineContext<'a, SymmKeyRef, AsymmKeyRef> + 'a>,
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
            .decrypt_and_store_symmetric_key(encryption_key, new_key_ref, encrypted_key)
    }
}
