use bitwarden_api_api::models::FolderRequestModel;
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
pub struct FolderCreateRequest {
    /// Encrypted folder name
    pub name: String,
}

impl Encryptable<FolderToSave> for FolderCreateRequest {
    fn encrypt(self, enc: &EncryptionSettings, _: &Option<Uuid>) -> Result<FolderToSave> {
        Ok(FolderToSave {
            id: None,
            name: enc.encrypt(&self.name.as_bytes(), &None)?,
        })
    }
}

pub(crate) async fn create_folder(client: &mut Client, input: FolderCreateRequest) -> Result<()> {
    let enc = client
        .get_encryption_settings()
        .as_ref()
        .ok_or(Error::VaultLocked)?;

    input.encrypt(enc, &None)?.save_to_server(client).await?;
    Ok(())
}
