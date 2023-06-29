use bitwarden_api_api::models::FolderRequestModel;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    crypto::Encryptable,
    error::{Error, Result},
    state::{domain::Folder, state_service::FOLDERS_SERVICE},
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

pub(crate) async fn update_folder(client: &mut Client, input: FolderUpdateRequest) -> Result<()> {
    let enc = client
        .get_encryption_settings()
        .as_ref()
        .ok_or(Error::VaultLocked)?;

    let name = input.name.encrypt(&enc, &None)?;

    let param = Some(FolderRequestModel {
        name: name.to_string(),
    });

    let config = client.get_api_configurations().await;
    let res = bitwarden_api_api::apis::folders_api::folders_id_put(
        &config.api,
        &input.id.to_string(),
        param,
    )
    .await?;

    client
        .get_state_service(FOLDERS_SERVICE)
        .modify(move |folders| {
            let id = res.id.unwrap();
            folders.insert(
                id,
                Folder {
                    id,
                    name,
                    revision_date: res
                        .revision_date
                        .unwrap()
                        .parse()
                        .map_err(|_| Error::InvalidResponse)?,
                },
            );
            Ok(())
        })
        .await?;
    Ok(())
}
