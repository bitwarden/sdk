/*
 * Bitwarden Internal API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

///
#[repr(i64)]
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize_repr, Deserialize_repr,
)]
pub enum PushType {
    Variant0 = 0,
    Variant1 = 1,
    Variant2 = 2,
    Variant3 = 3,
    Variant4 = 4,
    Variant5 = 5,
    Variant6 = 6,
    Variant7 = 7,
    Variant8 = 8,
    Variant9 = 9,
    Variant10 = 10,
    Variant11 = 11,
    Variant12 = 12,
    Variant13 = 13,
    Variant14 = 14,
    Variant15 = 15,
    Variant16 = 16,
}

impl ToString for PushType {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0 => String::from("0"),
            Self::Variant1 => String::from("1"),
            Self::Variant2 => String::from("2"),
            Self::Variant3 => String::from("3"),
            Self::Variant4 => String::from("4"),
            Self::Variant5 => String::from("5"),
            Self::Variant6 => String::from("6"),
            Self::Variant7 => String::from("7"),
            Self::Variant8 => String::from("8"),
            Self::Variant9 => String::from("9"),
            Self::Variant10 => String::from("10"),
            Self::Variant11 => String::from("11"),
            Self::Variant12 => String::from("12"),
            Self::Variant13 => String::from("13"),
            Self::Variant14 => String::from("14"),
            Self::Variant15 => String::from("15"),
            Self::Variant16 => String::from("16"),
        }
    }
}

impl Default for PushType {
    fn default() -> PushType {
        Self::Variant0
    }
}
