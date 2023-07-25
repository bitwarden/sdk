use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::vault::FolderView;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FolderDecryptResponse {
    pub folder: FolderView,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FolderDecryptListResponse {
    pub folders: Vec<FolderView>,
}
