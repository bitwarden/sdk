use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::vault::{Folder, FolderView};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct FolderDecryptRequest {
    pub folder: Folder,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct FolderDecryptResponse {
    pub folder: FolderView,
}
