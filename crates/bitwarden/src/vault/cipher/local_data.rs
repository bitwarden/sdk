use bitwarden_crypto::{CryptoError, KeyDecryptable, KeyEncryptable, SymmetricCryptoKey};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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

impl KeyEncryptable<SymmetricCryptoKey, LocalData> for LocalDataView {
    fn encrypt_with_key(self, _key: &SymmetricCryptoKey) -> Result<LocalData, CryptoError> {
        Ok(LocalData {
            last_used_date: self.last_used_date,
            last_launched: self.last_launched,
        })
    }
}

impl KeyDecryptable<SymmetricCryptoKey, LocalDataView> for LocalData {
    fn decrypt_with_key(&self, _key: &SymmetricCryptoKey) -> Result<LocalDataView, CryptoError> {
        Ok(LocalDataView {
            last_used_date: self.last_used_date,
            last_launched: self.last_launched,
        })
    }
}
