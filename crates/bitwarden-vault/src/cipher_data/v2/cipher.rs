use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub struct CipherDataV2 {
    pub(crate) data: serde_json::Value,
}
