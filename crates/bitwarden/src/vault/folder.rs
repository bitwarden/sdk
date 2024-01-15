use bitwarden_api_api::models::FolderResponseModel;
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    client::encryption_settings::EncryptionSettings,
    crypto::{purpose, EncString, KeyDecryptable, KeyEncryptable, LocateKey, SymmetricCryptoKey},
    error::{Error, Result},
};

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

impl LocateKey<purpose::UserEncryption> for FolderView {
    fn locate_key<'a>(
        &self,
        enc: &'a EncryptionSettings,
    ) -> Option<&'a SymmetricCryptoKey<purpose::UserEncryption>> {
        enc.get_user_key()
    }
}
impl KeyEncryptable<SymmetricCryptoKey<purpose::UserEncryption>, purpose::UserEncryption, Folder>
    for FolderView
{
    fn encrypt_with_key(self, key: &SymmetricCryptoKey<purpose::UserEncryption>) -> Result<Folder> {
        Ok(Folder {
            id: self.id,
            name: self.name.encrypt_with_key(key)?,
            revision_date: self.revision_date,
        })
    }
}

impl LocateKey<purpose::UserEncryption> for Folder {
    fn locate_key<'a>(
        &self,
        enc: &'a EncryptionSettings,
    ) -> Option<&'a SymmetricCryptoKey<purpose::UserEncryption>> {
        enc.get_user_key()
    }
}
impl
    KeyDecryptable<SymmetricCryptoKey<purpose::UserEncryption>, purpose::UserEncryption, FolderView>
    for Folder
{
    fn decrypt_with_key(
        &self,
        key: &SymmetricCryptoKey<purpose::UserEncryption>,
    ) -> Result<FolderView> {
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
