use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::vault::{Cipher, CipherListView};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct CipherDecryptListRequest {
    pub ciphers: Vec<Cipher>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct CipherDecryptListResponse {
    pub ciphers: Vec<CipherListView>,
}
