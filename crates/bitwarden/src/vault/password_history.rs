use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    crypto::{EncString, KeyDecryptable, KeyEncryptable, LocateKey, SymmetricCryptoKey},
    error::Result,
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct PasswordHistory {
    password: EncString,
    last_used_date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct PasswordHistoryView {
    password: String,
    last_used_date: DateTime<Utc>,
}

impl LocateKey for PasswordHistoryView {}
impl KeyEncryptable<PasswordHistory> for PasswordHistoryView {
    fn encrypt_with_key(self, key: &SymmetricCryptoKey) -> Result<PasswordHistory> {
        Ok(PasswordHistory {
            password: self.password.encrypt_with_key(key)?,
            last_used_date: self.last_used_date,
        })
    }
}

impl LocateKey for PasswordHistory {}
impl KeyDecryptable<PasswordHistoryView> for PasswordHistory {
    fn decrypt_with_key(&self, key: &SymmetricCryptoKey) -> Result<PasswordHistoryView> {
        Ok(PasswordHistoryView {
            password: self.password.decrypt_with_key(key)?,
            last_used_date: self.last_used_date,
        })
    }
}
