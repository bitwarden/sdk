use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    client::encryption_settings::EncryptionSettings,
    crypto::{EncString, KeyDecryptable, LocateKey, SymmetricCryptoKey},
    error::Result,
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct Collection {
    id: Uuid,
    organization_id: Uuid,

    name: EncString,

    external_id: Option<String>,
    hide_passwords: bool,
    read_only: bool,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct CollectionView {
    id: Uuid,
    organization_id: Uuid,

    name: String,

    external_id: Option<String>,
    hide_passwords: bool,
    read_only: bool,
}

impl LocateKey for Collection {
    fn locate_key<'a>(
        &self,
        enc: &'a EncryptionSettings,
        _: &Option<Uuid>,
    ) -> Option<&'a SymmetricCryptoKey> {
        enc.get_key(&Some(self.organization_id))
    }
}
impl KeyDecryptable<CollectionView> for Collection {
    fn decrypt_with_key(&self, key: &SymmetricCryptoKey) -> Result<CollectionView> {
        Ok(CollectionView {
            id: self.id,
            organization_id: self.organization_id,

            name: self.name.decrypt_with_key(key)?,

            external_id: self.external_id.clone(),
            hide_passwords: self.hide_passwords,
            read_only: self.read_only,
        })
    }
}
