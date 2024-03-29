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
pub struct CipherShareRequestModel {
    #[serde(rename = "collectionIds")]
    pub collection_ids: Vec<String>,
    #[serde(rename = "cipher")]
    pub cipher: Box<crate::models::CipherRequestModel>,
}

impl CipherShareRequestModel {
    pub fn new(
        collection_ids: Vec<String>,
        cipher: crate::models::CipherRequestModel,
    ) -> CipherShareRequestModel {
        CipherShareRequestModel {
            collection_ids,
            cipher: Box::new(cipher),
        }
    }
}
