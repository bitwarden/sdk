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
pub enum KdfType {
    PBKDF2_SHA256 = 0,
    Argon2id = 1,
}

impl ToString for KdfType {
    fn to_string(&self) -> String {
        match self {
            Self::PBKDF2_SHA256 => String::from("0"),
            Self::Argon2id => String::from("1"),
        }
    }
}

impl Default for KdfType {
    fn default() -> KdfType {
        Self::PBKDF2_SHA256
    }
}
