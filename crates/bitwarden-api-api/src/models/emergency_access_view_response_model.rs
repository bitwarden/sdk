/*
 * Bitwarden Internal API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmergencyAccessViewResponseModel {
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(rename = "keyEncrypted", skip_serializing_if = "Option::is_none")]
    pub key_encrypted: Option<String>,
    #[serde(rename = "ciphers", skip_serializing_if = "Option::is_none")]
    pub ciphers: Option<Vec<models::CipherResponseModel>>,
}

impl EmergencyAccessViewResponseModel {
    pub fn new() -> EmergencyAccessViewResponseModel {
        EmergencyAccessViewResponseModel {
            object: None,
            key_encrypted: None,
            ciphers: None,
        }
    }
}
