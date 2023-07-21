use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::Folder;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FolderDecryptRequest {
    pub folder: Folder,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FolderDecryptListRequest {
    pub folders: Vec<Folder>,
}
