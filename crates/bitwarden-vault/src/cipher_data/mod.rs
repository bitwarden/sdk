use bitwarden_core::VaultLocked;
use thiserror::Error;

pub mod latest;
mod v1;
mod v2;

#[derive(Debug, Error)]
pub enum MigrationError {
    #[error(transparent)]
    VaultLocked(#[from] VaultLocked),
}

pub trait Migrator<From, To> {
    async fn migrate_from(&self, from: From) -> Result<To, MigrationError>;
}
