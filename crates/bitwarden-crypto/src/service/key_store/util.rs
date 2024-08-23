use crate::service::key_ref::KeyRef;

// This trait represents some data stored sequentially in memory, with a fixed size.
// We use this to abstract the implementation over Vec/Box<[u8]/NonNull<[u8]>.
#[allow(drop_bounds)]
pub(crate) trait KeyData<Key: KeyRef>: Drop {
    fn new_with_capacity(capacity: usize) -> Self;

    fn get_key_data(&self) -> &[Option<(Key, Key::KeyValue)>];
    fn get_key_data_mut(&mut self) -> &mut [Option<(Key, Key::KeyValue)>];
}

/// This represents a container over an arbitrary fixed size slice.
/// This is meant to abstract over the different ways to store keys in memory,
/// whether we're using a Vec, a Box<[u8]> or a NonNull<u8>.
pub(crate) struct SliceKeyContainer<Key: KeyRef, Data: KeyData<Key>> {
    // This represents the number of elements in the container, it's always less than or equal to
    // the length of `data`.
    length: usize,

    // This represents the maximum number of elements that can be stored in the container.
    // This is always equal to the length of `data`, but we store it to avoid recomputing it.
    capacity: usize,

    // This is the actual data that stores the keys, optional as we can have it not yet
    // uninitialized
    data: Option<Data>,

    _key: std::marker::PhantomData<Key>,
}

impl<Key: KeyRef, Data: KeyData<Key>> Drop for SliceKeyContainer<Key, Data> {
    fn drop(&mut self) {
        self.clear();

        // Ensure the container gets dropped after the data is cleared
        let _ = self.data.take();
    }
}

#[allow(dead_code)]
impl<Key: KeyRef, Data: KeyData<Key>> SliceKeyContainer<Key, Data> {
    pub(crate) fn new_with_capacity(capacity: usize) -> Self {
        if capacity == 0 {
            return Self {
                length: 0,
                capacity: 0,
                data: None,
                _key: std::marker::PhantomData,
            };
        }

        let mut container = Self {
            length: 0,
            capacity,
            data: Some(Data::new_with_capacity(capacity)),
            _key: std::marker::PhantomData,
        };

        // Ensure the container is properly initialized
        container.clear();

        container
    }

    /// Check if the container has enough capacity to store `new_elements` more elements.
    /// If the result is Ok, the container has enough capacity.
    /// If it's Err, the container needs to be resized.
    /// The error value returns a suggested new capacity.
    fn check_capacity(&self, new_elements: usize) -> Result<(), usize> {
        let new_size = self.length + new_elements;

        // This is the first allocation
        if self.capacity == 0 {
            const PAGE_SIZE: usize = 4096;
            let entry_size = std::mem::size_of::<Option<(Key, Key::KeyValue)>>();

            // We're using mlock APIs to protect the memory, which lock at the page level.
            // To avoid wasting memory, we want to allocate at least a page.
            let entries_per_page = PAGE_SIZE / entry_size;
            Err(entries_per_page)

        // We need to resize the container
        } else if new_size > self.capacity {
            // We want to increase the capacity by a multiple to be mostly aligned with page size,
            // we also need to make sure that we have enough space for the new elements, so we round
            // up
            let increase_factor = usize::div_ceil(new_size, self.capacity);
            Err(self.capacity * increase_factor)

        // We still have enough capacity
        } else {
            Ok(())
        }
    }

    fn ensure_capacity(&mut self, new_elements: usize) {
        if let Err(new_capacity) = self.check_capacity(new_elements) {
            // Create a new store with the correct capacity and replace self with it
            let mut new_self = Self::new_with_capacity(new_capacity);
            new_self.copy_from(self);
            *self = new_self;
        }
    }

    // These two are just helper functions to avoid having to deal with the optional Data
    // When Data is None we just return empty slices, which don't allow any operations
    fn get_key_data(&self) -> &[Option<(Key, Key::KeyValue)>] {
        self.data.as_ref().map(|d| d.get_key_data()).unwrap_or(&[])
    }
    fn get_key_data_mut(&mut self) -> &mut [Option<(Key, Key::KeyValue)>] {
        self.data
            .as_mut()
            .map(|d| d.get_key_data_mut())
            .unwrap_or(&mut [])
    }

