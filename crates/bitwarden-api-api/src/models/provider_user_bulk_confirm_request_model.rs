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
pub struct ProviderUserBulkConfirmRequestModel {
    #[serde(rename = "keys")]
    pub keys: Vec<crate::models::ProviderUserBulkConfirmRequestModelEntry>,
}

impl ProviderUserBulkConfirmRequestModel {
    pub fn new(
        keys: Vec<crate::models::ProviderUserBulkConfirmRequestModelEntry>,
    ) -> ProviderUserBulkConfirmRequestModel {
        ProviderUserBulkConfirmRequestModel { keys }
    }
}
