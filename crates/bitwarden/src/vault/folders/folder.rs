use std::collections::HashMap;

use bitwarden_api_api::models::{FolderRequestModel, FolderResponseModel};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    crypto::CipherString,
    error::{Error, Result},
    state::{state::State, state_service::ServiceDefinition},
    Client,
};

/// Storage service for folders. Applies the `folders` namespace to stored items.
/// Private to enable tighter control over Folder state
const FOLDERS_SERVICE: ServiceDefinition<HashMap<Uuid, Folder>> = ServiceDefinition::new("folders");

/// Folder is the root struct representing a Folder as it is stored on disk and communicated from the server
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Folder {
    pub id: Uuid,
    pub name: CipherString,
    pub revision_date: DateTime<Utc>,
}

impl TryFrom<FolderResponseModel> for Folder {
    type Error = Error;

    fn try_from(value: FolderResponseModel) -> Result<Self> {
        Ok(Folder {
            id: value.id.ok_or(Error::MissingFields)?,
            name: value
                .name
                .ok_or(Error::MissingFields)?
                .parse()
                .map_err(|_| Error::InvalidResponse)?,
            revision_date: value
                .revision_date
                .ok_or(Error::MissingFields)?
                .parse()
                .map_err(|_| Error::InvalidResponse)?,
        })
    }
}

/// Struct representing a Folder that has been loaded from disk
#[derive(Debug, Clone)]
pub struct FolderFromDisk(Folder);

impl FolderFromDisk {
    pub fn id(&self) -> &Uuid {
        &self.0.id
    }

    pub fn name(&self) -> &CipherString {
        &self.0.name
    }

    pub fn revision_date(&self) -> &DateTime<Utc> {
        &self.0.revision_date
    }

    pub async fn list(client: &Client) -> Vec<FolderFromDisk> {
        client
            .get_state_service(FOLDERS_SERVICE)
            .get()
            .await
            .into_iter()
            .map(|f| FolderFromDisk(f.1))
            .collect()
    }

    pub async fn get(id: Uuid, client: &Client) -> Option<FolderFromDisk> {
        client
            .get_state_service(FOLDERS_SERVICE)
            .get()
            .await
            .into_iter()
            .find(|f| f.0 == id)
            .map(|f| FolderFromDisk(f.1))
    }
}

/// Struct representing a folder view that has been encrypted and must be stored on the server and on disk
#[derive(Debug, Clone)]
pub struct FolderToSave {
    pub id: Option<Uuid>,
    pub name: CipherString,
}

impl FolderToSave {
    pub async fn save_to_server(self, client: &mut Client) -> Result<FolderFromDisk> {
        let config = client.get_api_configurations().await;

        let request = Some(FolderRequestModel::new(self.name.to_string()));

        let res = match self.id {
            Some(id) => {
                bitwarden_api_api::apis::folders_api::folders_id_put(
                    &config.api,
                    &id.to_string(),
                    request,
                )
                .await?
            }
            None => {
                bitwarden_api_api::apis::folders_api::folders_post(&config.api, request).await?
            }
        };

        Ok(store_folder_response(res, client).await?)
    }
}

#[derive(Debug, Clone)]
pub struct FolderToDelete {
    pub id: Uuid,
}

impl FolderToDelete {
    pub async fn delete_from_server(self, client: &mut Client) -> Result<()> {
        let config = client.get_api_configurations().await;

        bitwarden_api_api::apis::folders_api::folders_id_delete(&config.api, &self.id.to_string())
            .await?;

        client
            .get_state_service(FOLDERS_SERVICE)
            .modify(|folders| {
                folders.remove(&self.id);
                Ok(())
            })
            .await?;

        Ok(remove_folder(self.id, client).await?)
    }
}

/// Processes a FolderResponseModel and stores it on disk
async fn store_folder_response(
    response: FolderResponseModel,
    client: &Client,
) -> Result<FolderFromDisk> {
    let folder = response.try_into()?;

    // TODO store folder on disk

    Ok(FolderFromDisk(folder))
}

/// Removes a folder from disk
async fn remove_folder(id: Uuid, client: &Client) -> Result<()> {
    client
        .get_state_service(FOLDERS_SERVICE)
        .modify(|folders| {
            folders.remove(&id);
            Ok(())
        })
        .await?;

    Ok(())
}

/// Clobbers folders list with given folder response values
/// used during sync events
pub async fn store_folders_from_sync(
    folders: Vec<FolderResponseModel>,
    state: &State,
) -> Result<()> {
    let folders: HashMap<Uuid, Folder> = prep_folders_for_storage(folders)?;

    state
        .get_state_service(FOLDERS_SERVICE)
        .modify(|f| {
            *f = folders;
            Ok(())
        })
        .await?;
    Ok(())
}

/// Processes a list of FolderResponseModels into [Folders](Folder).
/// this is used during [sync](crate::commands::sync::sync) to update the full list of folders for an account.
fn prep_folders_for_storage(folders: Vec<FolderResponseModel>) -> Result<HashMap<Uuid, Folder>> {
    folders
        .into_iter()
        .map(|r| {
            let folder: Folder = r.try_into()?;
            Ok((folder.id, folder))
        })
        .collect()
}
