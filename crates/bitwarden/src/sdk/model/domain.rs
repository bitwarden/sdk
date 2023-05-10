use std::collections::HashMap;

use std::str::FromStr;

use chrono::DateTime;
use chrono::Utc;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;

use bitwarden_api_api::models::{
    CipherDetailsResponseModel, FolderResponseModel, ProfileResponseModel, SyncResponseModel,
};

use crate::{
    client::{auth_settings::AuthSettings, LoginMethod},
    crypto::CipherString,
    error::{Error, Result},
};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RootData {
    pub global: GlobalData,

    #[serde(flatten)]
    pub accounts: HashMap<Uuid, AccountData>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GlobalData {
    pub state_version: u32,
    pub app_id: Uuid,

    pub active_user_id: Option<Uuid>,
    pub authenticated_accounts: Vec<Uuid>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EnvironmentUrls {
    pub base: Option<String>,
    pub api: Option<String>,
    pub identity: Option<String>,
    pub web_vault: Option<String>,
    pub icons: Option<String>,
    pub notifications: Option<String>,
    pub events: Option<String>,
    pub key_connector: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AccountData {
    pub keys: Option<Keys>,
    pub profile: Option<Profile>,
    pub data: Data,

    pub settings: Settings,
    pub auth: Auth,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Keys {
    pub crypto_symmetric_key: CipherString,
    pub organization_keys: HashMap<Uuid, CipherString>,
    pub private_key: CipherString,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub user_id: Uuid,
    pub name: String,
    pub email: String,
    pub last_sync: DateTime<Utc>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub ciphers: HashMap<Uuid, Cipher>,
    pub folders: HashMap<Uuid, Folder>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub environment_urls: EnvironmentUrls,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Auth {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub token_expiration: Option<chrono::DateTime<Utc>>,
    pub login_method: Option<LoginMethod>,

    pub kdf: Option<AuthSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Cipher {
    pub id: Uuid,
    pub organization_id: Option<Uuid>,
    pub folder_id: Option<Uuid>,

    pub name: CipherString,

    pub creation_date: DateTime<Utc>,
    pub deleted_date: Option<DateTime<Utc>>,
    pub revision_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Folder {
    pub id: Uuid,
    pub name: CipherString,

    pub revision_date: DateTime<Utc>,
}

pub(crate) fn copy_sync_to_domain(sync: SyncResponseModel, state: &mut AccountData) -> Result<()> {
    let profile = sync.profile.as_ref().ok_or(Error::MissingFields)?;
    state.keys = Some(convert_keys(profile)?);
    state.profile = Some(convert_profile(profile)?);
    state.data = convert_data(sync)?;

    Ok(())
}

fn convert_data(sync: SyncResponseModel) -> Result<Data> {
    Ok(Data {
        ciphers: sync
            .ciphers
            .unwrap_or_default()
            .into_iter()
            .map(convert_cipher)
            .collect::<Result<_, _>>()?,
        folders: sync
            .folders
            .unwrap_or_default()
            .into_iter()
            .map(convert_folder)
            .collect::<Result<_, _>>()?,
    })
}

fn convert_cipher(c: CipherDetailsResponseModel) -> Result<(Uuid, Cipher)> {
    Ok((
        c.id.ok_or(Error::MissingFields)?,
        Cipher {
            id: c.id.ok_or(Error::MissingFields)?,
            organization_id: c.organization_id,
            folder_id: c.folder_id,
            name: c
                .name
                .ok_or(Error::MissingFields)?
                .parse()
                .map_err(|_| Error::InvalidResponse)?,
            creation_date: c
                .creation_date
                .ok_or(Error::MissingFields)?
                .parse()
                .map_err(|_| Error::InvalidResponse)?,
            deleted_date: c
                .deleted_date
                .map(|d| d.parse())
                .transpose()
                .map_err(|_| Error::InvalidResponse)?,
            revision_date: c
                .revision_date
                .ok_or(Error::MissingFields)?
                .parse()
                .map_err(|_| Error::InvalidResponse)?,
        },
    ))
}

fn convert_folder(f: FolderResponseModel) -> Result<(Uuid, Folder)> {
    Ok((
        f.id.ok_or(Error::MissingFields)?,
        Folder {
            id: f.id.ok_or(Error::MissingFields)?,
            name: f
                .name
                .ok_or(Error::MissingFields)?
                .parse()
                .map_err(|_| Error::InvalidResponse)?,
            revision_date: f
                .revision_date
                .ok_or(Error::MissingFields)?
                .parse()
                .map_err(|_| Error::InvalidResponse)?,
        },
    ))
}

fn convert_keys(profile: &ProfileResponseModel) -> Result<Keys> {
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

fn convert_profile(profile: &ProfileResponseModel) -> Result<Profile> {
    Ok(Profile {
        user_id: profile.id.ok_or(Error::MissingFields)?,
        name: profile.name.clone().ok_or(Error::MissingFields)?,
        email: profile.email.clone().ok_or(Error::MissingFields)?,
        last_sync: Utc::now(),
    })
}
