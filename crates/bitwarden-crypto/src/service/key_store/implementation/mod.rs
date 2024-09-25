use super::{slice, KeyStore};
use crate::service::KeyRef;

#[cfg(all(target_os = "linux", not(feature = "no-memory-hardening")))]
pub(crate) mod linux_memfd_secret;
pub(crate) mod rust_slice;

pub(crate) fn create_key_store<Key: KeyRef>() -> Box<dyn KeyStore<Key>> {
    #[cfg(all(target_os = "linux", not(feature = "no-memory-hardening")))]
    if let Some(key_store) = linux_memfd_secret::LinuxMemfdSecretKeyStore::<Key>::new() {
        return Box::new(key_store);
    }

    Box::new(rust_slice::RustKeyStore::new().expect("RustKeyStore should always be available"))
}
