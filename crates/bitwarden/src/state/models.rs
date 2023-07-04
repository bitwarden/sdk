use std::collections::HashMap;

use std::str::FromStr;

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use bitwarden_api_api::models::ProfileResponseModel;

use crate::{
    client::{auth_settings::AuthSettings, LoginMethod},
    crypto::CipherString,
    error::{Error, Result},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Keys {
    pub crypto_symmetric_key: CipherString,
    pub organization_keys: HashMap<Uuid, CipherString>,
    pub private_key: CipherString,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub user_id: Uuid,
    pub name: String,
    pub email: String,
    pub last_sync: DateTime<Utc>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Auth {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub token_expiration: Option<chrono::DateTime<Utc>>,
    pub login_method: Option<LoginMethod>,

    pub kdf: Option<AuthSettings>,
}

impl TryFrom<&ProfileResponseModel> for Profile {
    type Error = Error;

    fn try_from(value: &ProfileResponseModel) -> Result<Self> {
        Ok(Profile {
            user_id: value.id.ok_or(Error::MissingFields)?,
            name: value.name.clone().ok_or(Error::MissingFields)?,
            email: value.email.clone().ok_or(Error::MissingFields)?,
            last_sync: Utc::now(),
        })
    }
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
