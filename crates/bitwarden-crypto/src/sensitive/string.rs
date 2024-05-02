use zeroize::Zeroizing;

/// A sensitive string that supports string operations.
pub struct BitString {
    inner: Zeroizing<String>,
}

impl BitString {
    #[inline(always)]
    pub fn new(inner: String) -> Self {
        Self {
            inner: Zeroizing::new(inner),
        }
    }

    pub fn with_capacity(len: usize) -> Self {
        Self {
            inner: Zeroizing::new(String::with_capacity(len)),
        }
    }

    /// Extend the size of the string by `size`.
    ///
    /// Internally creates a new string with the new size and copies the old string into it.
    pub fn extend_size(&mut self, size: usize) {
        let mut new_inner = Zeroizing::new(String::with_capacity(size));
        new_inner.push_str(&self.inner);

        self.inner = new_inner;
    }

    /// Extends the capacity of the string to `size` if it is larger than the current capacity.
    pub fn extend_spec(&mut self, size: usize) {
        if size > self.inner.capacity() {
            self.extend_size(size);
        }
    }

    /// Appends a given char onto the end of this `BitString`
    ///
    /// If the capacity of the `BitString` is not large enough it will zeroize the current string
    /// and create a new string with the new size.
    pub fn push(&mut self, c: char) {
        self.extend_spec(self.inner.len() + c.len_utf8());
        self.inner.push(c);
    }

    /// Appends a given string slice onto the end of this `BitString`
    ///
    /// If the capacity of the `BitString` is not large enough it will zeroize the current string
    /// and create a new string with the new size.
    pub fn push_str(&mut self, s: &str) {
        self.extend_spec(self.inner.len() + s.len());
        self.inner.push_str(s);
    }

    pub fn as_str(&self) -> &str {
        &self.inner
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl std::ops::Index<std::ops::Range<usize>> for BitString {
    type Output = str;

    #[inline]
    fn index(&self, index: std::ops::Range<usize>) -> &str {
        &self.inner[..][index]
    }
}

impl std::ops::Index<std::ops::RangeFrom<usize>> for BitString {
    type Output = str;

    #[inline]
    fn index(&self, index: std::ops::RangeFrom<usize>) -> &str {
        &self.inner[..][index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_string() {
        let mut bit_string = BitString::new("hello".to_string());
        bit_string.push_str(" world");

        assert_eq!(bit_string.inner.as_str(), "hello world");
    }

    #[test]
    fn test_len() {
        let s = BitString::new("Hello, world!".to_owned());
        assert_eq!(s.len(), 13);
    }

    #[test]
    fn test_is_empty() {
        let s = BitString::new("".to_owned());
        assert!(s.is_empty());
    }

    #[test]
    fn test_is_not_empty() {
        let s = BitString::new("Not empty".to_owned());
        assert!(!s.is_empty());
    }

    #[test]
    fn test_index_range() {
        let s = BitString::new("Hello, world!".to_owned());
        assert_eq!(&s[0..5], "Hello");
    }

    #[test]
    fn test_index_range_from() {
        let s = BitString::new("Hello, world!".to_owned());
        assert_eq!(&s[7..], "world!");
    }
}
