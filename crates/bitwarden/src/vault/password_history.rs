use bitwarden_api_api::models::CipherPasswordHistoryModel;
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    crypto::{EncString, KeyDecryptable, KeyEncryptable, LocateKey, SymmetricCryptoKey},
    error::{Error, Result},
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct PasswordHistory {
    pub password: EncString,
    pub last_used_date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct PasswordHistoryView {
    pub password: String,
    pub last_used_date: DateTime<Utc>,
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

impl TryFrom<CipherPasswordHistoryModel> for PasswordHistory {
    type Error = Error;

    fn try_from(model: CipherPasswordHistoryModel) -> Result<Self> {
        Ok(Self {
            password: model.password.parse()?,
            last_used_date: model.last_used_date.parse()?,
        })
    }
}
