use bitwarden_core::VaultLocked;
use bitwarden_crypto::SymmetricCryptoKey;
use thiserror::Error;

/// A wrapper for versioned data.
/// The internal data can be stored as any version, but data cannot
/// be accessed without migrating it to the latest version.
pub struct Versioned<Versions, LatestVersion> {
    data: Versions,
    _output: std::marker::PhantomData<LatestVersion>,
}

impl<Versions, LatestVersion> Versioned<Versions, LatestVersion> {
    pub fn new(data: Versions) -> Self {
        Self {
            data,
            _output: std::marker::PhantomData,
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
    LatestVersion: std::marker::Sync,
{
    async fn migrate(&self, key: &SymmetricCryptoKey) -> Result<LatestVersion, MigrationError> {
        self.data.migrate(key).await
    }
}
