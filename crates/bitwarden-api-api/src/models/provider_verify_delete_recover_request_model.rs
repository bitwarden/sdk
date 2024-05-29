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
pub struct ProviderVerifyDeleteRecoverRequestModel {
    #[serde(rename = "token")]
    pub token: String,
}

impl ProviderVerifyDeleteRecoverRequestModel {
    pub fn new(token: String) -> ProviderVerifyDeleteRecoverRequestModel {
        ProviderVerifyDeleteRecoverRequestModel { token }
    }
}
