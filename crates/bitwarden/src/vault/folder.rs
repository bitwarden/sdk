use bitwarden_api_api::models::FolderResponseModel;
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    client::encryption_settings::EncryptionSettings,
    crypto::{Decryptable, EncString, Encryptable},
    error::{Error, Result},
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct Folder {
    id: Uuid,
    name: EncString,
    revision_date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct FolderView {
    id: Uuid,
    name: String,
    revision_date: DateTime<Utc>,
}

impl Encryptable<Folder> for FolderView {
    fn encrypt(self, enc: &EncryptionSettings, _org: &Option<Uuid>) -> Result<Folder> {
        Ok(Folder {
            id: self.id,
            name: self.name.encrypt(enc, &None)?,
            revision_date: self.revision_date,
        })
    }
}

impl Decryptable<FolderView> for Folder {
    fn decrypt(&self, enc: &EncryptionSettings, _org: &Option<Uuid>) -> Result<FolderView> {
        Ok(FolderView {
            id: self.id,
            name: self.name.decrypt(enc, &None)?,
            revision_date: self.revision_date,
        })
    }
}

impl TryFrom<FolderResponseModel> for Folder {
    type Error = Error;

    fn try_from(folder: FolderResponseModel) -> Result<Self> {
        Ok(Folder {
            id: folder.id.ok_or(Error::MissingFields)?,
            name: EncString::try_from(folder.name)?.ok_or(Error::MissingFields)?,
            revision_date: folder.revision_date.ok_or(Error::MissingFields)?.parse()?,
        })
    }
}
