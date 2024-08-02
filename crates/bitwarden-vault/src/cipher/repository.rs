use std::sync::Arc;

use bitwarden_db::{named_params, params, Database, DatabaseError, DatabaseTrait};
use tokio::sync::Mutex;
use uuid::Uuid;

use super::Cipher;

/// A row in the ciphers table.
pub struct CipherRow {
    #[allow(dead_code)]
    id: Uuid,
    value: String,
}

pub struct CipherRepository {
    db: Arc<Mutex<Database>>,
}

impl CipherRepository {
    pub fn new(db: Arc<Mutex<Database>>) -> Self {
        Self { db: db.clone() }
    }

    pub async fn save(&self, cipher: &Cipher) -> Result<(), DatabaseError> {
        let id = cipher.id.unwrap();
        let serialized = serde_json::to_string(cipher).unwrap();

        let guard = self.db.lock().await;

        guard
            .execute(
                "
                    INSERT INTO ciphers (id, value)
                    VALUES (?1, ?2)
                    ON CONFLICT(id) DO UPDATE SET
                    value = ?2
                ",
                params![&id, &serialized],
            )
            .await
            .map(|_| ())
    }

    pub async fn replace_all(&self, ciphers: &[Cipher]) -> Result<(), DatabaseError> {
        let guard = self.db.lock().await;

        guard.execute("DELETE FROM ciphers", []).await?;

        for cipher in ciphers {
            let id = cipher.id.unwrap();
            let serialized = serde_json::to_string(&cipher).unwrap();

            guard
                .execute(
                    "INSERT INTO ciphers (id, value) VALUES (:id, :data)",
                    named_params! {":id": id, ":data": serialized},
                )
                .await?;
        }

        Ok(())
    }

    pub async fn delete_by_id(&self, id: Uuid) -> Result<(), DatabaseError> {
        let guard = self.db.lock().await;

        guard
            .execute("DELETE FROM ciphers WHERE id = ?1", [id])
            .await?;

        Ok(())
    }

    pub async fn get_all(&self) -> Result<Vec<CipherRow>, DatabaseError> {
        let guard = self.db.lock().await;

        let rows = guard
            .query_map(
                "SELECT id, value FROM ciphers",
                |row| -> Result<CipherRow, DatabaseError> {
                    Ok(CipherRow {
                        id: row.get(0)?,
                        value: row.get(1)?,
                    })
                },
            )
            .await?;

        Ok(rows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CipherRepromptType, CipherType};

    async fn init_database() -> Arc<Mutex<Database>> {
        let db = Database::new_test();

        db.execute_batch(
            "
                    CREATE TABLE ciphers (
                        id TEXT PRIMARY KEY,
                        value TEXT NOT NULL
                    )
                    ",
        )
        .await
        .unwrap();

        Arc::new(Mutex::new(db))
    }

    fn mock_cipher(id: Uuid) -> Cipher {
        Cipher {
            id: Some(id),
            organization_id: None,
            folder_id: None,
            collection_ids: vec![],
            key: None,
            name: "2.pMS6/icTQABtulw52pq2lg==|XXbxKxDTh+mWiN1HjH2N1w==|Q6PkuT+KX/axrgN9ubD5Ajk2YNwxQkgs3WJM0S0wtG8=".parse().unwrap(),
            notes: None,
            r#type: CipherType::Login,
            login: None,
            identity: None,
            card: None,
            secure_note: None,
            favorite: false,
            reprompt: CipherRepromptType::None,
            organization_use_totp: false,
            edit: true,
            view_password: true,
            local_data: None,
            attachments: None,
            fields: None,
            password_history: None,
            creation_date: "2024-01-30T17:55:36.150Z".parse().unwrap(),
            deleted_date: None,
            revision_date: "2024-01-30T17:55:36.150Z".parse().unwrap(),
        }
    }

    #[tokio::test]
    async fn test_save_get_all() {
        let repo = CipherRepository::new(init_database().await);

        let cipher = mock_cipher("d55d65d7-c161-40a4-94ca-b0d20184d91a".parse().unwrap());

        repo.save(&cipher).await.unwrap();

        let ciphers = repo.get_all().await.unwrap();

        assert_eq!(ciphers.len(), 1);
        assert_eq!(ciphers[0].id, cipher.id.unwrap());
    }

    #[tokio::test]
    async fn test_delete_by_id() {
        let repo = CipherRepository::new(init_database().await);

        let cipher = mock_cipher("d55d65d7-c161-40a4-94ca-b0d20184d91a".parse().unwrap());
        repo.save(&cipher).await.unwrap();

        let ciphers = repo.get_all().await.unwrap();
        assert_eq!(ciphers.len(), 1);

        repo.delete_by_id(cipher.id.unwrap()).await.unwrap();
        let ciphers = repo.get_all().await.unwrap();
        assert_eq!(ciphers.len(), 0);
    }

    #[tokio::test]
    async fn test_replace_all() {
        let repo = CipherRepository::new(init_database().await);

        let old_cipher = mock_cipher("d55d65d7-c161-40a4-94ca-b0d20184d91a".parse().unwrap());

        repo.save(&old_cipher).await.unwrap();

        let ciphers = repo.get_all().await.unwrap();
        assert_eq!(ciphers.len(), 1);
        assert_eq!(ciphers[0].id, old_cipher.id.unwrap());

        let new_ciphers = vec![mock_cipher(
            "d55d65d7-c161-40a4-94ca-b0d20184d91c".parse().unwrap(),
        )];

        repo.replace_all(new_ciphers.as_slice()).await.unwrap();

        let ciphers = repo.get_all().await.unwrap();
        assert_eq!(ciphers.len(), 1);
        assert_eq!(ciphers[0].id, new_ciphers[0].id.unwrap());
    }
}
