mod params;
pub use params::{Params, ToSql};

use serde::Serialize;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use super::{DatabaseError, DatabaseTrait};

#[wasm_bindgen]
extern "C" {
    type SqliteDatabase;

    //async fn runSql(sql: &str);

    #[wasm_bindgen(static_method_of = SqliteDatabase)]
    async fn factory(name: &str) -> JsValue;

    #[wasm_bindgen(method)]
    async fn get_version(this: &SqliteDatabase) -> JsValue;

    #[wasm_bindgen(method)]
    async fn set_version(this: &SqliteDatabase, version: u32);

    #[wasm_bindgen(method)]
    async fn execute_batch(this: &SqliteDatabase, sql: &str);

    #[wasm_bindgen(method)]
    async fn execute(this: &SqliteDatabase, sql: &str, params: JsValue);
}

impl core::fmt::Debug for SqliteDatabase {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "SqliteDatabase")
    }
}

#[derive(Debug)]
pub struct WasmDatabase {
    db: SqliteDatabase,
}

impl WasmDatabase {
    pub async fn default() -> Result<Self, DatabaseError> {
        WasmDatabase::new().await
    }

    pub async fn new() -> Result<Self, DatabaseError> {
        let db: SqliteDatabase = SqliteDatabase::factory("test").await.into();
        db.execute_batch(
            "CREATE TABLE IF NOT EXISTS ciphers (
                id TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
        )
        .await;
        Ok(WasmDatabase { db })
    }
    /*
    pub fn execute(&self, sql: &str) -> Result<(), DatabaseError> {
        runSql(sql);
        Ok(())
    }*/
}

impl DatabaseTrait for WasmDatabase {
    async fn get_version(&self) -> Result<usize, DatabaseError> {
        Ok(self
            .db
            .get_version()
            .await
            .as_f64()
            .ok_or(DatabaseError::UnableToGetVersion)? as usize)
    }

    async fn set_version(&self, version: usize) -> Result<(), DatabaseError> {
        self.db.set_version(version.try_into().unwrap_or(0)).await;

        Ok(())
    }

    async fn execute_batch(&self, sql: &str) -> Result<(), DatabaseError> {
        self.db.execute_batch(sql).await;

        Ok(())
    }

    async fn execute<P: Params>(&self, sql: &str, params: P) -> Result<usize, DatabaseError> {
        self.db.execute(sql, params.to_sql()).await;

        Ok(0)
    }
}
