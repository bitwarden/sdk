use std::sync::RwLock;

use super::key_store::create_key_store;
use crate::{
    service::{key_store::KeyStore, AsymmetricKeyRef, SymmetricKeyRef},
    SymmetricCryptoKey,
};

mod context;
pub(crate) use context::RustCryptoEngineContext;

pub(crate) struct RustCryptoEngine<SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef> {
    key_stores: RwLock<RustCryptoEngineKeys<SymmKeyRef, AsymmKeyRef>>,
}

// This is just a wrapper around the keys so we only deal with one RwLock
struct RustCryptoEngineKeys<SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef> {
    symmetric_keys: Box<dyn KeyStore<SymmKeyRef>>,
    asymmetric_keys: Box<dyn KeyStore<AsymmKeyRef>>,
}

impl<SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef>
    RustCryptoEngine<SymmKeyRef, AsymmKeyRef>
{
    pub(crate) fn new() -> Self {
        Self {
            key_stores: RwLock::new(RustCryptoEngineKeys {
                symmetric_keys: create_key_store(),
                asymmetric_keys: create_key_store(),
            }),
        }
    }
}

impl<SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef>
    RustCryptoEngine<SymmKeyRef, AsymmKeyRef>
{
    pub(crate) fn context(&'_ self) -> RustCryptoEngineContext<'_, SymmKeyRef, AsymmKeyRef> {
        // TODO: Cache these?, or maybe initialize them lazily? or both?
        RustCryptoEngineContext {
            global_keys: self.key_stores.read().expect("RwLock is poisoned"),
            local_symmetric_keys: create_key_store(),
            local_asymmetric_keys: create_key_store(),
        }
    }

    pub(crate) fn insert_symmetric_key(&self, key_ref: SymmKeyRef, key: SymmetricCryptoKey) {
        self.key_stores
            .write()
            .expect("RwLock is poisoned")
            .symmetric_keys
            .insert(key_ref, key);
    }

    pub(crate) fn clear(&self) {
        let mut keys = self.key_stores.write().expect("RwLock is poisoned");
        keys.symmetric_keys.clear();
        keys.asymmetric_keys.clear();
    }
}
