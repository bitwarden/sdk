use std::sync::{Arc, Mutex};

use bitwarden_core::{require, Database, DatabaseError, DatabaseTrait, Error};
use idb::{DatabaseEvent, Factory, KeyPath, ObjectStoreParams, TransactionMode};
use serde::Serialize;
use serde_wasm_bindgen::Serializer;
use uuid::Uuid;

use super::Cipher;

/// A row in the ciphers table.
struct CipherRow {
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

    pub fn save(&self, cipher: &Cipher) -> Result<(), DatabaseError> {
        let id = require!(cipher.id);
        let serialized = serde_json::to_string(cipher)?;

        /*let guard = self.db.lock().map_err(|_| DatabaseError::DatabaseLock)?;

        let mut stmt = guard.exec(
            "
                INSERT INTO ciphers (id, value)
                VALUES (?1, ?2)
                ON CONFLICT(id) DO UPDATE SET
                value = ?2
            ",
        )?;
        */
        //stmt.execute((&id, &serialized))?;

        Ok(())
    }

    pub async fn replace_all(&mut self, ciphers: &[Cipher]) -> Result<(), DatabaseError> {
        let mut guard = self.db.lock().map_err(|_| DatabaseError::DatabaseLock)?;

        /*
        let queries: Vec<String> = ciphers
            .iter()
            .map(|c| {
                let id = c.id.unwrap();
                let serialized = serde_json::to_string(c).unwrap();
                format!(
                    "INSERT INTO ciphers (id, value) VALUES ('{}', '{}');",
                    id, serialized
                )
            })
            .collect();

        guard
            .execute_batch(&format!("BEGIN TRANSACTION;{}COMMIT;", queries.join("")))
            .await?;
          */

        // Get a factory instance from global scope
        let factory = Factory::new().unwrap();

        // Create an open request for the database
        let mut open_request = factory.open("test", Some(1)).unwrap();

        // Add an upgrade handler for database
        open_request.on_upgrade_needed(|event| {
            // Get database instance from event
            let database = event.database().unwrap();

            // Prepare object store params
            let mut store_params = ObjectStoreParams::new();
            store_params.auto_increment(true);
            store_params.key_path(Some(KeyPath::new_single("id")));

            // Create object store
            let store = database
                .create_object_store("ciphers", store_params)
                .unwrap();
        });

        // `await` open request
        let database = open_request.await.unwrap();

        // Create a read-write transaction
        let transaction = database
            .transaction(&["ciphers"], TransactionMode::ReadWrite)
            .unwrap();

        // Get the object store
        let store = transaction.object_store("ciphers").unwrap();

        for cipher in ciphers {
            let id = store
                .add(
                    &cipher.serialize(&Serializer::json_compatible()).unwrap(),
                    None,
                )
                .unwrap()
                .await
                .unwrap();
        }

        // Commit the transaction
        transaction.commit().unwrap().await.unwrap();

        //let tx = guard.conn.transaction()?;
        //{
        //guard.execute("DELETE FROM ciphers")?;

        /*let mut stmt = tx.prepare(
            "
            INSERT INTO ciphers (id, value)
            VALUES (?1, ?2)
        ",
        )?;*/

        /*
        for cipher in ciphers {
            let id = require!(cipher.id);
            let serialized = serde_json::to_string(&cipher)?;

            guard
                .execute_batch(&format!(
                    "INSERT INTO ciphers (id, value) VALUES ('{}', '{}')",
                    id, "abc"
                ))
                .await?;
        }
        */

        //}
        //tx.commit()?;

        Ok(())
    }

    pub fn delete_by_id(&self, id: Uuid) -> Result<(), DatabaseError> {
        let guard = self.db.lock().map_err(|_| DatabaseError::DatabaseLock)?;

        //let mut stmt = guard.conn.prepare("DELETE FROM ciphers WHERE id = ?1")?;
        //stmt.execute(params![id])?;

        Ok(())
    }

    pub fn get_all(&self) -> Result<Vec<Cipher>, DatabaseError> {
        let guard = self.db.lock().map_err(|_| DatabaseError::DatabaseLock)?;
        /*
        let mut stmt = guard.conn.prepare("SELECT id, value FROM ciphers")?;
        let rows = stmt.query_map([], |row| {
            Ok(CipherRow {
                id: row.get(0)?,
                value: row.get(1)?,
            })
        })?;

        let ciphers: Vec<Cipher> = rows
            .flatten()
            .flat_map(|row| -> Result<Cipher, Error> {
                let cipher: Cipher = serde_json::from_str(&row.value)?;
                Ok(cipher)
            })
            .collect();

        Ok(ciphers)
        */
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CipherRepromptType, CipherType};

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

    #[test]
    fn test_save_get_all() {
        let repo = CipherRepository::new(Arc::new(Mutex::new(Database::new_test())));

        let cipher = mock_cipher("d55d65d7-c161-40a4-94ca-b0d20184d91a".parse().unwrap());

        repo.save(&cipher).unwrap();

        let ciphers = repo.get_all().unwrap();

        assert_eq!(ciphers.len(), 1);
        assert_eq!(ciphers[0].id, cipher.id);
    }

    #[test]
    fn test_delete_by_id() {
        let repo = CipherRepository::new(Arc::new(Mutex::new(Database::new_test())));

        let cipher = mock_cipher("d55d65d7-c161-40a4-94ca-b0d20184d91a".parse().unwrap());
        repo.save(&cipher).unwrap();

        let ciphers = repo.get_all().unwrap();
        assert_eq!(ciphers.len(), 1);

        repo.delete_by_id(cipher.id.unwrap()).unwrap();
        let ciphers = repo.get_all().unwrap();
        assert_eq!(ciphers.len(), 0);
    }

    #[tokio::test]
    async fn test_replace_all() {
        let mut repo = CipherRepository::new(Arc::new(Mutex::new(Database::new_test())));

        let old_cipher = mock_cipher("d55d65d7-c161-40a4-94ca-b0d20184d91a".parse().unwrap());

        repo.save(&old_cipher).unwrap();

        let ciphers = repo.get_all().unwrap();
        assert_eq!(ciphers.len(), 1);
        assert_eq!(ciphers[0].id, old_cipher.id);

        let new_ciphers = vec![mock_cipher(
            "d55d65d7-c161-40a4-94ca-b0d20184d91c".parse().unwrap(),
        )];

        repo.replace_all(new_ciphers.as_slice()).await.unwrap();

        let ciphers = repo.get_all().unwrap();
        assert_eq!(ciphers.len(), 1);
        assert_eq!(ciphers[0].id, new_ciphers[0].id);
    }
}
