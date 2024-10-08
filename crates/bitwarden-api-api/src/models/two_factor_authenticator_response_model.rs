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
pub struct TwoFactorAuthenticatorResponseModel {
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(
        rename = "userVerificationToken",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_verification_token: Option<String>,
}

impl TwoFactorAuthenticatorResponseModel {
    pub fn new() -> TwoFactorAuthenticatorResponseModel {
        TwoFactorAuthenticatorResponseModel {
            object: None,
            enabled: None,
            key: None,
            user_verification_token: None,
        }
    }
}
