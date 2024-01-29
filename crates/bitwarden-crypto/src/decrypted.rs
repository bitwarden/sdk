use std::{
    borrow::Cow,
    fmt::{self, Formatter},
};

use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::{CryptoError, CryptoKey, KeyEncryptable};

/// Wrapper for decrypted values which makes a best effort to enforce zeroization of the inner value
/// on drop. The inner value exposes a [`Decrypted::expose`] method which returns a reference to the
/// inner value. Care must be taken to avoid accidentally exposing the inner value through copying
/// or cloning.
///
/// Internally [`Decrypted`] contains a [`Box`] which ensures the value is placed on the heap. It
/// implements the [`Drop`] trait which calls `zeroize` on the inner value.
#[derive(PartialEq, Clone, Zeroize, ZeroizeOnDrop)]
pub struct Decrypted<V: Zeroize> {
    value: Box<V>,
}

impl<V: Zeroize> Decrypted<V> {
    /// Create a new [`Decrypted`] value. In an attempt to avoid accidentally placing this on the
    /// stack it only accepts a [`Box`] value. The rust compiler should be able to optimize away the
    /// initial stack allocation presuming the value is not used before being boxed.
    pub fn new(value: Box<V>) -> Self {
        Self { value }
    }

    /// Expose the inner value. By exposing the inner value, you take responsibility for ensuring
    /// that any copy of the value is zeroized.
    pub fn expose(&self) -> &V {
        &self.value
    }

    /// Expose the inner value mutable. By exposing the inner value, you take responsibility for
    /// ensuring that any copy of the value is zeroized.
    pub fn expose_mut(&mut self) -> &mut V {
        &mut self.value
    }
}

/// Helper to convert a `Decrypted<Vec<u8>>` to a `Decrypted<String>`, care is taken to ensure any
/// intermediate copies are zeroed to avoid leaking sensitive data.
impl TryFrom<DecryptedVec> for DecryptedString {
    type Error = CryptoError;

    fn try_from(mut v: DecryptedVec) -> Result<Self, CryptoError> {
        let value = std::mem::take(&mut v.value);

        let rtn = String::from_utf8(*value).map_err(|_| CryptoError::InvalidUtf8String);
        rtn.map(|v| Decrypted::new(Box::new(v)))
    }
}

impl<V: Zeroize + Default> Default for Decrypted<V> {
    fn default() -> Self {
        Self::new(Box::default())
    }
}

impl<V: Zeroize + Serialize> fmt::Debug for Decrypted<V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Decrypted")
            .field("type", &std::any::type_name::<V>())
            .field("value", &"********")
            .finish()
    }
}

/// Unfortunately once we serialize a `DecryptedString` we can't control the future memory.
impl<V: Zeroize + Serialize> Serialize for Decrypted<V> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.value.serialize(serializer)
    }
}

impl<'de, V: Zeroize + Deserialize<'de>> Deserialize<'de> for Decrypted<V> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::new(Box::new(V::deserialize(deserializer)?)))
    }
}

/// Transparently expose the inner value for serialization
impl<V: Zeroize + JsonSchema> JsonSchema for Decrypted<V> {
    fn schema_name() -> String {
        V::schema_name()
    }

    fn schema_id() -> Cow<'static, str> {
        V::schema_id()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        V::json_schema(gen)
    }
}

impl<T: KeyEncryptable<Key, Output> + Zeroize + Clone, Key: CryptoKey, Output>
    KeyEncryptable<Key, Output> for Decrypted<T>
{
    fn encrypt_with_key(self, key: &Key) -> Result<Output, CryptoError> {
        self.value.clone().encrypt_with_key(key)
    }
}

pub type DecryptedVec = Decrypted<Vec<u8>>;
pub type DecryptedString = Decrypted<String>;
