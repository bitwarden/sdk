use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    client::encryption_settings::EncryptionSettings,
    crypto::{Decryptable, Encryptable},
    error::{Error, Result},
    state::{domain, state_service::FOLDERS_SERVICE},
    Client,
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FolderCreateRequest {
    /// Encrypted folder name
    pub name: String,
}

pub(crate) async fn list_folders(client: &Client) -> Result<FoldersResponse> {
    let enc = client
        .get_encryption_settings()
        .as_ref()
        .ok_or(Error::VaultLocked)?;

    let folders = client
        .get_state_service(FOLDERS_SERVICE)
        .get()
        .await
        .decrypt(enc, &None)?;

    Ok(FoldersResponse {
        data: folders.into_iter().map(|f| f.1).collect(),
    })
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FoldersResponse {
    pub data: Vec<FolderView>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct FolderView {
    pub id: Uuid,
    pub name: String,

    pub revision_date: DateTime<Utc>,
}

impl Decryptable<FolderView> for domain::Folder {
    fn decrypt(self, enc: &EncryptionSettings, _: &Option<Uuid>) -> Result<FolderView> {
        Ok(FolderView {
            id: self.id,
            name: self.name.decrypt(enc, &None)?,
            revision_date: self.revision_date,
        })
    }
}

impl Encryptable<domain::Folder> for FolderView {
    fn encrypt(self, enc: &EncryptionSettings, _: &Option<Uuid>) -> Result<domain::Folder> {
        Ok(domain::Folder {
            id: self.id,
            name: self.name.encrypt(enc, &None)?,
            revision_date: self.revision_date,
        })
    }
}
