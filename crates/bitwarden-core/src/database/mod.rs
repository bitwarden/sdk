use migrator::MigratorError;

mod migrator;

#[cfg(all(feature = "sqlite", feature = "wasm"))]
compile_error!("Sqlite and wasm are mutually exclusive and cannot be enabled together");

#[cfg(feature = "sqlite")]
mod sqlite;
#[cfg(feature = "sqlite")]
pub type Database = sqlite::SqliteDatabase;

#[cfg(feature = "wasm")]
mod wasm;
#[cfg(feature = "wasm")]
pub type Database = wasm::WasmDatabase;

use thiserror::Error;

use crate::MissingFieldError;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Database lock")]
    DatabaseLock,

    #[error("Failed to open connection to database")]
    FailedToOpenConnection,

    #[error(transparent)]
    Migrator(#[from] MigratorError),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    MissingField(#[from] MissingFieldError),

    #[error("Unable to get version")]
    UnableToGetVersion,
    #[error("Unable to set version")]
    UnableToSetVersion,
}

/// Persistent storage for the Bitwarden SDK
///
/// The database is used to store the user's data, such as ciphers, folders, and settings.
/// Since we need to support multiple platforms, the database is abstracted to allow for different
/// implementations.
///
/// The default and recommended implementation is SqliteDatabase.
pub trait DatabaseTrait {
    async fn get_version(&self) -> Result<usize, DatabaseError>;
    async fn set_version(&self, version: usize) -> Result<(), DatabaseError>;

    async fn execute_batch(&self, sql: &str) -> Result<(), DatabaseError>;
}
