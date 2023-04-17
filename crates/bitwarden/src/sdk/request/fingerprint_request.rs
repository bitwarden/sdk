use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FingerprintRequest {
    /// The user's user id, used in the fingerprint generation process.
    pub user_id: String,
    /// The user's public key
    pub public_key: String,
}
