use std::{
    borrow::Cow,
    fmt::{self, Formatter},
};

use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Wrapper for sensitive values which makes a best effort to enforce zeroization of the inner value
/// on drop. The inner value exposes a [`Sensitive::expose`] method which returns a reference to the
/// inner value. Care must be taken to avoid accidentally exposing the inner value through copying
/// or cloning.
///
/// Internally [`Sensitive`] contains a [`Box`] which ensures the value is placed on the heap. It
/// implements the [`Drop`] trait which calls `zeroize` on the inner value.
#[derive(PartialEq, Clone, Zeroize, ZeroizeOnDrop)]
pub struct Sensitive<V: Zeroize> {
    pub(super) value: Box<V>,
}

/// Important: This type does not protect against reallocations made by the Vec.
/// This means that if you insert any elements past the capacity, the data will be copied to a
/// new allocation and the old allocation will not be zeroized.
/// To avoid this, use Vec::with_capacity to preallocate the capacity you need.
pub type SensitiveVec = Sensitive<Vec<u8>>;

impl<V: Zeroize> Sensitive<V> {
    /// Create a new [`Sensitive`] value. In an attempt to avoid accidentally placing this on the
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

/// Helper to convert a `Sensitive<[u8, N]>` to a `SensitiveVec`.
impl<const N: usize> From<Sensitive<[u8; N]>> for SensitiveVec {
    fn from(sensitive: Sensitive<[u8; N]>) -> Self {
        SensitiveVec::new(Box::new(sensitive.value.to_vec()))
    }
}

impl<V: Zeroize + Default> Default for Sensitive<V> {
    fn default() -> Self {
        Self::new(Box::default())
    }
}

impl<V: Zeroize + Serialize> fmt::Debug for Sensitive<V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Sensitive")
            .field("type", &std::any::type_name::<V>())
            .field("value", &"********")
            .finish()
    }
}

/// Unfortunately once we serialize a `SensitiveString` we can't control the future memory.
impl<V: Zeroize + Serialize> Serialize for Sensitive<V> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.value.serialize(serializer)
    }
}

impl<'de, V: Zeroize + Deserialize<'de>> Deserialize<'de> for Sensitive<V> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::new(Box::new(V::deserialize(deserializer)?)))
    }
}

/// Transparently expose the inner value for serialization
impl<V: Zeroize + JsonSchema> JsonSchema for Sensitive<V> {
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

#[cfg(test)]
mod tests {
    use schemars::schema_for;

    use super::*;

    /*
    #[test]
    fn test_debug() {
        let string = Sensitive::test("test");
        assert_eq!(
            format!("{:?}", string),
            "Sensitive { type: \"alloc::string::String\", value: \"********\" }"
        );

        let vector = Sensitive::new(Box::new(vec![1, 2, 3]));
        assert_eq!(
            format!("{:?}", vector),
            "Sensitive { type: \"alloc::vec::Vec<i32>\", value: \"********\" }"
        );
    }
    */

    #[test]
    fn test_schemars() {
        #[derive(JsonSchema)]
        struct TestStruct {
            #[allow(dead_code)]
            v: SensitiveVec,
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
                    "$ref": "#/definitions/Array_of_uint8"
                }
            },
            "definitions": {
                "Array_of_uint8": {
                    "type": "array",
                    "items": {
                        "type": "integer",
                        "format": "uint8",
                        "minimum": 0.0
                    }
                }
            }
        }"##;

        assert_eq!(
            json.parse::<serde_json::Value>().unwrap(),
            expected.parse::<serde_json::Value>().unwrap()
        );
    }
}
