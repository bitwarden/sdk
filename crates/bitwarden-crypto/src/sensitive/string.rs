use std::borrow::Cow;

use schemars::JsonSchema;
use zeroize::{Zeroize, Zeroizing};

use crate::{Sensitive, SensitiveVec};

/// A sensitive string that supports string operations.
///
/// Important: The `SensitiveString` protects against reallocations in the internal string. However,
/// be careful when using any str or byte slices as taking ownership of them will create a copy
/// which is not protected.
#[derive(Eq, Clone, Zeroize)]
pub struct SensitiveString {
    inner: Zeroizing<String>,
}

impl std::fmt::Debug for SensitiveString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Sensitive")
            .field("type", &std::any::type_name::<String>())
            .field("value", &"********")
            .finish()
    }
}

impl SensitiveString {
    #[inline(always)]
    pub fn new(inner: String) -> Self {
        Self {
            inner: Zeroizing::new(inner),
        }
    }

    /// Creates a new empty `SensitiveString` with at least the specified capacity.
    pub fn with_capacity(len: usize) -> Self {
        Self {
            inner: Zeroizing::new(String::with_capacity(len)),
        }
    }

    /// Extend the size of the string by `size`.
    ///
    /// Internally creates a new string with the new size and copies the old string into it,
    /// zeroing the original.
    fn extend_size(&mut self, size: usize) {
        let new_size = self.inner.len() + size;
        let mut new_inner = Zeroizing::new(String::with_capacity(new_size));
        new_inner.push_str(&self.inner);

        self.inner = new_inner;
    }

    /// Extends the capacity of the string to `size` if it is larger than the current capacity.
    fn extend_spec(&mut self, size: usize) {
        if size > self.inner.capacity() {
            self.extend_size(size);
        }
    }

    /// Appends a given char onto the end of this `SensitiveString`
    ///
    /// If the capacity of the `SensitiveString` is not large enough it will zeroize the current
    /// string and create a new string with the new size.
    pub fn push(&mut self, c: char) {
        self.extend_spec(self.inner.len() + c.len_utf8());
        self.inner.push(c);
    }

    /// Appends a given string slice onto the end of this `SensitiveString`
    ///
    /// If the capacity of the `SensitiveString` is not large enough it will zeroize the current
    /// string and create a new string with the new size.
    pub fn push_str(&mut self, s: &str) {
        self.extend_spec(self.inner.len() + s.len());
        self.inner.push_str(s);
    }

    /// Expose the inner value as a str slice.
    ///
    /// Warning: Taking ownership of the inner value will create a copy of the string.
    pub fn as_str(&self) -> &str {
        &self.inner
    }

    /// Expose the inner value as a byte slice.
    ///
    /// Warning: Taking ownership of the inner value will create a copy of the string.
    pub fn as_bytes(&self) -> &[u8] {
        self.inner.as_bytes()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    // The predicate is specifically a fn() and not a closure to forbid capturing values
    // from the environment, which would make it easier to accidentally leak some data.
    // For example, the following won't compile with fn() but would work with impl Fn():
    // ```
    // let mut chars = Mutex::new(Vec::new());
    // self.any_chars(|c| {chars.lock().unwrap().push(c); true});
    // ```
    // Note that this is not a perfect solution, as it is still possible to leak the characters by
    // using a global variable or a static variable. Also `char` implements Copy so it's hard to
    // ensure the compiler is not making a copy of the character.
    #[inline(always)]
    pub fn any_chars(&self, predicate: fn(char) -> bool) -> bool {
        self.as_str().chars().any(predicate)
    }
}

impl Default for SensitiveString {
    /// Creates an empty `SensitiveString`.
    #[inline]
    fn default() -> Self {
        Self::new(String::default())
    }
}

impl From<SensitiveString> for SensitiveVec {
    fn from(mut s: SensitiveString) -> Self {
        let value: String = std::mem::take(&mut s.inner);
        Sensitive::new(Box::new(value.into_bytes()))
    }
}

impl std::ops::Index<std::ops::Range<usize>> for SensitiveString {
    type Output = str;

