use zeroize::ZeroizeOnDrop;

use super::{util::SliceKeyContainer, KeyRef, KeyStore};

// This is a basic in-memory key store for the cases where we don't have a secure key store
// available. We still make use mlock to protect the memory from being swapped to disk, and we
// zeroize the values when dropped.
pub(crate) struct RustKeyStore<Key: KeyRef> {
    #[allow(clippy::type_complexity)]
    container: SliceKeyContainer<Key, Box<[Option<(Key, Key::KeyValue)>]>>,
}

const ENABLE_MLOCK: bool = true;

impl<Key: KeyRef> RustKeyStore<Key> {
    pub(crate) fn new() -> Self {
        // This might not be exactly correct in all platforms, but it's a good enough approximation
        const PAGE_SIZE: usize = 4096;
        let entry_size = std::mem::size_of::<Option<(Key, Key::KeyValue)>>();

        let entries_per_page = PAGE_SIZE / entry_size;

        // We're using mlock APIs to protect the memory, so allocating less than a page is a waste
        let capacity = std::cmp::max(32, entries_per_page);

        Self::with_capacity(capacity)
    }

    pub(crate) fn with_capacity(capacity: usize) -> Self {
        let entry_size = std::mem::size_of::<Option<(Key, Key::KeyValue)>>();

        // This is a bit awkward, but we need to fill the entire slice with None, and we can't just
        // use vec![None; capacity] because that requires adding a Clone bound to the key
        // value
        let mut keys: Box<_> = std::iter::repeat_with(|| None).take(capacity).collect();

        if ENABLE_MLOCK {
            unsafe {
                memsec::mlock(keys.as_mut_ptr() as *mut u8, capacity * entry_size);
            }
        }

        let container = SliceKeyContainer::new(keys);

        // Validate that the entry size is correct
        debug_assert_eq!(container.entry_size(), entry_size);

        Self { container }
    }
}

impl<Key: KeyRef> ZeroizeOnDrop for RustKeyStore<Key> {}

impl<Key: KeyRef> Drop for RustKeyStore<Key> {
    fn drop(&mut self) {
        if ENABLE_MLOCK {
            // We need to ensure the values get dropped and zeroized _before_
            // the mlock gets removed, to avoid any last minute swaps to disk
            self.container.clear();

            unsafe {
                memsec::munlock(
                    self.container.inner_mut().as_mut_ptr() as *mut u8,
                    self.container.byte_len(),
                );
            }
        }
    }
}

impl<Key: KeyRef> KeyStore<Key> for RustKeyStore<Key> {
    fn insert(&mut self, key_ref: Key, key: Key::KeyValue) {
        if let Err(new_capacity) = self.container.ensure_capacity(1) {
            // Create a new store with the correct capacity and replace self with it
            let mut new_self = Self::with_capacity(new_capacity);
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
