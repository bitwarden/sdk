use std::collections::HashMap;

use chrono::DateTime;
use chrono::Utc;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::client::encryption_settings::EncryptionSettings;
use crate::{crypto::CipherString, error::Result};

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

pub(crate) fn convert_domain_to_view(
    data: domain::AccountData,
    enc: &EncryptionSettings,
) -> Result<AccountDataView> {
    Ok(AccountDataView {
        profile: data.profile,
        data: DataView {
            ciphers: data
                .data
                .ciphers
                .into_iter()
                .map(|(id, c)| Ok((id, convert_cipher(c, enc)?)))
                .collect::<Result<_>>()?,
            folders: data
                .data
                .folders
                .into_iter()
                .map(|(id, f)| Ok((id, convert_folder(f, enc)?)))
                .collect::<Result<_>>()?,
        },
        settings: data.settings,
        auth: data.auth,
    })
}

fn convert_cipher(c: domain::Cipher, enc: &EncryptionSettings) -> Result<CipherView> {
    Ok(CipherView {
        id: c.id,
        organization_id: c.organization_id,
        folder_id: c.folder_id,
        name: enc.decrypt(&c.name, c.organization_id)?,
        creation_date: c.creation_date,
        deleted_date: c.deleted_date,
        revision_date: c.revision_date,
    })
}
fn convert_folder(f: domain::Folder, enc: &EncryptionSettings) -> Result<FolderView> {
    Ok(FolderView {
        id: f.id,
        name: enc.decrypt(&f.name, None)?,
        revision_date: f.revision_date,
    })
}
