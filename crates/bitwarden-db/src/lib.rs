#![allow(async_fn_in_trait)]
mod error;
pub use error::DatabaseError;
mod migrator;
pub use migrator::*;

// #[cfg(all(feature = "sqlite", feature = "wasm"))]
// compile_error!("Sqlite and wasm are mutually exclusive and cannot be enabled together");

#[cfg(not(target_arch = "wasm32"))]
mod sqlite;
#[cfg(not(target_arch = "wasm32"))]
pub type Database = sqlite::SqliteDatabase;
#[cfg(not(target_arch = "wasm32"))]
pub use sqlite::{named_params, params, Params, Row};

#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
pub type Database = wasm::WasmDatabase;
#[cfg(target_arch = "wasm32")]
pub use wasm::{Params, Row, ToSql};

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

    /// Convenience method to prepare and execute a single SQL statement.
    ///
    /// On success, returns the number of rows that were changed or inserted or deleted.
    async fn execute<P: Params>(&self, sql: &str, params: P) -> Result<usize, DatabaseError>;

    async fn query_map<P: Params, T, F>(
        &self,
        query: &str,
        params: P,
        row_to_type: F,
    ) -> Result<Vec<T>, DatabaseError>
    where
        F: Fn(&Row) -> Result<T, DatabaseError>;
}
