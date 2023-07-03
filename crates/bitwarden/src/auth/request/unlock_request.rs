use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Unlock the Bitwarden vault
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UnlockRequest {
    /// Bitwarden account master password
    pub password: String,
}
