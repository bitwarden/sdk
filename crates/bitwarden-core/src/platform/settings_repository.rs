use std::sync::Arc;

use bitwarden_db::{params, Database, DatabaseError, DatabaseTrait};
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;
use tokio::sync::Mutex;

#[derive(Debug, Error)]
pub enum SettingsRepositoryError {
    #[error(transparent)]
    Database(#[from] DatabaseError),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}

pub struct SettingsRepository {
    db: Arc<Mutex<Database>>,
}

impl SettingsRepository {
    pub fn new(db: Arc<Mutex<Database>>) -> Self {
        Self { db: db.clone() }
    }

    pub async fn get<T: DeserializeOwned>(
        &self,
        key: &str,
    ) -> Result<Option<T>, SettingsRepositoryError> {
        let guard = self.db.lock().await;

        let res = guard
            .query_map(
                "SELECT value FROM settings WHERE key = ?1",
                [key],
                |row| -> Result<String, _> { row.get(0) },
            )
            .await?
            .first()
            .map(|x| serde_json::from_str::<T>(x))
            .transpose()?;

        Ok(res)
    }

    pub async fn set<T: Serialize>(
        &self,
        key: &str,
        value: &T,
    ) -> Result<(), SettingsRepositoryError> {
        let value = serde_json::to_string(value)?;
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
