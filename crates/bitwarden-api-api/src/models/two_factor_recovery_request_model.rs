/*
 * Bitwarden Internal API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TwoFactorRecoveryRequestModel {
    #[serde(rename = "masterPasswordHash", skip_serializing_if = "Option::is_none")]
    pub master_password_hash: Option<String>,
    #[serde(rename = "otp", skip_serializing_if = "Option::is_none")]
    pub otp: Option<String>,
    #[serde(
        rename = "authRequestAccessCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub auth_request_access_code: Option<String>,
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "authRequestId", skip_serializing_if = "Option::is_none")]
    pub auth_request_id: Option<String>,
    #[serde(
        rename = "ssoEmail2FaSessionToken",
        skip_serializing_if = "Option::is_none"
    )]
    pub sso_email2_fa_session_token: Option<String>,
    #[serde(rename = "recoveryCode")]
    pub recovery_code: String,
}

impl TwoFactorRecoveryRequestModel {
    pub fn new(email: String, recovery_code: String) -> TwoFactorRecoveryRequestModel {
        TwoFactorRecoveryRequestModel {
            master_password_hash: None,
            otp: None,
            auth_request_access_code: None,
            secret: None,
            email,
            auth_request_id: None,
            sso_email2_fa_session_token: None,
            recovery_code,
        }
    }
}