    #[inline]
    fn index(&self, index: std::ops::Range<usize>) -> &str {
        &self.inner[..][index]
    }
}

impl std::ops::Index<std::ops::RangeFrom<usize>> for SensitiveString {
    type Output = str;

    #[inline]
    fn index(&self, index: std::ops::RangeFrom<usize>) -> &str {
        &self.inner[..][index]
    }
}

impl PartialEq<SensitiveString> for SensitiveString {
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl PartialEq<&str> for SensitiveString {
    fn eq(&self, other: &&str) -> bool {
        self.inner.as_str().eq(*other)
    }
}

/// Transparently expose the inner value for serialization
impl JsonSchema for SensitiveString {
    fn schema_name() -> String {
        String::schema_name()
    }

    fn schema_id() -> Cow<'static, str> {
        String::schema_id()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }
}

/// Unfortunately once we serialize a `SensitiveString` we can't control the future memory.
impl serde::Serialize for SensitiveString {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.inner.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for SensitiveString {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::new(String::deserialize(deserializer)?))
    }
}

// We use a lot of `&str` and `&[u8]` in our tests, so we expose this helper
// to make it easier.
// IMPORTANT: This should not be used outside of test code
// Note that we can't just mark it with #[cfg(test)] because that only applies
// when testing this crate, not when testing other crates that depend on it.
// By at least limiting it to &'static reference we should be able to avoid accidental usages
impl SensitiveString {
    pub fn test<S: Into<String>>(value: S) -> Self {
        Self::new(value.into())
    }
}

#[cfg(test)]
mod tests {
    use schemars::schema_for;

    use super::*;

    #[test]
    fn test_senitive_string() {
        let mut s = SensitiveString::new("hello".to_string());
        s.push_str(" world");

        assert_eq!(s, "hello world");
    }

    #[test]
    fn test_len() {
        let s = SensitiveString::new("Hello, world!".to_owned());
        assert_eq!(s.len(), 13);
    }

    #[test]
    fn test_is_empty() {
        let s = SensitiveString::new("".to_owned());
        assert!(s.is_empty());
    }

    #[test]
    fn test_is_not_empty() {
        let s = SensitiveString::new("Not empty".to_owned());
        assert!(!s.is_empty());
    }

    #[test]
    fn test_index_range() {
        let s = SensitiveString::new("Hello, world!".to_owned());
        assert_eq!(&s[0..5], "Hello");
    }

    #[test]
    fn test_index_range_from() {
        let s = SensitiveString::new("Hello, world!".to_owned());
        assert_eq!(&s[7..], "world!");
    }

    #[test]
    fn test_schemars() {
        #[derive(JsonSchema)]
        struct TestStruct {
            #[allow(dead_code)]
            v: SensitiveString,
        }

        let schema = schema_for!(TestStruct);
        let json = serde_json::to_string_pretty(&schema).unwrap();
        let expected = r##"{
            "$schema": "http://json-schema.org/draft-07/schema#",
            "title": "TestStruct",
            "type": "object",
            "required": ["v"],
            "properties": {
                "v": {
                    "$ref": "#/definitions/String"
                }
            },
            "definitions": {
                "String": {
                    "type": "string"
                }
            }
        }"##;

        assert_eq!(
            json.parse::<serde_json::Value>().unwrap(),
            expected.parse::<serde_json::Value>().unwrap()
        );
    }

    #[test]
    fn test_eq_sensitive_string() {
        let s1 = SensitiveString::new("Hello, world!".to_owned());
        let s2 = SensitiveString::new("Hello, world!".to_owned());
        assert_eq!(s1, s2);
    }

    #[test]
    fn test_neq_sensitive_string() {
        let s1 = SensitiveString::new("Hello, world!".to_owned());
        let s2 = SensitiveString::new("Goodbye, world!".to_owned());
        assert_ne!(s1, s2);
    }

    #[test]
    fn test_eq_str() {
        let s = SensitiveString::new("Hello, world!".to_owned());
        assert_eq!(s, "Hello, world!");
    }

    #[test]
    fn test_neq_str() {
        let s = SensitiveString::new("Hello, world!".to_owned());
        assert_ne!(s, "Goodbye, world!");
    }
}
