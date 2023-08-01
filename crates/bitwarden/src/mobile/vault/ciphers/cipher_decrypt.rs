use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::vault::{Cipher, CipherListView, CipherView};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CipherDecryptRequest {
    pub cipher: Cipher,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CipherDecryptResponse {
    pub cipher: CipherView,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CipherDecryptListResponse {
    pub ciphers: Vec<CipherListView>,
}
