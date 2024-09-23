use zeroize::ZeroizeOnDrop;

use crate::service::KeyRef;

#[cfg(target_os = "linux")]
mod linux_memfd_secret_impl;
mod rust_impl;
mod util;

#[cfg(target_os = "linux")]
pub(crate) use linux_memfd_secret_impl::LinuxMemfdSecretKeyStore;
pub(crate) use rust_impl::RustKeyStore;

pub(crate) fn create_key_store<Key: KeyRef>() -> Box<dyn KeyStore<Key>> {
    #[cfg(target_os = "linux")]
    if let Some(key_store) = LinuxMemfdSecretKeyStore::<Key>::new() {
        return Box::new(key_store);
    }

    Box::new(RustKeyStore::new())
}

/// This trait represents a platform that can securely store and return keys. The `RustKeyStore`
/// implementation is a simple in-memory store without any security guarantees. Other
/// implementations could use secure enclaves or HSMs, or OS provided keychains.
#[allow(dead_code)]
pub(crate) trait KeyStore<Key: KeyRef>: ZeroizeOnDrop + Send + Sync {
    fn insert(&mut self, key_ref: Key, key: Key::KeyValue);
    fn get(&self, key_ref: Key) -> Option<&Key::KeyValue>;
    fn remove(&mut self, key_ref: Key);
    fn clear(&mut self);

    fn retain(&mut self, f: fn(Key) -> bool);
}

fn _ensure_that_trait_is_object_safe<Key: KeyRef>(_: Box<dyn KeyStore<Key>>) {}
