use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{FolderFromDisk, FolderView};
use crate::{
    crypto::Decryptable,
    error::{Error, Result},
    Client,
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FolderGetRequest {
    /// Folder id to get
    pub id: Uuid,
}

pub(crate) async fn get_folder(client: &Client, input: FolderGetRequest) -> Result<FolderResponse> {
    let enc = client
        .get_encryption_settings()
        .as_ref()
        .ok_or(Error::VaultLocked)?;

    let folder = FolderFromDisk::get(input.id, client)
        .await
        .map(|f| f.decrypt(enc, &None)).transpose()?;

    Ok(FolderResponse { data: folder })
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FolderResponse {
    pub data: Option<FolderView>,
}
