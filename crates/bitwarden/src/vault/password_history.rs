use bitwarden_api_api::models::CipherPasswordHistoryModel;
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    client::encryption_settings::EncryptionSettings,
    crypto::{purpose, EncString, KeyDecryptable, KeyEncryptable, LocateKey, SymmetricCryptoKey},
    error::{Error, Result},
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

impl LocateKey<purpose::UserEncryption> for PasswordHistoryView {
    fn locate_key<'a>(
        &self,
        enc: &'a EncryptionSettings,
    ) -> Option<&'a SymmetricCryptoKey<purpose::UserEncryption>> {
        enc.get_user_key()
    }
}
// PasswordHistory can be both part of a cipher, or part of the user's
// general password history, so we need to support both encryption purposes
// TODO: Might want to introduce a trait to handle this
impl
    KeyEncryptable<
        SymmetricCryptoKey<purpose::UserEncryption>,
        purpose::UserEncryption,
        PasswordHistory,
    > for PasswordHistoryView
{
    fn encrypt_with_key(
        self,
        key: &SymmetricCryptoKey<purpose::UserEncryption>,
    ) -> Result<PasswordHistory> {
        Ok(PasswordHistory {
            password: self.password.encrypt_with_key(key)?,
            last_used_date: self.last_used_date,
        })
    }
}
impl
    KeyEncryptable<
        SymmetricCryptoKey<purpose::CipherEncryption>,
        purpose::CipherEncryption,
        PasswordHistory,
    > for PasswordHistoryView
{
    fn encrypt_with_key(
        self,
        key: &SymmetricCryptoKey<purpose::CipherEncryption>,
    ) -> Result<PasswordHistory> {
        Ok(PasswordHistory {
            password: self.password.encrypt_with_key(key)?,
            last_used_date: self.last_used_date,
        })
    }
}

impl LocateKey<purpose::UserEncryption> for PasswordHistory {
    fn locate_key<'a>(
        &self,
        enc: &'a EncryptionSettings,
    ) -> Option<&'a SymmetricCryptoKey<purpose::UserEncryption>> {
        enc.get_user_key()
    }
}
impl
    KeyDecryptable<
        SymmetricCryptoKey<purpose::UserEncryption>,
        purpose::UserEncryption,
        PasswordHistoryView,
    > for PasswordHistory
{
    fn decrypt_with_key(
        &self,
        key: &SymmetricCryptoKey<purpose::UserEncryption>,
    ) -> Result<PasswordHistoryView> {
        Ok(PasswordHistoryView {
            password: self.password.decrypt_with_key(key)?,
            last_used_date: self.last_used_date,
        })
    }
}
impl
    KeyDecryptable<
        SymmetricCryptoKey<purpose::CipherEncryption>,
        purpose::CipherEncryption,
        PasswordHistoryView,
    > for PasswordHistory
{
    fn decrypt_with_key(
        &self,
        key: &SymmetricCryptoKey<purpose::CipherEncryption>,
    ) -> Result<PasswordHistoryView> {
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
