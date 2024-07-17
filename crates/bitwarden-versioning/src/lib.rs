use std::sync::{Arc, Mutex};

use bitwarden_crypto::{CryptoError, SymmetricCryptoKey};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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

    #[cfg(test)]
    pub fn get_raw_data(&self) -> &Versions {
        &self.data
    }
}

pub trait Migrator<LatestVersion> {
    fn migrate(&self, key: &SymmetricCryptoKey) -> Result<LatestVersion, CryptoError>;
}

impl<Data, LatestVersion> Migrator<LatestVersion> for Versioned<Data, LatestVersion>
where
    Data: Migrator<LatestVersion> + std::marker::Sync,
    LatestVersion: Clone + std::marker::Sync + std::marker::Send,
{
    fn migrate(&self, key: &SymmetricCryptoKey) -> Result<LatestVersion, CryptoError> {
        let mut cache = self.cache.lock().expect("Mutex is not poisoned");

        let migrated = match cache.as_ref() {
            Some(value) => value.clone(),
            None => {
                let migrated: LatestVersion = self.data.migrate(key)?;
                *cache = Some(migrated.clone());
                migrated
            }
        };

        Ok(migrated)
    }
}
