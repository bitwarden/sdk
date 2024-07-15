use wasm_bindgen::prelude::wasm_bindgen;

use crate::DatabaseError;

#[wasm_bindgen]
extern "C" {
    async fn runSql(sql: &str);
}

#[derive(Debug)]
pub struct SqliteDatabase {}

impl SqliteDatabase {
    pub fn default() -> Result<Self, DatabaseError> {
        SqliteDatabase::new()
    }

    pub fn new() -> Result<Self, DatabaseError> {
        runSql(
            "CREATE TABLE IF NOT EXISTS ciphers (
                id TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
        );
        Ok(SqliteDatabase {})
    }

    pub fn execute(&self, sql: &str) -> Result<(), DatabaseError> {
        runSql(sql);
        Ok(())
    }
}
