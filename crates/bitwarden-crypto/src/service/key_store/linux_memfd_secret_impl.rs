use std::{mem::MaybeUninit, ptr::NonNull, sync::OnceLock};

use super::{
    util::{KeyData, SliceKeyStore},
    KeyRef,
};

// This is an in-memory key store that is protected by memfd_secret on Linux 5.14+.
// This should be secure against memory dumps from anything except a malicious kernel driver.
// Note that not all 5.14+ systems have support for memfd_secret enabled, so
// LinuxMemfdSecretKeyStore::new returns an Option.
pub(crate) type LinuxMemfdSecretKeyStore<Key> = SliceKeyStore<Key, MemfdSecretImplKeyData>;

pub(crate) struct MemfdSecretImplKeyData {
    ptr: std::ptr::NonNull<[u8]>,
    capacity: usize,
}

// For Send+Sync to be safe, we need to ensure that the memory is only accessed mutably from one
// thread. To do this, we have to make sure that any funcion in `MemfdSecretImplKeyData` that
// accesses the pointer mutably is defined as &mut self, and that the pointer is never copied or
// moved outside the struct.
unsafe impl Send for MemfdSecretImplKeyData {}
unsafe impl Sync for MemfdSecretImplKeyData {}

impl Drop for MemfdSecretImplKeyData {
    fn drop(&mut self) {
        unsafe {
            memsec::free_memfd_secret(self.ptr);
        }
    }
}

impl<Key: KeyRef> KeyData<Key> for MemfdSecretImplKeyData {
    fn is_available() -> bool {
        static IS_SUPPORTED: OnceLock<bool> = OnceLock::new();

        *IS_SUPPORTED.get_or_init(|| unsafe {
            let Some(ptr) = memsec::memfd_secret_sized(1) else {
                return false;
            };
            memsec::free_memfd_secret(ptr);
            true
        })
    }

    fn with_capacity(capacity: usize) -> Self {
        let entry_size = std::mem::size_of::<Option<(Key, Key::KeyValue)>>();

        unsafe {
            let ptr: NonNull<[u8]> = memsec::memfd_secret_sized(capacity * entry_size)
                .expect("memfd_secret_sized failed");

            // Initialize the array with Nones using MaybeUninit
            let uninit_slice: &mut [MaybeUninit<_>] = std::slice::from_raw_parts_mut(
                ptr.as_ptr() as *mut MaybeUninit<Option<(Key, Key::KeyValue)>>,
                capacity,
            );
            for elem in uninit_slice {
                elem.write(None);
            }

            MemfdSecretImplKeyData { ptr, capacity }
        }
    }

    fn get_key_data(&self) -> &[Option<(Key, Key::KeyValue)>] {
        let ptr = self.ptr.as_ptr() as *const Option<(Key, Key::KeyValue)>;
        // SAFETY: The pointer is valid and points to a valid slice of the correct size.
        // This function is &self so it only takes a immutable *const pointer.
        unsafe { std::slice::from_raw_parts(ptr, self.capacity) }
    }

    fn get_key_data_mut(&mut self) -> &mut [Option<(Key, Key::KeyValue)>] {
        let ptr = self.ptr.as_ptr() as *mut Option<(Key, Key::KeyValue)>;
        // SAFETY: The pointer is valid and points to a valid slice of the correct size.
        // This function is &mut self so it can take a mutable *mut pointer.
        unsafe { std::slice::from_raw_parts_mut(ptr, self.capacity) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::service::key_store::{util::tests::*, KeyStore as _};

    #[test]
    fn test_resize() {
        let mut store = LinuxMemfdSecretKeyStore::<TestKey>::with_capacity(1).unwrap();

        for (idx, key) in [
            TestKey::A,
            TestKey::B(10),
            TestKey::C,
            TestKey::B(7),
            TestKey::A,
            TestKey::C,
        ]
        .into_iter()
        .enumerate()
        {
            store.insert(key, TestKeyValue::new(idx));
        }

        assert_eq!(store.get(TestKey::A), Some(&TestKeyValue::new(4)));
        assert_eq!(store.get(TestKey::B(10)), Some(&TestKeyValue::new(1)));
        assert_eq!(store.get(TestKey::C), Some(&TestKeyValue::new(5)));
        assert_eq!(store.get(TestKey::B(7)), Some(&TestKeyValue::new(3)));
        assert_eq!(store.get(TestKey::B(20)), None);
    }
}
