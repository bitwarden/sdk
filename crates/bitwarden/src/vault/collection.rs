use bitwarden_api_api::models::CollectionDetailsResponseModel;
use bitwarden_crypto::{
    CryptoError, EncString, KeyContainer, KeyDecryptable, LocateKey, SymmetricCryptoKey,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{Error, Result};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct Collection {
    id: Option<Uuid>,
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
    id: Option<Uuid>,
    organization_id: Uuid,

    name: String,

    external_id: Option<String>,
    hide_passwords: bool,
    read_only: bool,
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
            organization_id: collection.organization_id.ok_or(Error::MissingFields)?,
            name: collection.name.ok_or(Error::MissingFields)?.parse()?,
            external_id: collection.external_id,
            hide_passwords: collection.hide_passwords.unwrap_or(false),
            read_only: collection.read_only.unwrap_or(false),
        })
    }
}
