mod sqlite;
use std::borrow::Cow;

pub use sqlite::SqliteDatabase;
use thiserror::Error;

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
}

#[derive(Debug, Error)]
pub enum MigratorError {
    #[error("Internal error: {0}")]
    Internal(Cow<'static, str>),
}
