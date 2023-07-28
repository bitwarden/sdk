use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::vault::Cipher;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CipherEncryptResponse {
    pub cipher: Cipher,
}
