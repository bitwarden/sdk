use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::vault::{Cipher, CipherView};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CipherEncryptRequest {
    pub cipher: CipherView,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CipherDecryptRequest {
    pub cipher: Cipher,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CipherDecryptListRequest {
    pub ciphers: Vec<Cipher>,
}
