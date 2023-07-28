use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::vault::CipherView;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CipherEncryptRequest {
    pub cipher: CipherView,
}
