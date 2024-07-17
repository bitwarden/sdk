pub mod conversions;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::version_agnostic::Data;

#[derive(Clone, Serialize, Deserialize, Debug, Default, JsonSchema)]
pub struct CipherDataV2 {
    pub(crate) data: serde_json::Value,
}

impl From<CipherDataV2> for Data {
    fn from(value: CipherDataV2) -> Self {
        Data::V2(value)
    }
}
