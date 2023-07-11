use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::TwoFactorRequest;

/// Login to Bitwarden with Username and Password
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PasswordLoginRequest {
    /// Bitwarden account email address
    pub email: String,
    /// Bitwarden account master password
    pub password: String,
    // Two-factor authentication
    pub two_factor: Option<TwoFactorRequest>,
}
