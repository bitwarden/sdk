use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::vault::PasswordHistoryView;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PasswordHistoryEncryptRequest {
    pub history: PasswordHistoryView,
}
