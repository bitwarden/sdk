mod sqlite;
use std::borrow::Cow;

pub use sqlite::SqliteDatabase;
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
    Rusqlite(#[from] rusqlite::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    MissingField(#[from] MissingFieldError),
}

#[derive(Debug, Error)]
pub enum MigratorError {
    #[error("Internal error: {0}")]
    Internal(Cow<'static, str>),
}
