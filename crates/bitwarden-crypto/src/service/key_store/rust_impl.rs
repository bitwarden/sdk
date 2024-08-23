use zeroize::ZeroizeOnDrop;

use super::{
    util::{KeyData, SliceKeyContainer},
    KeyRef, KeyStore,
};
const ENABLE_MLOCK: bool = true;

struct Mem<Key: KeyRef> {
    #[allow(clippy::type_complexity)]
    data: Box<[Option<(Key, Key::KeyValue)>]>,
}

impl<Key: KeyRef> Drop for Mem<Key> {
    fn drop(&mut self) {
        if ENABLE_MLOCK {
            let entry_size = std::mem::size_of::<Option<(Key, Key::KeyValue)>>();
            unsafe {
                memsec::munlock(
                    self.data.as_mut_ptr() as *mut u8,
                    self.data.len() * entry_size,
                );
            }
        }
    }
}

impl<Key: KeyRef> KeyData<Key> for Mem<Key> {
    fn new_with_capacity(capacity: usize) -> Self {
        let mut data: Box<_> = std::iter::repeat_with(|| None).take(capacity).collect();

        if ENABLE_MLOCK {
            let entry_size = std::mem::size_of::<Option<(Key, Key::KeyValue)>>();
            unsafe {
                memsec::mlock(data.as_mut_ptr() as *mut u8, capacity * entry_size);
            }
        }
        Mem { data }
    }

    fn get_key_data(&self) -> &[Option<(Key, Key::KeyValue)>] {
        self.data.as_ref()
    }

    fn get_key_data_mut(&mut self) -> &mut [Option<(Key, Key::KeyValue)>] {
        self.data.as_mut()
    }
}

// This is a basic in-memory key store for the cases where we don't have a secure key store
// available. We still make use mlock to protect the memory from being swapped to disk, and we
// zeroize the values when dropped.
pub(crate) struct RustKeyStore<Key: KeyRef> {
    #[allow(clippy::type_complexity)]
    container: SliceKeyContainer<Key, Mem<Key>>,
}

impl<Key: KeyRef> RustKeyStore<Key> {
    pub(crate) fn new() -> Self {
        Self::with_capacity(0)
    }

    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            container: SliceKeyContainer::new_with_capacity(capacity),
        }
    }
}

// Zeroize is done by the Drop impl of SliceKeyContainer
impl<Key: KeyRef> ZeroizeOnDrop for RustKeyStore<Key> {}

impl<Key: KeyRef> KeyStore<Key> for RustKeyStore<Key> {
    fn insert(&mut self, key_ref: Key, key: Key::KeyValue) {
        /*
         if let Err(new_capacity) = self.container.ensure_capacity(1) {
            // Create a new store with the correct capacity and replace self with it
            let mut new_self = Self::with_capacity(new_capacity);
            new_self.container.copy_from(&mut self.container);
            *self = new_self;
        };

        let ok = self.container.insert(key_ref, key);
        debug_assert!(ok, "insert failed");

         */

        self.container.insert(key_ref, key)
    }

    fn get(&self, key_ref: Key) -> Option<&Key::KeyValue> {
        self.container.get(key_ref)
    }

    fn remove(&mut self, key_ref: Key) {
        self.container.remove(key_ref);
    }

    fn clear(&mut self) {
        self.container.clear();
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
        let mut store = super::RustKeyStore::<TestKey>::with_capacity(1);

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
