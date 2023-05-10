use bitwarden_api_api::models::FolderRequestModel;

use crate::{
    client::Client,
    crypto::Encryptable,
    error::{Error, Result},
    sdk::{
        model::{domain::Folder, state_service::FOLDERS_SERVICE},
        request::folders_request::{FolderCreateRequest, FolderDeleteRequest, FolderUpdateRequest},
    },
};

pub(crate) async fn create_folder(client: &mut Client, input: FolderCreateRequest) -> Result<()> {
    let enc = client
        .get_encryption_settings()
        .as_ref()
        .ok_or(Error::VaultLocked)?;

    let name = input.name.encrypt(&enc, &None)?;

    let param = Some(FolderRequestModel {
        name: name.to_string(),
    });

    let config = client.get_api_configurations().await;
    let res = bitwarden_api_api::apis::folders_api::folders_post(&config.api, param).await?;

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

pub(crate) async fn delete_folder(client: &mut Client, input: FolderDeleteRequest) -> Result<()> {
    let config: &crate::client::ApiConfigurations = client.get_api_configurations().await;
    bitwarden_api_api::apis::folders_api::folders_id_delete(&config.api, &input.id.to_string())
        .await?;

    client
        .get_state_service(FOLDERS_SERVICE)
        .modify(move |folders| {
            folders.remove(&input.id);
            Ok(())
        })
        .await?;
    Ok(())
}
