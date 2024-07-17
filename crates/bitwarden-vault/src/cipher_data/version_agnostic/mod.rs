use bitwarden_versioning::Versioned;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::UniffiCustomTypeConverter;

use super::v1::CipherDataV1;
use super::v2::CipherDataV2;

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
enum Data {
    V1(CipherDataV1),
    V2(CipherDataV2),
}

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub struct VersionedCipherData {
    data: Versioned<Data, CipherDataV2>,
}

uniffi::custom_type!(VersionedCipherData, String);

impl UniffiCustomTypeConverter for VersionedCipherData {
    type Builtin = String;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(serde_json::from_str(&val)?)
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        // TODO: Fix unwrap?
        serde_json::to_string(&obj).unwrap()
    }
}
