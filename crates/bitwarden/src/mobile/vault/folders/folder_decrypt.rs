use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::vault::{Folder, FolderView};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FolderDecryptRequest {
    pub folder: Folder,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FolderDecryptResponse {
    pub folder: FolderView,
}
