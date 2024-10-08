use bitwarden_api_api::models::CipherPasswordHistoryModel;
use bitwarden_core::key_management::{AsymmetricKeyRef, SymmetricKeyRef};
use bitwarden_crypto::{
    service::CryptoServiceContext, CryptoError, Decryptable, EncString, Encryptable, UsesKey,
};
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::VaultParseError;

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct PasswordHistory {
    password: EncString,
    last_used_date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct PasswordHistoryView {
    password: String,
    last_used_date: DateTime<Utc>,
}

impl UsesKey<SymmetricKeyRef> for PasswordHistory {
    fn uses_key(&self) -> SymmetricKeyRef {
        SymmetricKeyRef::User
    }
}

impl UsesKey<SymmetricKeyRef> for PasswordHistoryView {
    fn uses_key(&self) -> SymmetricKeyRef {
        SymmetricKeyRef::User
    }
}

impl Encryptable<SymmetricKeyRef, AsymmetricKeyRef, SymmetricKeyRef, PasswordHistory>
    for PasswordHistoryView
{
    fn encrypt(
        &self,
        ctx: &mut CryptoServiceContext<SymmetricKeyRef, AsymmetricKeyRef>,
        key: SymmetricKeyRef,
    ) -> Result<PasswordHistory, CryptoError> {
        Ok(PasswordHistory {
            password: self.password.encrypt(ctx, key)?,
            last_used_date: self.last_used_date,
        })
    }
}

impl Decryptable<SymmetricKeyRef, AsymmetricKeyRef, SymmetricKeyRef, PasswordHistoryView>
    for PasswordHistory
{
    fn decrypt(
        &self,
        ctx: &mut CryptoServiceContext<SymmetricKeyRef, AsymmetricKeyRef>,
        key: SymmetricKeyRef,
    ) -> Result<PasswordHistoryView, CryptoError> {
        Ok(PasswordHistoryView {
            password: self.password.decrypt(ctx, key).ok().unwrap_or_default(),
            last_used_date: self.last_used_date,
        })
    }
}

impl TryFrom<CipherPasswordHistoryModel> for PasswordHistory {
    type Error = VaultParseError;

    fn try_from(model: CipherPasswordHistoryModel) -> Result<Self, Self::Error> {
        Ok(Self {
            password: model.password.parse()?,
            last_used_date: model.last_used_date.parse()?,
        })
    }
}
