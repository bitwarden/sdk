use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::UniffiCustomTypeConverter;

use super::v1::CipherDataV1;
use super::v2::CipherDataV2;

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
enum Data {
    V1(CipherDataV1),
    V2(CipherDataV2),
    Latest(CipherDataLatest),
}

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub struct CipherData {
    data: Data,
}

uniffi::custom_type!(CipherData, String);

impl UniffiCustomTypeConverter for CipherData {
    type Builtin = String;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(serde_json::from_str(&val)?)
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        // TODO: Fix unwrap?
        serde_json::to_string(&obj).unwrap()
    }
}
