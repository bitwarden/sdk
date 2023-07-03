use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Migration;
use crate::{client::encryption_settings::EncryptionSettings, error::Result};

#[derive(serde::Serialize, serde::Deserialize)]
pub(super) struct StateV0 {
    pub profile: Option<ProfileV0>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub(super) struct StateV1 {
    pub profile: Option<ProfileV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ProfileV0 {
    user_id: Uuid,
    email: Option<String>,
    email_address: Option<String>,
    name: String,
    last_sync: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ProfileV1 {
    user_id: Uuid,
    email: String,
    name: String,
    last_sync: DateTime<Utc>,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct MigrationV1;
impl Migration<0, 1> for MigrationV1 {
    type Input = StateV0;

    type Output = StateV1;

    fn migrate(&self, input: Self::Input, _: &EncryptionSettings) -> Result<Self::Output> {
        Ok(Self::Output {
            profile: input.profile.map(|p| ProfileV1 {
                user_id: p.user_id,
                email: p.email_address.or(p.email).unwrap_or_default(),
                name: p.name,
                last_sync: p.last_sync,
            }),
        })
    }

    fn rollback(&self, input: Self::Output, _: &EncryptionSettings) -> Result<Self::Input> {
        Ok(Self::Input {
            profile: input.profile.map(|p| ProfileV0 {
                user_id: p.user_id,
                name: String::new(),
                email: Some(p.email.clone()),
                email_address: Some(p.email),
                last_sync: p.last_sync,
            }),
        })
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};

    use super::super::json_map;
    use super::*;
    use crate::client::encryption_settings::{EncryptionSettings, SymmetricCryptoKey};

    #[test]
    pub fn test_both_emails() {
        let enc = EncryptionSettings::new_single_key(SymmetricCryptoKey::generate("test"));
        let migration = MigrationV1;
        let input = json_map(json!({
            "profile": {
                "userId": "80b97379-80ea-4d12-86cf-ffb1dfb94af9",
                "name": "test",
                "emailAddress": "test1",
                "email": "test2",
                "lastSync": "2023-07-14T10:16:48Z",
            },
        }));

        let mut version = 0;
        let result = migration.try_migrate(input, &enc, &mut version).unwrap();
        assert_eq!(version, 1);
        assert_eq!(result["profile"]["email"], Value::String("test1".into()));
    }

    #[test]
    pub fn test_only_email() {
        let enc = EncryptionSettings::new_single_key(SymmetricCryptoKey::generate("test"));
        let migration = MigrationV1;
        let input = json_map(json!({
            "profile": {
                "userId": "80b97379-80ea-4d12-86cf-ffb1dfb94af9",
                "name": "test",
                "email": "test2",
                "lastSync": "2023-07-14T10:16:48Z",
            },
        }));

        let mut version = 0;
        let result = migration.try_migrate(input, &enc, &mut version).unwrap();
        assert_eq!(version, 1);
        assert_eq!(result["profile"]["email"], Value::String("test2".into()));
    }
}