    fn find_by_key_ref(&self, key_ref: &Key) -> Result<usize, usize> {
        // Because we know all the None's are at the end and all the Some values are at the
        // beginning, we only need to search for the key in the first `size` elements.
        let slice = &self.get_key_data()[..self.length];

        // This structure is almost always used for reads instead of writes, so we can use a binary
        // search to optimize for the read case.
        slice.binary_search_by(|k| {
            debug_assert!(
                k.is_some(),
                "We should never have a None value in the middle of the slice"
            );

            match k {
                Some((k, _)) => k.cmp(key_ref),
                None => std::cmp::Ordering::Greater,
            }
        })
    }

    pub(crate) fn clear(&mut self) {
        self.get_key_data_mut().fill_with(|| None);
        self.length = 0;
    }

    pub(crate) fn remove(&mut self, key_ref: Key) {
        if let Ok(idx) = self.find_by_key_ref(&key_ref) {
            let len = self.length;
            let slice = self.get_key_data_mut();
            slice[idx] = None;
            slice[idx..len].rotate_left(1);
            self.length -= 1;
        }
    }

    pub(crate) fn insert(&mut self, key_ref: Key, key: <Key as KeyRef>::KeyValue) {
        match self.find_by_key_ref(&key_ref) {
            Ok(idx) => {
                // Key already exists, we just need to replace the value
                let slice = self.get_key_data_mut();
                slice[idx] = Some((key_ref, key));
            }
            Err(idx) => {
                // Make sure that we have enough capacity, and resize if needed
                self.ensure_capacity(1);

                let len = self.length;
                let slice = self.get_key_data_mut();
                if idx < len {
                    // If we're not right at the end, we have to shift all the following elements
                    // one position to the right
                    slice[idx..=len].rotate_right(1);
                }
                slice[idx] = Some((key_ref, key));
                self.length += 1;
            }
        }
    }

    pub(crate) fn get(&self, key_ref: Key) -> Option<&<Key as KeyRef>::KeyValue> {
        self.find_by_key_ref(&key_ref)
            .ok()
            .and_then(|idx| self.get_key_data().get(idx))
            .and_then(|f| f.as_ref().map(|f| &f.1))
    }

    pub(crate) fn retain(&mut self, f: fn(Key) -> bool) {
        let len = self.length;
        let slice = self.get_key_data_mut();

        let mut removed_elements = 0;

        for value in slice.iter_mut().take(len) {
            let key = value
                .as_ref()
                .map(|e| e.0)
                .expect("Values in a slice are always Some");

            if !f(key) {
                *value = None;
                removed_elements += 1;
            }
        }

        // If we haven't removed any elements, we don't need to compact the slice
        if removed_elements == 0 {
            return;
        }

        // Remove all the None values from the middle of the slice

        for idx in 0..len {
            if slice[idx].is_none() {
                slice[idx..len].rotate_left(1);
            }
        }

        self.length -= removed_elements;
    }

