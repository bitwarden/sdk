use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    client::encryption_settings::EncryptionSettings,
    crypto::{Decryptable, Encryptable},
    error::Result,
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct LocalData {
    last_used_date: Option<u32>,
    last_launched: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct LocalDataView {
    last_used_date: Option<u32>,
    last_launched: Option<u32>,
}

impl Encryptable<LocalData> for LocalDataView {
    fn encrypt(self, _enc: &EncryptionSettings, _org_id: &Option<Uuid>) -> Result<LocalData> {
        Ok(LocalData {
            last_used_date: self.last_used_date,
            last_launched: self.last_launched,
        })
    }
}

impl Decryptable<LocalDataView> for LocalData {
    fn decrypt(&self, _enc: &EncryptionSettings, _org_id: &Option<Uuid>) -> Result<LocalDataView> {
        Ok(LocalDataView {
            last_used_date: self.last_used_date,
            last_launched: self.last_launched,
        })
    }
}
