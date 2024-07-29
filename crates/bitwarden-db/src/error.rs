use thiserror::Error;

use crate::migrator::MigratorError;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Database lock")]
    DatabaseLock,

    #[error("Failed to open connection to database")]
    FailedToOpenConnection,

    #[error(transparent)]
    Migrator(#[from] MigratorError),

    // #[error(transparent)]
    // SerdeJson(#[from] serde_json::Error),
    // #[error(transparent)]
    // MissingField(#[from] MissingFieldError),
    #[error("Unable to get version")]
    UnableToGetVersion,
    #[error("Unable to set version")]
    UnableToSetVersion,

    #[cfg(not(target_arch = "wasm32"))]
    #[error(transparent)]
    Rusqlite(#[from] rusqlite::Error),
}
