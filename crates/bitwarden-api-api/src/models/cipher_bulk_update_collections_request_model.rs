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
pub struct CipherBulkUpdateCollectionsRequestModel {
    #[serde(rename = "organizationId", skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<uuid::Uuid>,
    #[serde(rename = "cipherIds", skip_serializing_if = "Option::is_none")]
    pub cipher_ids: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "collectionIds", skip_serializing_if = "Option::is_none")]
    pub collection_ids: Option<Vec<uuid::Uuid>>,
    /// If true, the collections will be removed from the ciphers. Otherwise, they will be added.
    #[serde(rename = "removeCollections", skip_serializing_if = "Option::is_none")]
    pub remove_collections: Option<bool>,
}

impl CipherBulkUpdateCollectionsRequestModel {
    pub fn new() -> CipherBulkUpdateCollectionsRequestModel {
        CipherBulkUpdateCollectionsRequestModel {
            organization_id: None,
            cipher_ids: None,
            collection_ids: None,
            remove_collections: None,
        }
    }
}
