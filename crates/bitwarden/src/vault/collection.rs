use std::str::FromStr;

use bitwarden_api_api::models::CollectionDetailsResponseModel;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    client::encryption_settings::EncryptionSettings,
    crypto::{Decryptable, EncString},
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

impl From<CollectionDetailsResponseModel> for Collection {
    fn from(collection: CollectionDetailsResponseModel) -> Self {
        Collection {
            id: collection.id.unwrap(),
            organization_id: collection.organization_id.unwrap(),
            name: EncString::from_str(&collection.name.unwrap()).unwrap(),
            external_id: collection.external_id,
            hide_passwords: collection.hide_passwords.unwrap_or(false),
            read_only: collection.read_only.unwrap_or(false),
        }
    }
}
