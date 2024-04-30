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
}
