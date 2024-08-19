use bitwarden_crypto::Kdf;

use crate::platform::{SettingsRepository, SettingsRepositoryError};

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

pub struct AuthRepository {
    settings_repository: SettingsRepository,
}

impl AuthRepository {
    pub fn new(settings_repository: SettingsRepository) -> Self {
        Self {
            settings_repository,
        }
    }

    pub(crate) async fn save(&self, setting: AuthSettings) -> Result<(), SettingsRepositoryError> {
        self.settings_repository.set(SETTINGS_KEY, &setting).await?;

        Ok(())
    }

    pub(crate) async fn get(&self) -> Result<Option<AuthSettings>, SettingsRepositoryError> {
        let settings = self.settings_repository.get(SETTINGS_KEY).await?;

        Ok(settings)
    }
}
