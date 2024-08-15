use std::sync::Arc;

use bitwarden_db::{params, Database, DatabaseError, DatabaseTrait};
use tokio::sync::Mutex;

pub struct SettingsRepository {
    db: Arc<Mutex<Database>>,
}

impl SettingsRepository {
    pub fn new(db: Arc<Mutex<Database>>) -> Self {
        Self { db: db.clone() }
    }

    pub async fn get(&self, key: &str) -> Result<Option<String>, DatabaseError> {
        let guard = self.db.lock().await;

        let res = guard
            .query_map(
                "SELECT value FROM settings WHERE key = ?1",
                [key],
                |row| -> Result<String, _> { row.get(0) },
            )
            .await?
            .first()
            .map(|x| x.to_owned());

        Ok(res)
    }

    pub async fn set(&self, key: &str, value: &str) -> Result<(), DatabaseError> {
        let guard = self.db.lock().await;

        guard
            .execute(
                "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
                params![key, value],
            )
            .await?;

        Ok(())
    }
}
