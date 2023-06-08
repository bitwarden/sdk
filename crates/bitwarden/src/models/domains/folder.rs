use std::str::FromStr;
use bitwarden_api_api::models::FolderRequestModel;
use uuid::Uuid;

use crate::{crypto::CipherString, error::{Result, Error}, Client};

struct Folder {
    pub id: Uuid,
    pub name: CipherString,
    pub revisionDate: String
}

pub struct FolderFromDisk(Folder);

impl FolderFromDisk {
    pub fn id(&self) -> &Uuid {
        &self.0.id
    }

    pub fn name(&self) -> &CipherString {
        &self.0.name
    }

    pub fn revision_date(&self) -> &String {
        &self.0.revisionDate
    }
}

pub struct FolderToSave {
    pub id: Option<Uuid>,
    pub name: CipherString,
    pub revision_date: String
}

impl FolderToSave {
    pub async fn save(self, client: &mut Client) -> Result<FolderFromDisk> {
        Ok(self
            .save_to_server(client).await?
            .save(client).await?)
    }

    async fn save_to_server(self, client: &mut Client) -> Result<FolderToStore> {
        let config = client.get_api_configurations().await;

        let request = Some(FolderRequestModel::new(
            self.name.to_string(),
        ));

        let res = match self.id {
            Some(id) => {
                bitwarden_api_api::apis::folders_api::folders_id_put(
                    &config.api,
                    &id.to_string(),
                    request
                ).await?
            },
            None => {
                bitwarden_api_api::apis::folders_api::folders_post(
                    &config.api,
                    request
                ).await?
            }
        };

        let folder = Folder {
            id: res.id.ok_or(Error::MissingFields)?,
            name: CipherString::from_str(&res.name.ok_or(Error::MissingFields)?)?,
            revisionDate: res.revision_date.ok_or(Error::MissingFields)?,
        };

        Ok(FolderToStore(folder))
    }
}

struct FolderToStore(Folder);

impl FolderToStore {
    pub async fn save(self, client: &mut Client) -> Result<FolderFromDisk> {
        // TODO: save to disk

        Ok(FolderFromDisk(self.0))
    }
}
