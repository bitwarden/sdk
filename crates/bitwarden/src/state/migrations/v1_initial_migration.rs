use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Map;
use uuid::Uuid;

use super::Migration;
use crate::{
    client::{auth_settings::AuthSettings, LoginMethod},
    crypto::CipherString,
    error::Result,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeysV1 {
    pub crypto_symmetric_key: CipherString,
    pub organization_keys: HashMap<Uuid, CipherString>,
    pub private_key: CipherString,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileV1 {
    pub user_id: Uuid,
    pub name: String,
    pub email: String,
    pub last_sync: DateTime<Utc>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthV1 {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub token_expiration: Option<chrono::DateTime<Utc>>,
    pub login_method: Option<LoginMethod>,

    pub kdf: Option<AuthSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CipherV1 {
    pub id: Uuid,
    pub organization_id: Option<Uuid>,
    pub folder_id: Option<Uuid>,

    pub name: CipherString,

    pub creation_date: DateTime<Utc>,
    pub deleted_date: Option<DateTime<Utc>>,
    pub revision_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FolderV1 {
    pub id: Uuid,
    pub name: CipherString,

    pub revision_date: DateTime<Utc>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct StateV1 {
    pub keys: Option<KeysV1>,
    pub profile: Option<ProfileV1>,
    pub ciphers: HashMap<Uuid, CipherV1>,
    pub folders: HashMap<Uuid, FolderV1>,
    pub auth: AuthV1,

    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

pub(super) struct MigrationV1;
impl Migration<0, 1> for MigrationV1 {
    // This is the first migration, so just start from an empty map
    type Input = Map<String, serde_json::Value>;

    type Output = StateV1;

    fn migrate(&self, _: Self::Input) -> Result<Self::Output> {
        Ok(Self::Output {
            keys: None,
            profile: None,
            ciphers: HashMap::new(),
            folders: HashMap::new(),
            auth: AuthV1::default(),

            extra: HashMap::new(),
        })
    }

    fn rollback(&self, _: Self::Output) -> Result<Self::Input> {
        Ok(Self::Input::new())
    }
}
