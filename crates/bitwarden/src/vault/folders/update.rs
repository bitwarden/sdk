use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::FolderToSave;
use crate::{
    client::encryption_settings::EncryptionSettings,
    crypto::Encryptable,
    error::{Error, Result},
    Client,
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FolderUpdateRequest {
    /// ID of the folder to update
    pub id: Uuid,

    /// Encrypted folder name
    pub name: String,
}

impl Encryptable<FolderToSave> for FolderUpdateRequest {
    fn encrypt(self, enc: &EncryptionSettings, _: &Option<Uuid>) -> Result<FolderToSave> {
        Ok(FolderToSave {
            id: Some(self.id),
            name: enc.encrypt(&self.name.as_bytes(), &None)?,
        })
    }
}

pub(crate) async fn update_folder(client: &mut Client, input: FolderUpdateRequest) -> Result<()> {
    let enc = client
        .get_encryption_settings()
        .as_ref()
        .ok_or(Error::VaultLocked)?;

    input.encrypt(enc, &None)?.save_to_server(client).await?;
    Ok(())
}
