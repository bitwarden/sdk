use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::vault::{PasswordHistory, PasswordHistoryView};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct PasswordHistoryEncryptRequest {
    pub history: PasswordHistoryView,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct PasswordHistoryEncryptResponse {
    pub history: PasswordHistory,
}
