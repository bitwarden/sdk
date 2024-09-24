use super::{
    util::{KeyData, SliceKeyStore},
    KeyRef,
};

// This is a basic in-memory key store for the cases where we don't have a secure key store
// available. We still make use mlock to protect the memory from being swapped to disk, and we
// zeroize the values when dropped.
pub(crate) type RustKeyStore<Key> = SliceKeyStore<Key, RustImplKeyData<Key>>;

pub(crate) struct RustImplKeyData<Key: KeyRef> {
    #[allow(clippy::type_complexity)]
    data: Box<[Option<(Key, Key::KeyValue)>]>,
}

impl<Key: KeyRef> Drop for RustImplKeyData<Key> {
    fn drop(&mut self) {
        #[cfg(all(
            not(target_arch = "wasm32"),
            not(feature = "no-memory-hardening"),
            not(windows)
        ))]
        {
            use std::mem::MaybeUninit;

            let entry_size = std::mem::size_of::<Option<(Key, Key::KeyValue)>>();
            unsafe {
                memsec::munlock(
                    self.data.as_mut_ptr() as *mut u8,
                    self.data.len() * entry_size,
                );

                // Note: munlock is zeroing the memory, which leaves the data in an inconsistent
                // state. So we need to set it to None again, in case any Drop impl
                // expects the data to be correct.
                let uninit_slice: &mut [MaybeUninit<_>] = std::slice::from_raw_parts_mut(
                    self.data.as_mut_ptr() as *mut MaybeUninit<Option<(Key, Key::KeyValue)>>,
                    self.data.len(),
                );
                for elem in uninit_slice {
                    elem.write(None);
                }
            }
        }
    }
}

impl<Key: KeyRef> KeyData<Key> for RustImplKeyData<Key> {
    fn is_available() -> bool {
        true
    }

    fn with_capacity(capacity: usize) -> Self {
        #[allow(unused_mut)]
        let mut data: Box<_> = std::iter::repeat_with(|| None).take(capacity).collect();

        #[cfg(all(
            not(target_arch = "wasm32"),
            not(feature = "no-memory-hardening"),
            not(windows)
        ))]
        {
            let entry_size = std::mem::size_of::<Option<(Key, Key::KeyValue)>>();
            unsafe {
                memsec::mlock(data.as_mut_ptr() as *mut u8, capacity * entry_size);
            }
        }
        RustImplKeyData { data }
    }

    fn get_key_data(&self) -> &[Option<(Key, Key::KeyValue)>] {
        self.data.as_ref()
    }

    fn get_key_data_mut(&mut self) -> &mut [Option<(Key, Key::KeyValue)>] {
        self.data.as_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::service::key_store::{util::tests::*, KeyStore as _};

    #[test]
    fn test_resize() {
        let mut store = RustKeyStore::<TestKey>::with_capacity(1).unwrap();

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
