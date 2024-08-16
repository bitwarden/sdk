use bitwarden_crypto::Kdf;
use bitwarden_db::DatabaseError;
use thiserror::Error;

use crate::platform::SettingsRepository;

const SETTINGS_KEY: &str = "auth";

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AuthSettings {
    pub email: String,
    pub token: String,
    pub refresh_token: Option<String>,
    pub kdf: Kdf,

    pub user_key: String,
    pub private_key: String,
}

#[derive(Debug, Error)]
pub enum AuthRepositoryError {
    #[error(transparent)]
    Database(#[from] DatabaseError),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}

pub struct AuthRepository {
    settings_repository: SettingsRepository,
}

impl AuthRepository {
    pub fn new(settings_repository: SettingsRepository) -> Self {
        Self {
            settings_repository,
        }
    }

    pub(crate) async fn save(&self, setting: AuthSettings) -> Result<(), AuthRepositoryError> {
        let serialized = serde_json::to_string(&setting)?;
        self.settings_repository
            .set(SETTINGS_KEY, &serialized)
            .await?;

        Ok(())
    }

    pub(crate) async fn get(&self) -> Result<Option<AuthSettings>, AuthRepositoryError> {
        let settings = self.settings_repository.get(SETTINGS_KEY).await?;

        match settings {
            Some(settings) => {
                let settings: AuthSettings = serde_json::from_str(&settings)?;
                Ok(Some(settings))
            }
            None => Ok(None),
        }
    }
}
