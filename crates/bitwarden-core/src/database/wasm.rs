use wasm_bindgen::prelude::wasm_bindgen;

use crate::DatabaseError;

#[wasm_bindgen]
extern "C" {
    async fn runSql(sql: &str);
}

#[derive(Debug)]
pub struct WasmDatabase {}

impl WasmDatabase {
    pub fn default() -> Result<Self, DatabaseError> {
        WasmDatabase::new()
    }

    pub fn new() -> Result<Self, DatabaseError> {
        runSql(
            "CREATE TABLE IF NOT EXISTS ciphers (
                id TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
        );
        Ok(WasmDatabase {})
    }

    pub fn execute(&self, sql: &str) -> Result<(), DatabaseError> {
        runSql(sql);
        Ok(())
    }
}
