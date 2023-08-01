use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::vault::{Cipher, CipherView};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CipherEncryptRequest {
    pub cipher: Box<CipherView>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CipherEncryptResponse {
    pub cipher: Box<Cipher>,
}
