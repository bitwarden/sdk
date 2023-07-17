use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{FolderFromDisk, FolderView};
use crate::{
    crypto::Decryptable,
    error::{Error, Result},
    Client,
};

pub(crate) async fn list_folders(client: &Client) -> Result<FoldersResponse> {
    let enc = client
        .get_encryption_settings()
        .as_ref()
        .ok_or(Error::VaultLocked)?;

    let folders = FolderFromDisk::list(client)
        .await
        .into_iter()
        .map(|f| f.decrypt(enc, &None))
        .collect::<Result<Vec<FolderView>>>()?;

    Ok(FoldersResponse { data: folders })
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FoldersResponse {
    pub data: Vec<FolderView>,
}
