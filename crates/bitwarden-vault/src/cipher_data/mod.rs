use bitwarden_core::VaultLocked;
use thiserror::Error;

// pub mod domain;
mod v1;
mod v2;
mod version_agnostic;

#[derive(Debug, Error)]
pub enum MigrationError {
    #[error(transparent)]
    VaultLocked(#[from] VaultLocked),
}

pub trait Migrator<From, To> {
    async fn migrate_from(&self, from: From) -> Result<To, MigrationError>;
}
