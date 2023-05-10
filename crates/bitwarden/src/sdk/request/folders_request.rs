use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FolderCreateRequest {
    /// Encrypted folder name
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FolderUpdateRequest {
    /// ID of the folder to update
    pub id: Uuid,

    /// Encrypted folder name
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FolderDeleteRequest {
    /// ID of the folder to delete
    pub id: Uuid,
}
