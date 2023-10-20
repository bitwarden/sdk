use bitwarden_api_api::models::CollectionDetailsResponseModel;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    client::encryption_settings::EncryptionSettings,
    crypto::{Decryptable, EncString},
    error::{Error, Result},
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

impl Decryptable<CollectionView> for Collection {
    fn decrypt(&self, enc: &EncryptionSettings, _: &Option<Uuid>) -> Result<CollectionView> {
        let org_id = Some(self.organization_id);

        Ok(CollectionView {
            id: self.id,
            organization_id: self.organization_id,

            name: self.name.decrypt(enc, &org_id)?,

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
            id: collection.id.ok_or(Error::MissingFields)?,
            organization_id: collection.organization_id.ok_or(Error::MissingFields)?,
            name: collection.name.ok_or(Error::MissingFields)?.parse()?,
            external_id: collection.external_id,
            hide_passwords: collection.hide_passwords.unwrap_or(false),
            read_only: collection.read_only.unwrap_or(false),
        })
    }
}
