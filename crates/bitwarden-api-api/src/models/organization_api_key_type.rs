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
pub enum OrganizationApiKeyType {
    Default = 0,
    BillingSync = 1,
    Scim = 2,
}

impl ToString for OrganizationApiKeyType {
    fn to_string(&self) -> String {
        match self {
            Self::Default => String::from("0"),
            Self::BillingSync => String::from("1"),
            Self::Scim => String::from("2"),
        }
    }
}

impl Default for OrganizationApiKeyType {
    fn default() -> OrganizationApiKeyType {
        Self::Default
    }
}
