use std::sync::Arc;

use bitwarden_core::VaultLocked;
use bitwarden_crypto::SymmetricCryptoKey;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::Mutex;

/// A wrapper for versioned data.
/// The internal data can be stored as any version, but data cannot
/// be accessed without migrating it to the latest version.
#[derive(Clone, Serialize, Deserialize, Debug, Default, JsonSchema)]
pub struct Versioned<Versions, LatestVersion> {
    data: Versions,
    #[serde(skip)]
    cache: Arc<Mutex<Option<LatestVersion>>>,
}

impl<Versions, LatestVersion> Versioned<Versions, LatestVersion> {
    pub fn new(data: Versions) -> Self {
        Self {
            data,
            cache: Arc::new(None.into()),
        }
    }
}

#[derive(Debug, Error)]
pub enum MigrationError {
    #[error(transparent)]
    VaultLocked(#[from] VaultLocked),
}

pub trait Migrator<LatestVersion> {
    fn migrate(
        &self,
        key: &SymmetricCryptoKey,
    ) -> impl std::future::Future<Output = Result<LatestVersion, MigrationError>> + Send;
}

impl<Data, LatestVersion> Migrator<LatestVersion> for Versioned<Data, LatestVersion>
where
    Data: Migrator<LatestVersion> + std::marker::Sync,
    LatestVersion: Clone + std::marker::Sync + std::marker::Send,
{
    async fn migrate(&self, key: &SymmetricCryptoKey) -> Result<LatestVersion, MigrationError> {
        let mut cache = self.cache.lock().await;

        let migrated = match cache.as_ref() {
            Some(value) => value.clone(),
            None => {
                let migrated: LatestVersion = self.data.migrate(key).await?;
                *cache = Some(migrated.clone());
                migrated
            }
        };

        Ok(migrated)
    }
}