    pub(crate) fn copy_from(&mut self, other: &mut Self) -> bool {
        if other.capacity > self.capacity {
            return false;
        }

        // Empty the current container
        self.clear();

        let new_length = other.length;

        // Move the data from the other container
        let this = self.get_key_data_mut();
        let that = other.get_key_data_mut();
        for idx in 0..new_length {
            std::mem::swap(&mut this[idx], &mut that[idx]);
        }

        // Update the length
        self.length = new_length;

        true
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use zeroize::Zeroize;

    use super::*;
    use crate::{service::key_ref::KeyRef, CryptoKey};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum TestKey {
        A,
        B(u8),
        C,
    }
    #[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
    pub struct TestKeyValue([u8; 16]);
    impl zeroize::ZeroizeOnDrop for TestKeyValue {}
    impl CryptoKey for TestKeyValue {}
    impl TestKeyValue {
        pub fn new(value: usize) -> Self {
            // Just fill the array with some values
            let mut key = [0; 16];
            key[0..8].copy_from_slice(&value.to_le_bytes());
            key[8..16].copy_from_slice(&value.to_be_bytes());
            Self(key)
        }
    }

    impl Drop for TestKeyValue {
        fn drop(&mut self) {
            self.0.as_mut().zeroize();
        }
    }

    impl KeyRef for TestKey {
        type KeyValue = TestKeyValue;

        fn is_local(&self) -> bool {
            false
        }
    }

    pub struct TestKeyContainer<Key: KeyRef> {
        keys: Vec<Option<(Key, Key::KeyValue)>>,
    }
    impl<Key: KeyRef> Drop for TestKeyContainer<Key> {
        fn drop(&mut self) {}
    }

    impl<Key: KeyRef> KeyData<Key> for TestKeyContainer<Key> {
        fn new_with_capacity(capacity: usize) -> Self {
            Self {
                keys: std::iter::repeat_with(|| None).take(capacity).collect(),
            }
        }

        fn get_key_data(&self) -> &[Option<(Key, Key::KeyValue)>] {
            self.keys.as_slice()
        }

        fn get_key_data_mut(&mut self) -> &mut [Option<(Key, Key::KeyValue)>] {
            self.keys.as_mut_slice()
        }
    }

    #[test]
    fn test_slice_container_insertion() {
        let mut container = SliceKeyContainer::<TestKey, TestKeyContainer<_>>::new_with_capacity(5);

        assert_eq!(container.get_key_data(), [None, None, None, None, None]);

        // Insert one key, which should be at the beginning
        container.insert(TestKey::B(10), TestKeyValue::new(110));
        assert_eq!(
            container.get_key_data(),
            [
                Some((TestKey::B(10), TestKeyValue::new(110))),
                None,
                None,
                None,
                None
            ]
        );

        // Insert a key that should be right after the first one
        container.insert(TestKey::C, TestKeyValue::new(1000));
        assert_eq!(
            container.get_key_data(),
            [
                Some((TestKey::B(10), TestKeyValue::new(110))),
                Some((TestKey::C, TestKeyValue::new(1000))),
                None,
                None,
                None
            ]
        );

        // Insert a key in the middle
        container.insert(TestKey::B(20), TestKeyValue::new(210));
        assert_eq!(
            container.get_key_data(),
            [
                Some((TestKey::B(10), TestKeyValue::new(110))),
                Some((TestKey::B(20), TestKeyValue::new(210))),
                Some((TestKey::C, TestKeyValue::new(1000))),
                None,
                None
            ]
        );

        // Insert a key right at the start
        container.insert(TestKey::A, TestKeyValue::new(0));
        assert_eq!(
            container.get_key_data(),
            [
                Some((TestKey::A, TestKeyValue::new(0))),
                Some((TestKey::B(10), TestKeyValue::new(110))),
                Some((TestKey::B(20), TestKeyValue::new(210))),
                Some((TestKey::C, TestKeyValue::new(1000))),
                None
            ]
        );

        // Insert a key in the middle, which fills the container
        container.insert(TestKey::B(30), TestKeyValue::new(310));
        assert_eq!(
            container.get_key_data(),
            [
                Some((TestKey::A, TestKeyValue::new(0))),
                Some((TestKey::B(10), TestKeyValue::new(110))),
                Some((TestKey::B(20), TestKeyValue::new(210))),
                Some((TestKey::B(30), TestKeyValue::new(310))),
                Some((TestKey::C, TestKeyValue::new(1000))),
            ]
        );

        // Replacing an existing value at the start
        container.insert(TestKey::A, TestKeyValue::new(1));
        assert_eq!(
            container.get_key_data(),
            [
                Some((TestKey::A, TestKeyValue::new(1))),
                Some((TestKey::B(10), TestKeyValue::new(110))),
                Some((TestKey::B(20), TestKeyValue::new(210))),
                Some((TestKey::B(30), TestKeyValue::new(310))),
                Some((TestKey::C, TestKeyValue::new(1000))),
            ]
        );

        // Replacing an existing value at the middle
        container.insert(TestKey::B(20), TestKeyValue::new(211));
        assert_eq!(
            container.get_key_data(),
            [
                Some((TestKey::A, TestKeyValue::new(1))),
                Some((TestKey::B(10), TestKeyValue::new(110))),
                Some((TestKey::B(20), TestKeyValue::new(211))),
                Some((TestKey::B(30), TestKeyValue::new(310))),
                Some((TestKey::C, TestKeyValue::new(1000))),
            ]
        );

        // Replacing an existing value at the end
        container.insert(TestKey::C, TestKeyValue::new(1001));
        assert_eq!(
            container.get_key_data(),
            [
                Some((TestKey::A, TestKeyValue::new(1))),
                Some((TestKey::B(10), TestKeyValue::new(110))),
                Some((TestKey::B(20), TestKeyValue::new(211))),
                Some((TestKey::B(30), TestKeyValue::new(310))),
                Some((TestKey::C, TestKeyValue::new(1001))),
            ]
        );
    }

    #[test]
    fn test_slice_container_get() {
        let mut container = SliceKeyContainer::<TestKey, TestKeyContainer<_>>::new_with_capacity(5);

        for (key, value) in [
            (TestKey::A, TestKeyValue::new(1)),
            (TestKey::B(10), TestKeyValue::new(110)),
            (TestKey::C, TestKeyValue::new(1000)),
        ] {
            container.insert(key, value);
        }

        assert_eq!(container.get(TestKey::A), Some(&TestKeyValue::new(1)));
        assert_eq!(container.get(TestKey::B(10)), Some(&TestKeyValue::new(110)));
        assert_eq!(container.get(TestKey::B(20)), None);
        assert_eq!(container.get(TestKey::C), Some(&TestKeyValue::new(1000)));
    }

    #[test]
    fn test_slice_container_clear() {
        let mut container = SliceKeyContainer::<TestKey, TestKeyContainer<_>>::new_with_capacity(5);

        for (key, value) in [
            (TestKey::A, TestKeyValue::new(1)),
            (TestKey::B(10), TestKeyValue::new(110)),
            (TestKey::B(20), TestKeyValue::new(210)),
            (TestKey::B(30), TestKeyValue::new(310)),
            (TestKey::C, TestKeyValue::new(1000)),
        ] {
            container.insert(key, value);
        }

        assert_eq!(
            container.get_key_data(),
            [
                Some((TestKey::A, TestKeyValue::new(1))),
                Some((TestKey::B(10), TestKeyValue::new(110))),
                Some((TestKey::B(20), TestKeyValue::new(210))),
                Some((TestKey::B(30), TestKeyValue::new(310))),
                Some((TestKey::C, TestKeyValue::new(1000))),
            ]
        );

        container.clear();

        assert_eq!(container.get_key_data(), [None, None, None, None, None]);
    }

    #[test]
    fn test_slice_container_ensure_capacity() {
        let mut container = SliceKeyContainer::<TestKey, TestKeyContainer<_>>::new_with_capacity(5);

        assert_eq!(container.capacity, 5);
        assert_eq!(container.length, 0);

        assert_eq!(container.check_capacity(0), Ok(()));
        assert_eq!(container.check_capacity(6), Err(10));
        assert_eq!(container.check_capacity(10), Err(10));
        assert_eq!(container.check_capacity(11), Err(15));
        assert_eq!(container.check_capacity(51), Err(55));

        for (key, value) in [
            (TestKey::A, TestKeyValue::new(1)),
            (TestKey::B(10), TestKeyValue::new(110)),
            (TestKey::B(20), TestKeyValue::new(210)),
            (TestKey::B(30), TestKeyValue::new(310)),
            (TestKey::C, TestKeyValue::new(1000)),
        ] {
            container.insert(key, value);
        }

        assert_eq!(container.check_capacity(0), Ok(()));
        assert_eq!(container.check_capacity(6), Err(15));
        assert_eq!(container.check_capacity(10), Err(15));
        assert_eq!(container.check_capacity(11), Err(20));
        assert_eq!(container.check_capacity(51), Err(60));
    }

    #[test]
    fn test_slice_container_removal() {
        let mut container = SliceKeyContainer::<TestKey, TestKeyContainer<_>>::new_with_capacity(5);

        for (key, value) in [
            (TestKey::A, TestKeyValue::new(1)),
            (TestKey::B(10), TestKeyValue::new(110)),
            (TestKey::B(20), TestKeyValue::new(210)),
            (TestKey::B(30), TestKeyValue::new(310)),
            (TestKey::C, TestKeyValue::new(1000)),
        ] {
            container.insert(key, value);
        }

        // Remove the last element
        container.remove(TestKey::C);
        assert_eq!(
            container.get_key_data(),
            [
                Some((TestKey::A, TestKeyValue::new(1))),
                Some((TestKey::B(10), TestKeyValue::new(110))),
                Some((TestKey::B(20), TestKeyValue::new(210))),
                Some((TestKey::B(30), TestKeyValue::new(310))),
                None,
            ]
        );

        // Remove the first element
        container.remove(TestKey::A);
        assert_eq!(
            container.get_key_data(),
            [
                Some((TestKey::B(10), TestKeyValue::new(110))),
                Some((TestKey::B(20), TestKeyValue::new(210))),
                Some((TestKey::B(30), TestKeyValue::new(310))),
                None,
                None
            ]
        );

        // Remove a non-existing element
        container.remove(TestKey::A);
        assert_eq!(
            container.get_key_data(),
            [
                Some((TestKey::B(10), TestKeyValue::new(110))),
                Some((TestKey::B(20), TestKeyValue::new(210))),
                Some((TestKey::B(30), TestKeyValue::new(310))),
                None,
                None
            ]
        );

        // Remove an element in the middle
        container.remove(TestKey::B(20));
        assert_eq!(
            container.get_key_data(),
            [
                Some((TestKey::B(10), TestKeyValue::new(110))),
                Some((TestKey::B(30), TestKeyValue::new(310))),
                None,
                None,
                None
            ]
        );

        // Remove all the remaining elements
        container.remove(TestKey::B(30));
        assert_eq!(
            container.get_key_data(),
            [
                Some((TestKey::B(10), TestKeyValue::new(110))),
                None,
                None,
                None,
                None
            ]
        );
        container.remove(TestKey::B(10));
        assert_eq!(container.get_key_data(), [None, None, None, None, None]);

        // Remove from an empty container
        container.remove(TestKey::B(10));
        assert_eq!(container.get_key_data(), [None, None, None, None, None]);
    }

    #[test]
    fn test_slice_container_retain_removes_one() {
        let mut container = SliceKeyContainer::<TestKey, TestKeyContainer<_>>::new_with_capacity(5);

        for (key, value) in [
            (TestKey::A, TestKeyValue::new(1)),
            (TestKey::B(10), TestKeyValue::new(110)),
            (TestKey::B(20), TestKeyValue::new(210)),
            (TestKey::B(30), TestKeyValue::new(310)),
            (TestKey::C, TestKeyValue::new(1000)),
        ] {
            container.insert(key, value);
        }

        // Remove the last element
        container.retain(|k| k != TestKey::C);
        assert_eq!(
            container.get_key_data(),
            [
                Some((TestKey::A, TestKeyValue::new(1))),
                Some((TestKey::B(10), TestKeyValue::new(110))),
                Some((TestKey::B(20), TestKeyValue::new(210))),
                Some((TestKey::B(30), TestKeyValue::new(310))),
                None,
            ]
        );

        // Remove the first element
        container.retain(|k| k != TestKey::A);
        assert_eq!(
            container.get_key_data(),
            [
                Some((TestKey::B(10), TestKeyValue::new(110))),
                Some((TestKey::B(20), TestKeyValue::new(210))),
                Some((TestKey::B(30), TestKeyValue::new(310))),
                None,
                None
            ]
        );

        // Remove a non-existing element
        container.retain(|k| k != TestKey::A);
        assert_eq!(
            container.get_key_data(),
            [
                Some((TestKey::B(10), TestKeyValue::new(110))),
                Some((TestKey::B(20), TestKeyValue::new(210))),
                Some((TestKey::B(30), TestKeyValue::new(310))),
                None,
                None
            ]
        );

        // Remove an element in the middle
        container.retain(|k| k != TestKey::B(20));
        assert_eq!(
            container.get_key_data(),
            [
                Some((TestKey::B(10), TestKeyValue::new(110))),
                Some((TestKey::B(30), TestKeyValue::new(310))),
                None,
                None,
                None
            ]
        );

        // Remove all the remaining elements
        container.retain(|k| k != TestKey::B(30));
        assert_eq!(
            container.get_key_data(),
            [
                Some((TestKey::B(10), TestKeyValue::new(110))),
                None,
                None,
                None,
                None
            ]
        );
        container.retain(|k| k != TestKey::B(10));
        assert_eq!(container.get_key_data(), [None, None, None, None, None]);

        // Remove from an empty container
        container.retain(|k| k != TestKey::B(10));
        assert_eq!(container.get_key_data(), [None, None, None, None, None]);
    }

    #[test]
    fn test_slice_container_retain_removes_none() {
        let mut container = SliceKeyContainer::<TestKey, TestKeyContainer<_>>::new_with_capacity(5);

        for (key, value) in [
            (TestKey::A, TestKeyValue::new(1)),
            (TestKey::B(10), TestKeyValue::new(110)),
            (TestKey::B(20), TestKeyValue::new(210)),
            (TestKey::B(30), TestKeyValue::new(310)),
            (TestKey::C, TestKeyValue::new(1000)),
        ] {
            container.insert(key, value);
        }

        container.retain(|_k| true);
        assert_eq!(
            container.get_key_data(),
            [
                Some((TestKey::A, TestKeyValue::new(1))),
                Some((TestKey::B(10), TestKeyValue::new(110))),
                Some((TestKey::B(20), TestKeyValue::new(210))),
                Some((TestKey::B(30), TestKeyValue::new(310))),
                Some((TestKey::C, TestKeyValue::new(1000))),
            ]
        );
    }

    #[test]
    fn test_slice_container_retain_removes_some() {
        let mut container = SliceKeyContainer::<TestKey, TestKeyContainer<_>>::new_with_capacity(5);

        for (key, value) in [
            (TestKey::A, TestKeyValue::new(1)),
            (TestKey::B(10), TestKeyValue::new(110)),
            (TestKey::B(20), TestKeyValue::new(210)),
            (TestKey::B(30), TestKeyValue::new(310)),
            (TestKey::C, TestKeyValue::new(1000)),
        ] {
            container.insert(key, value);
        }

        container.retain(|k| matches!(k, TestKey::A | TestKey::B(20) | TestKey::C));
        assert_eq!(
            container.get_key_data(),
            [
                Some((TestKey::A, TestKeyValue::new(1))),
                Some((TestKey::B(20), TestKeyValue::new(210))),
                Some((TestKey::C, TestKeyValue::new(1000))),
                None,
                None,
            ]
        );
    }

    #[test]
    fn test_slice_container_retain_removes_all() {
        let mut container = SliceKeyContainer::<TestKey, TestKeyContainer<_>>::new_with_capacity(5);

        for (key, value) in [
            (TestKey::A, TestKeyValue::new(1)),
            (TestKey::B(10), TestKeyValue::new(110)),
            (TestKey::B(20), TestKeyValue::new(210)),
            (TestKey::B(30), TestKeyValue::new(310)),
            (TestKey::C, TestKeyValue::new(1000)),
        ] {
            container.insert(key, value);
        }

        container.retain(|_k| false);
        assert_eq!(container.get_key_data(), [None, None, None, None, None]);
    }
}
