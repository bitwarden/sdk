use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::folder::FolderFromDisk;
use crate::{
    client::encryption_settings::EncryptionSettings,
    crypto::Decryptable,
    error::{Error, Result},
    Client,
};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct FolderView {
    id: Uuid,
    name: String,
    revision_date: DateTime<Utc>,
}

impl Decryptable<FolderView> for FolderFromDisk {
    fn decrypt(&self, enc: &EncryptionSettings, _: &Option<Uuid>) -> Result<FolderView> {
        Ok(FolderView {
            id: self.id().to_owned(),
            name: enc.decrypt(&self.name(), &None)?,
            revision_date: self.revision_date().clone(),
        })
    }
}
