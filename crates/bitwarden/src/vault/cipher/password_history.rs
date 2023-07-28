use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    client::encryption_settings::EncryptionSettings,
    crypto::{CipherString, Decryptable, Encryptable},
    error::Result,
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PasswordHistory {
    password: CipherString,
    last_used_date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PasswordHistoryView {
    password: String,
    last_used_date: DateTime<Utc>,
}

impl Encryptable<PasswordHistory> for PasswordHistoryView {
    fn encrypt(self, enc: &EncryptionSettings, org_id: &Option<Uuid>) -> Result<PasswordHistory> {
        Ok(PasswordHistory {
            password: self.password.encrypt(enc, org_id)?,
            last_used_date: self.last_used_date,
        })
    }
}

impl Decryptable<PasswordHistoryView> for PasswordHistory {
    fn decrypt(
        &self,
        enc: &EncryptionSettings,
        org_id: &Option<Uuid>,
    ) -> Result<PasswordHistoryView> {
        Ok(PasswordHistoryView {
            password: self.password.decrypt(enc, org_id)?,
            last_used_date: self.last_used_date,
        })
    }
}
