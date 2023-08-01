use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::vault::{Folder, FolderView};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FolderDecryptListRequest {
    pub folders: Vec<Folder>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FolderDecryptListResponse {
    pub folders: Vec<FolderView>,
}
