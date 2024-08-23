use std::{ptr::NonNull, sync::OnceLock};

use zeroize::ZeroizeOnDrop;

use super::{
    util::{KeyData, SliceKeyContainer},
    KeyRef, KeyStore,
};

fn is_memfd_supported() -> bool {
    static IS_SUPPORTED: OnceLock<bool> = OnceLock::new();

    *IS_SUPPORTED.get_or_init(|| unsafe {
        let Some(ptr) = memsec::memfd_secret_sized(1) else {
            return false;
        };
        memsec::free_memfd_secret(ptr);
        true
    })
}

struct MemPtr {
    ptr: std::ptr::NonNull<[u8]>,
    capacity: usize,
}

// TODO: Is this safe?
unsafe impl Send for MemPtr {}
unsafe impl Sync for MemPtr {}

impl Drop for MemPtr {
    fn drop(&mut self) {
        unsafe {
            memsec::free_memfd_secret(self.ptr);
        }
    }
}

impl<Key: KeyRef> KeyData<Key> for MemPtr {
    fn new_with_capacity(capacity: usize) -> Self {
        let entry_size = std::mem::size_of::<Option<(Key, Key::KeyValue)>>();

        unsafe {
            let ptr: NonNull<[u8]> =
                memsec::memfd_secret_sized(capacity * entry_size).expect("Supported operation");
            MemPtr { ptr, capacity }
        }
    }

    fn get_key_data(&self) -> &[Option<(Key, Key::KeyValue)>] {
        let ptr = self.ptr.as_ptr() as *const Option<(Key, Key::KeyValue)>;
        // SAFETY: The pointer is valid and points to a valid slice of the correct size.
        unsafe { std::slice::from_raw_parts(ptr, self.capacity) }
    }

    fn get_key_data_mut(&mut self) -> &mut [Option<(Key, Key::KeyValue)>] {
        let ptr = self.ptr.as_ptr() as *mut Option<(Key, Key::KeyValue)>;
        // SAFETY: The pointer is valid and points to a valid slice of the correct size.
        unsafe { std::slice::from_raw_parts_mut(ptr, self.capacity) }
    }
}

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
        Self::with_capacity(0)
    }

    pub(crate) fn with_capacity(capacity: usize) -> Option<Self> {
        if !is_memfd_supported() {
            return None;
        }

        Some(Self {
            container: SliceKeyContainer::new_with_capacity(capacity),
            _key: std::marker::PhantomData,
        })
    }
}

// Zeroize is done by the Drop impl of SliceKeyContainer
impl<Key: KeyRef> ZeroizeOnDrop for LinuxMemfdSecretKeyStore<Key> {}

impl<Key: KeyRef> KeyStore<Key> for LinuxMemfdSecretKeyStore<Key> {
    fn insert(&mut self, key_ref: Key, key: Key::KeyValue) {
        self.container.insert(key_ref, key);
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
