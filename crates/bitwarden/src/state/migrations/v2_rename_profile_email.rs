use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Migration;
use crate::error::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileV2 {
    pub user_id: Uuid,
    pub name: String,
    pub email_address: String,
    pub last_sync: DateTime<Utc>,
}

pub type KeysV2 = super::v1_initial_migration::KeysV1;
pub type CipherV2 = super::v1_initial_migration::CipherV1;
pub type FolderV2 = super::v1_initial_migration::FolderV1;
pub type AuthV2 = super::v1_initial_migration::AuthV1;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct StateV2 {
    pub keys: Option<KeysV2>,
    pub profile: Option<ProfileV2>,
    pub ciphers: HashMap<Uuid, CipherV2>,
    pub folders: HashMap<Uuid, FolderV2>,
    pub auth: AuthV2,

    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

pub(super) struct MigrationV2;
impl Migration<1, 2> for MigrationV2 {
    type Input = super::v1_initial_migration::StateV1;

    type Output = StateV2;

    fn migrate(&self, input: Self::Input) -> Result<Self::Output> {
        Ok(Self::Output {
            keys: input.keys,
            profile: input.profile.map(|p| ProfileV2 {
                user_id: p.user_id,
                name: p.name,
                email_address: p.email,
                last_sync: p.last_sync,
            }),
            ciphers: input.ciphers,
            folders: input.folders,
            auth: input.auth,

            extra: input.extra,
        })
    }

    fn rollback(&self, input: Self::Output) -> Result<Self::Input> {
        Ok(Self::Input {
            keys: input.keys,
            profile: input
                .profile
                .map(|p| super::v1_initial_migration::ProfileV1 {
                    user_id: p.user_id,
                    name: p.name,
                    email: p.email_address,
                    last_sync: p.last_sync,
                }),
            ciphers: input.ciphers,
            folders: input.folders,
            auth: input.auth,

            extra: input.extra,
        })
    }
}
