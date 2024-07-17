use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::version_agnostic::Data;

mod conversions;

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub struct CipherDataV1 {
    pub(crate) data: serde_json::Value,
}

impl From<CipherDataV1> for Data {
    fn from(value: CipherDataV1) -> Self {
        Data::V1(value)
    }
}
