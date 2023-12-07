use bitwarden_crypto::{EncString, KeyDecryptable, KeyEncryptable, LocateKey, SymmetricCryptoKey};
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
