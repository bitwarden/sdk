use thiserror::Error;

use crate::RowError;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Database lock")]
    DatabaseLock,

    #[error("Failed to open connection to database")]
    FailedToOpenConnection,

    #[error("Unable to get version")]
    UnableToGetVersion,
    #[error("Unable to set version")]
    UnableToSetVersion,

    #[cfg(not(target_arch = "wasm32"))]
    #[error(transparent)]
    Rusqlite(#[from] rusqlite::Error),

    // Used on the consuming side, resolves to rusqlite for sqlite and our RowError for wasm
    #[error(transparent)]
    RowError(RowError),
}
