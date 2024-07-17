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

#[derive(Debug, Error)]
pub enum MigrationError {
    #[error(transparent)]
    VaultLocked(#[from] VaultLocked),
}

pub trait Migrator<Versions, LatestVersion> {
    fn migrate(
        &self,
        key: &SymmetricCryptoKey,
        from: Versions,
    ) -> impl std::future::Future<Output = Result<LatestVersion, MigrationError>> + Send;
}
