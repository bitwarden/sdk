/*
 * Bitwarden Internal API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationApiKeyInformation {
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(rename = "keyType", skip_serializing_if = "Option::is_none")]
    pub key_type: Option<crate::models::OrganizationApiKeyType>,
    #[serde(rename = "revisionDate", skip_serializing_if = "Option::is_none")]
    pub revision_date: Option<String>,
}

impl OrganizationApiKeyInformation {
    pub fn new() -> OrganizationApiKeyInformation {
        OrganizationApiKeyInformation {
            object: None,
            key_type: None,
            revision_date: None,
        }
    }
}
