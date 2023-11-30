use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    crypto::{KeyDecryptable, KeyEncryptable, SymmetricCryptoKey},
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

impl KeyEncryptable<LocalData> for LocalDataView {
    fn encrypt_with_key(self, _key: &SymmetricCryptoKey) -> Result<LocalData> {
        Ok(LocalData {
            last_used_date: self.last_used_date,
            last_launched: self.last_launched,
        })
    }
}

impl KeyDecryptable<LocalDataView> for LocalData {
    fn decrypt_with_key(&self, _key: &SymmetricCryptoKey) -> Result<LocalDataView> {
        Ok(LocalDataView {
            last_used_date: self.last_used_date,
            last_launched: self.last_launched,
        })
    }
}
