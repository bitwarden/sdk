use zeroize::ZeroizeOnDrop;

use crate::service::KeyRef;

mod implementation;
mod slice;

pub use implementation::create_key_store;

/// This trait represents a platform that can securely store and return keys. The `SliceKeyStore`
/// implementation is a simple in-memory store with some platform-specific security features. Other
/// implementations could use secure enclaves or HSMs, or OS provided keychains.
pub trait KeyStore<Key: KeyRef>: ZeroizeOnDrop + Send + Sync {
    fn insert(&mut self, key_ref: Key, key: Key::KeyValue);
    fn get(&self, key_ref: Key) -> Option<&Key::KeyValue>;
    fn remove(&mut self, key_ref: Key);
    fn clear(&mut self);

    fn retain(&mut self, f: fn(Key) -> bool);
}
