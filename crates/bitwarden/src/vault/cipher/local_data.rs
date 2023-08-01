use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{client::encryption_settings::EncryptionSettings, crypto::Encryptable, error::Result};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LocalData {
    last_used_date: Option<usize>,
    last_launched: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LocalDataView {
    last_used_date: Option<usize>,
    last_launched: Option<usize>,
}

impl Encryptable<LocalData> for LocalDataView {
    fn encrypt(self, _enc: &EncryptionSettings, _org_id: &Option<Uuid>) -> Result<LocalData> {
        Ok(LocalData {
            last_used_date: self.last_used_date,
            last_launched: self.last_launched,
        })
    }
}
