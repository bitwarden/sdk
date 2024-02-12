use bitwarden_api_api::models::CipherPasswordHistoryModel;
use bitwarden_crypto::{
    CryptoError, EncString, KeyDecryptable, KeyEncryptable, LocateKey, SymmetricCryptoKey,
};
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

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
impl KeyEncryptable<SymmetricCryptoKey, PasswordHistory> for PasswordHistoryView {
    fn encrypt_with_key(self, key: &SymmetricCryptoKey) -> Result<PasswordHistory, CryptoError> {
        Ok(PasswordHistory {
            password: self.password.encrypt_with_key(key)?,
            last_used_date: self.last_used_date,
        })
    }
}

impl LocateKey for PasswordHistory {}
impl KeyDecryptable<SymmetricCryptoKey, PasswordHistoryView> for PasswordHistory {
    fn decrypt_with_key(
        &self,
        key: &SymmetricCryptoKey,
    ) -> Result<PasswordHistoryView, CryptoError> {
        Ok(PasswordHistoryView {
            password: self.password.decrypt_with_key(key).ok().unwrap_or_default(),
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
