use bitwarden_api_api::models::FolderResponseModel;
use bitwarden_crypto::{EncString, KeyDecryptable, KeyEncryptable, LocateKey, SymmetricCryptoKey};
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{Error, Result};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct Folder {
    id: Option<Uuid>,
    name: EncString,
    revision_date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct FolderView {
    id: Option<Uuid>,
    name: String,
    revision_date: DateTime<Utc>,
}

impl LocateKey for FolderView {}
impl KeyEncryptable<Folder> for FolderView {
    fn encrypt_with_key(self, key: &SymmetricCryptoKey) -> bitwarden_crypto::Result<Folder> {
        Ok(Folder {
            id: self.id,
            name: self.name.encrypt_with_key(key)?,
            revision_date: self.revision_date,
        })
    }
}

impl LocateKey for Folder {}
impl KeyDecryptable<FolderView> for Folder {
    fn decrypt_with_key(&self, key: &SymmetricCryptoKey) -> bitwarden_crypto::Result<FolderView> {
        Ok(FolderView {
            id: self.id,
            name: self.name.decrypt_with_key(key)?,
            revision_date: self.revision_date,
        })
    }
}

impl TryFrom<FolderResponseModel> for Folder {
    type Error = Error;

    fn try_from(folder: FolderResponseModel) -> Result<Self> {
        Ok(Folder {
            id: folder.id,
            name: EncString::try_from_optional(folder.name)?.ok_or(Error::MissingFields)?,
            revision_date: folder.revision_date.ok_or(Error::MissingFields)?.parse()?,
        })
    }
}
