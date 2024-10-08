/*
 * Bitwarden Identity
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegisterVerificationEmailClickedRequestModel {
    #[serde(rename = "email")]
    pub email: Option<String>,
    #[serde(rename = "emailVerificationToken")]
    pub email_verification_token: Option<String>,
}

impl RegisterVerificationEmailClickedRequestModel {
    pub fn new(
        email: Option<String>,
        email_verification_token: Option<String>,
    ) -> RegisterVerificationEmailClickedRequestModel {
        RegisterVerificationEmailClickedRequestModel {
            email,
            email_verification_token,
        }
    }
}
