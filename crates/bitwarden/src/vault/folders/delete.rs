use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{error::Result, Client};

use super::folder::FolderToDelete;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FolderDeleteRequest {
    /// ID of the folder to delete
    pub id: Uuid,
}

impl From<FolderDeleteRequest> for FolderToDelete {
    fn from(input: FolderDeleteRequest) -> Self {
        Self {
            id: input.id,
        }
    }
}

pub(crate) async fn delete_folder(client: &mut Client, input: FolderDeleteRequest) -> Result<()> {
    let input: FolderToDelete = input.into();
    Ok(input.delete_from_server(client).await?)
}
