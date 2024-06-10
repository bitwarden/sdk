use bitwarden_api_api::models::CollectionDetailsResponseModel;
use crate::require;
use bitwarden_crypto::{
    CryptoError, EncString, KeyContainer, KeyDecryptable, LocateKey, SymmetricCryptoKey,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{Error, Result};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Collection {
    pub id: Option<Uuid>,
    pub organization_id: Uuid,

    pub name: EncString,

    pub external_id: Option<String>,
    pub hide_passwords: bool,
    pub read_only: bool,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct CollectionView {
    pub id: Option<Uuid>,
    pub organization_id: Uuid,

    pub name: String,

    pub external_id: Option<String>,
    pub hide_passwords: bool,
    pub read_only: bool,
}

impl LocateKey for Collection {
    fn locate_key<'a>(
        &self,
        enc: &'a dyn KeyContainer,
        _: &Option<Uuid>,
    ) -> Option<&'a SymmetricCryptoKey> {
        enc.get_key(&Some(self.organization_id))
    }
}
impl KeyDecryptable<SymmetricCryptoKey, CollectionView> for Collection {
    fn decrypt_with_key(&self, key: &SymmetricCryptoKey) -> Result<CollectionView, CryptoError> {
        Ok(CollectionView {
            id: self.id,
            organization_id: self.organization_id,

            name: self.name.decrypt_with_key(key).ok().unwrap_or_default(),

            external_id: self.external_id.clone(),
            hide_passwords: self.hide_passwords,
            read_only: self.read_only,
        })
    }
}

impl TryFrom<CollectionDetailsResponseModel> for Collection {
    type Error = Error;

    fn try_from(collection: CollectionDetailsResponseModel) -> Result<Self> {
        Ok(Collection {
            id: collection.id,
            organization_id: require!(collection.organization_id),
            name: require!(collection.name).parse()?,
            external_id: collection.external_id,
            hide_passwords: collection.hide_passwords.unwrap_or(false),
            read_only: collection.read_only.unwrap_or(false),
        })
    }
}
