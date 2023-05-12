use std::collections::HashMap;

use chrono::DateTime;
use chrono::Utc;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::client::encryption_settings::EncryptionSettings;
use crate::crypto::Decryptable;
use crate::crypto::Encryptable;
use crate::error::Result;

use super::domain;

#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AccountDataView {
    pub profile: Option<domain::Profile>,
    pub data: DataView,

    pub settings: domain::Settings,
    pub auth: domain::Auth,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DataView {
    pub ciphers: HashMap<Uuid, CipherView>,
    pub folders: HashMap<Uuid, FolderView>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CipherView {
    pub id: Uuid,
    pub organization_id: Option<Uuid>,
    pub folder_id: Option<Uuid>,

    pub name: String,

    pub creation_date: DateTime<Utc>,
    pub deleted_date: Option<DateTime<Utc>>,
    pub revision_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct FolderView {
    pub id: Uuid,
    pub name: String,

    pub revision_date: DateTime<Utc>,
}

impl Decryptable<AccountDataView> for domain::AccountData {
    fn decrypt(self, enc: &EncryptionSettings, _: &Option<Uuid>) -> Result<AccountDataView> {
        Ok(AccountDataView {
            profile: self.profile,
            data: DataView {
                ciphers: self.data.ciphers.decrypt(enc, &None)?,
                folders: self.data.folders.decrypt(enc, &None)?,
            },
            settings: self.settings,
            auth: self.auth,
        })
    }
}

impl Decryptable<CipherView> for domain::Cipher {
    fn decrypt(self, enc: &EncryptionSettings, _: &Option<Uuid>) -> Result<CipherView> {
        Ok(CipherView {
            id: self.id,
            organization_id: self.organization_id,
            folder_id: self.folder_id,
            name: self.name.decrypt(enc, &self.organization_id)?,
            creation_date: self.creation_date,
            deleted_date: self.deleted_date,
            revision_date: self.revision_date,
        })
    }
}

impl Encryptable<domain::Cipher> for CipherView {
    fn encrypt(self, enc: &EncryptionSettings, _: &Option<Uuid>) -> Result<domain::Cipher> {
        Ok(domain::Cipher {
            id: self.id,
            organization_id: self.organization_id,
            folder_id: self.folder_id,
            name: self.name.encrypt(enc, &self.organization_id)?,
            creation_date: self.creation_date,
            deleted_date: self.deleted_date,
            revision_date: self.revision_date,
        })
    }
}

impl Decryptable<FolderView> for domain::Folder {
    fn decrypt(self, enc: &EncryptionSettings, _: &Option<Uuid>) -> Result<FolderView> {
        Ok(FolderView {
            id: self.id,
            name: self.name.decrypt(enc, &None)?,
            revision_date: self.revision_date,
        })
    }
}

impl Encryptable<domain::Folder> for FolderView {
    fn encrypt(self, enc: &EncryptionSettings, _: &Option<Uuid>) -> Result<domain::Folder> {
        Ok(domain::Folder {
            id: self.id,
            name: self.name.encrypt(enc, &None)?,
            revision_date: self.revision_date,
        })
    }
}
