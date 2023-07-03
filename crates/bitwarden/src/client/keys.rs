use std::collections::HashMap;

use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use bitwarden_api_api::models::ProfileResponseModel;

use crate::{
    crypto::CipherString,
    error::{Error, Result},
    state::state_service::ServiceDefinition,
    Client,
};

const KEYS_SERVICE: ServiceDefinition<Option<Keys>> = ServiceDefinition::new("keys");

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Keys {
    pub crypto_symmetric_key: CipherString,
    pub organization_keys: HashMap<Uuid, CipherString>,
    pub private_key: CipherString,
}

impl TryFrom<&ProfileResponseModel> for Keys {
    type Error = Error;

    fn try_from(profile: &ProfileResponseModel) -> Result<Self> {
        Ok(Keys {
            crypto_symmetric_key: profile
                .key
                .as_ref()
                .map(|s| s.parse())
                .transpose()?
                .ok_or(Error::MissingFields)?,

            organization_keys: profile
                .organizations
                .as_deref()
                .unwrap_or_default()
                .iter()
                .filter_map(|o| o.id.zip(o.key.as_deref()))
                .map(|(id, key)| CipherString::from_str(key).map(|k| (id, k)))
                .collect::<Result<_>>()?,

            private_key: profile
                .private_key
                .as_ref()
                .map(|s| s.parse())
                .transpose()?
                .ok_or(Error::MissingFields)?,
        })
    }
}

impl Keys {
    pub(crate) async fn get(client: &Client) -> Option<Keys> {
        client.get_state_service(KEYS_SERVICE).get().await
    }
}

pub(crate) async fn store_keys_from_sync(
    profile: &ProfileResponseModel,
    client: &Client,
) -> Result<()> {
    client
        .get_state_service(KEYS_SERVICE)
        .modify(|k| {
            *k = Some(profile.try_into()?);
            Ok(())
        })
        .await
}
