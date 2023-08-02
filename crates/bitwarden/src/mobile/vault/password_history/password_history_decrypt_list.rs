use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::vault::{PasswordHistory, PasswordHistoryView};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PasswordHistoryDecryptListRequest {
    pub history: PasswordHistory,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PasswordHistoryDecryptListResponse {
    pub history: PasswordHistoryView,
}
