use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::vault::{Cipher, CipherView};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CipherDecryptRequest {
    pub cipher: Box<Cipher>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CipherDecryptResponse {
    pub cipher: Box<CipherView>,
}
