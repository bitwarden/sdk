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
    async fn execute(this: &SqliteDatabase, sql: &str);
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
        db.execute(
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
        self.db.execute(sql).await;

        Ok(())
    }
}
