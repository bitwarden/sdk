use bitwarden_api_api::models::CollectionDetailsResponseModel;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    client::encryption_settings::EncryptionSettings,
    crypto::{purpose, EncString, KeyDecryptable, LocateKey, SymmetricCryptoKey},
    error::{Error, Result},
};

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

impl LocateKey<purpose::OrgEncryption> for Collection {
    fn locate_key<'a>(
        &self,
        enc: &'a EncryptionSettings,
    ) -> Option<&'a SymmetricCryptoKey<purpose::OrgEncryption>> {
        enc.get_org_key(self.organization_id)
    }
}
impl
    KeyDecryptable<
        SymmetricCryptoKey<purpose::OrgEncryption>,
        purpose::OrgEncryption,
        CollectionView,
    > for Collection
{
    fn decrypt_with_key(
        &self,
        key: &SymmetricCryptoKey<purpose::OrgEncryption>,
    ) -> Result<CollectionView> {
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
