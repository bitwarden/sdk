use bitwarden_core::key_management::{AsymmetricKeyRef, SymmetricKeyRef};
use bitwarden_crypto::{service::CryptoServiceContext, CryptoError, Decryptable, Encryptable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct LocalData {
    last_used_date: Option<u32>,
    last_launched: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct LocalDataView {
    last_used_date: Option<u32>,
    last_launched: Option<u32>,
}

impl Encryptable<SymmetricKeyRef, AsymmetricKeyRef, SymmetricKeyRef, LocalData> for LocalDataView {
    fn encrypt(
        &self,
        _ctx: &mut CryptoServiceContext<SymmetricKeyRef, AsymmetricKeyRef>,
        _key: SymmetricKeyRef,
    ) -> Result<LocalData, CryptoError> {
        Ok(LocalData {
            last_used_date: self.last_used_date,
            last_launched: self.last_launched,
        })
    }
}

impl Decryptable<SymmetricKeyRef, AsymmetricKeyRef, SymmetricKeyRef, LocalDataView> for LocalData {
    fn decrypt(
        &self,
        _ctx: &mut CryptoServiceContext<SymmetricKeyRef, AsymmetricKeyRef>,
        _key: SymmetricKeyRef,
    ) -> Result<LocalDataView, CryptoError> {
        Ok(LocalDataView {
            last_used_date: self.last_used_date,
            last_launched: self.last_launched,
        })
    }
}
