use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecretVerificationRequest {
    /// The user's master password to use for user verification. If supplied, this will be used for
    /// verification purposes.
    pub master_password: Option<String>,
    /// Alternate user verification method through OTP. This is provided for users who have no
    /// master password due to use of Customer Managed Encryption. Must be present and valid if
    /// master_password is absent.
    pub otp: Option<String>,
}
