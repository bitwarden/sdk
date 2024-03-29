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
pub struct SendAccessRequestModel {
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

impl SendAccessRequestModel {
    pub fn new() -> SendAccessRequestModel {
        SendAccessRequestModel { password: None }
    }
}
