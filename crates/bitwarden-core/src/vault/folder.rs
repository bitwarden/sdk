use crate::require;
use bitwarden_api_api::models::FolderResponseModel;
use bitwarden_crypto::{
    CryptoError, EncString, KeyDecryptable, KeyEncryptable, SymmetricCryptoKey,
};
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{Error, Result};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Folder {
    id: Option<Uuid>,
    name: EncString,
    revision_date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct FolderView {
    pub id: Option<Uuid>,
    pub name: String,
    pub revision_date: DateTime<Utc>,
}

impl KeyEncryptable<SymmetricCryptoKey, Folder> for FolderView {
    fn encrypt_with_key(self, key: &SymmetricCryptoKey) -> Result<Folder, CryptoError> {
        Ok(Folder {
            id: self.id,
            name: self.name.encrypt_with_key(key)?,
            revision_date: self.revision_date,
        })
    }
}

impl KeyDecryptable<SymmetricCryptoKey, FolderView> for Folder {
    fn decrypt_with_key(&self, key: &SymmetricCryptoKey) -> Result<FolderView, CryptoError> {
        Ok(FolderView {
            id: self.id,
            name: self.name.decrypt_with_key(key).ok().unwrap_or_default(),
            revision_date: self.revision_date,
        })
    }
}

impl TryFrom<FolderResponseModel> for Folder {
    type Error = Error;

    fn try_from(folder: FolderResponseModel) -> Result<Self> {
        Ok(Folder {
            id: folder.id,
            name: require!(EncString::try_from_optional(folder.name)?),
            revision_date: require!(folder.revision_date).parse()?,
        })
    }
}
