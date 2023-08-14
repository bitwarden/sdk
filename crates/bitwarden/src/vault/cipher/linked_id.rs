use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug, JsonSchema)]
#[serde(untagged)]
pub enum LinkedIdType {
    Login(LoginLinkedIdType),
    Card(CardLinkedIdType),
    Identity(IdentityLinkedIdType),
}

#[cfg(feature = "mobile")]
use crate::UniffiCustomTypeConverter;
#[cfg(feature = "mobile")]
uniffi::custom_type!(LinkedIdType, u32);
#[cfg(feature = "mobile")]
impl UniffiCustomTypeConverter for LinkedIdType {
    type Builtin = u32;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        let val = serde_json::Value::Number(val.into());
        Ok(serde_json::from_value(val)?)
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        serde_json::to_value(&obj)
            .expect("LinkedIdType should be serializable")
            .as_u64()
            .expect("Not a numeric enum value") as u32
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr, Debug, JsonSchema)]
#[repr(u16)]
pub enum LoginLinkedIdType {
    Username = 100,
    Password = 101,
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr, Debug, JsonSchema)]
#[repr(u16)]
pub enum CardLinkedIdType {
    CardholderName = 300,
    ExpMonth = 301,
    ExpYear = 302,
    Code = 303,
    Brand = 304,
    Number = 305,
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr, Debug, JsonSchema)]
#[repr(u16)]
pub enum IdentityLinkedIdType {
    Title = 400,
    MiddleName = 401,
    Address1 = 402,
    Address2 = 403,
    Address3 = 404,
    City = 405,
    State = 406,
    PostalCode = 407,
    Country = 408,
    Company = 409,
    Email = 410,
    Phone = 411,
    Ssn = 412,
    Username = 413,
    PassportNumber = 414,
    LicenseNumber = 415,
    FirstName = 416,
    LastName = 417,
    FullName = 418,
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_linked_id_serialization() {
        use super::{LinkedIdType, LoginLinkedIdType};

        #[derive(serde::Serialize, serde::Deserialize)]
        struct Test {
            id: LinkedIdType,
        }

        let json = "{\"id\":100}";
        let val = serde_json::from_str::<Test>(json).unwrap();

        assert_eq!(val.id, LinkedIdType::Login(LoginLinkedIdType::Username));

        let serialized = serde_json::to_string(&val).unwrap();
        assert_eq!(serialized, json);
    }

    #[cfg(feature = "mobile")]
    #[test]
    fn test_linked_id_serialization_uniffi() {
        use super::{CardLinkedIdType, LinkedIdType, LoginLinkedIdType};

        assert_eq!(
            100,
            crate::UniffiCustomTypeConverter::from_custom(LinkedIdType::Login(
                LoginLinkedIdType::Username
            ))
        );
        assert_eq!(
            303,
            crate::UniffiCustomTypeConverter::from_custom(LinkedIdType::Card(
                CardLinkedIdType::Code
            ))
        );

        assert_eq!(
            LinkedIdType::Login(LoginLinkedIdType::Username),
            crate::UniffiCustomTypeConverter::into_custom(100).unwrap()
        );
        assert_eq!(
            LinkedIdType::Card(CardLinkedIdType::Code),
            crate::UniffiCustomTypeConverter::into_custom(303).unwrap()
        );
    }
}
