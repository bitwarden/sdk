use std::{mem::MaybeUninit, ptr::NonNull};

use zeroize::{Zeroize, ZeroizeOnDrop};

use super::{
    util::{MemPtr, SliceKeyContainer},
    KeyRef, KeyStore,
};

// This is an in-memory key store that is protected by memfd_secret on Linux 5.14+.
// This should be secure against memory dumps from anything except a malicious kernel driver.
// Note that not all 5.14+ systems have support for memfd_secret enabled, so
// LinuxMemfdSecretKeyStore::new returns an Option.
pub(crate) struct LinuxMemfdSecretKeyStore<Key: KeyRef> {
    container: SliceKeyContainer<Key, MemPtr>,

    _key: std::marker::PhantomData<Key>,
}

impl<Key: KeyRef> LinuxMemfdSecretKeyStore<Key> {
    pub(crate) fn new() -> Option<Self> {
        // This might not be exactly correct in all platforms, but it's a good enough approximation
        const PAGE_SIZE: usize = 4096;
        let entry_size = std::mem::size_of::<Option<(Key, Key::KeyValue)>>();
        let elements_per_page = PAGE_SIZE / entry_size;

        // We're using mlock APIs to protect the memory, so allocating less than a page is a waste
        let capacity = std::cmp::max(32, elements_per_page);

        Self::with_capacity(capacity)
    }

    pub(crate) fn with_capacity(capacity: usize) -> Option<Self> {
        let entry_size = std::mem::size_of::<Option<(Key, Key::KeyValue)>>();

        let memory = unsafe {
            let ptr: NonNull<[u8]> = memsec::memfd_secret_sized(capacity * entry_size)?;
            MemPtr::new(ptr, capacity)
        };

        let container = SliceKeyContainer::new(memory);

        // Validate that the entry size is correct
        debug_assert_eq!(container.entry_size(), entry_size);

        Some(Self {
            container,
            _key: std::marker::PhantomData,
        })
    }
}

impl<Key: KeyRef> ZeroizeOnDrop for LinuxMemfdSecretKeyStore<Key> {}

impl<Key: KeyRef> Drop for LinuxMemfdSecretKeyStore<Key> {
    fn drop(&mut self) {
        // Freeing the memory should clear all the secrets, but to ensure all the Drop impls
        // are called correctly we clear the container first
        self.container.clear();
        unsafe {
            memsec::free_memfd_secret(self.container.inner_mut().as_ptr());
        }
    }
}

impl<Key: KeyRef> KeyStore<Key> for LinuxMemfdSecretKeyStore<Key> {
    fn insert(&mut self, key_ref: Key, key: Key::KeyValue) {
        if let Err(new_capacity) = self.container.ensure_capacity(1) {
            // Create a new store with the correct capacity and replace self with it
            let mut new_self =
                Self::with_capacity(new_capacity).expect("Failed to allocate new memfd_secret");
            new_self.container.copy_from(&mut self.container);
            *self = new_self;
        };

        let ok = self.container.insert(key_ref, key);
        debug_assert!(ok, "insert failed");
    }

    fn get(&self, key_ref: Key) -> Option<&Key::KeyValue> {
        self.container.get(key_ref)
    }
    fn remove(&mut self, key_ref: Key) {
        self.container.remove(key_ref)
    }
    fn clear(&mut self) {
        self.container.clear()
    }
    fn retain(&mut self, f: fn(Key) -> bool) {
        self.container.retain(f);
    }
}

#[cfg(test)]
mod tests {
    use super::{super::util::tests::*, *};

    #[test]
    fn test_resize() {
        let mut store = super::LinuxMemfdSecretKeyStore::<TestKey>::with_capacity(1).unwrap();

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
