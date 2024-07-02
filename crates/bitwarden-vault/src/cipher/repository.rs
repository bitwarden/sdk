use std::sync::{Arc, Mutex};

use rusqlite::params;
use uuid::Uuid;

use super::Cipher;
use bitwarden_core::{require, DatabaseError, Error, SqliteDatabase};

pub trait CipherRepository {
    /// Save a cipher to the repository.
    fn save(&self, cipher: &Cipher) -> Result<(), DatabaseError>;

    /// Replace all ciphers in the repository with the given ciphers.
    ///
    /// Typically used during a sync operation.
    fn replace_all(&mut self, ciphers: &[Cipher]) -> Result<(), DatabaseError>;

    /// Delete a cipher by its ID.
    fn delete_by_id(&self, id: Uuid) -> Result<(), DatabaseError>;

    /// Get all ciphers from the repository.
    fn get_all(&self) -> Result<Vec<Cipher>, DatabaseError>;
}

/// A row in the ciphers table.
struct CipherRow {
    #[allow(dead_code)]
    id: Uuid,
    value: String,
}

pub struct CipherSqliteRepository {
    db: Arc<Mutex<SqliteDatabase>>,
}

impl CipherSqliteRepository {
    pub fn new(db: Arc<Mutex<SqliteDatabase>>) -> Self {
        Self { db: db.clone() }
    }
}

impl CipherRepository for CipherSqliteRepository {
    fn save(&self, cipher: &Cipher) -> Result<(), DatabaseError> {
        let id = require!(cipher.id);
        let serialized = serde_json::to_string(cipher)?;

        let guard = self.db.lock().map_err(|_| DatabaseError::DatabaseLock)?;

        let mut stmt = guard.conn.prepare(
            "
                INSERT INTO ciphers (id, value)
                VALUES (?1, ?2)
                ON CONFLICT(id) DO UPDATE SET
                value = ?2
            ",
        )?;
        stmt.execute((&id, &serialized))?;

        Ok(())
    }

    fn replace_all(&mut self, ciphers: &[Cipher]) -> Result<(), DatabaseError> {
        let mut guard = self.db.lock().map_err(|_| DatabaseError::DatabaseLock)?;

        let tx = guard.conn.transaction()?;
        {
            tx.execute("DELETE FROM ciphers", [])?;

            let mut stmt = tx.prepare(
                "
                INSERT INTO ciphers (id, value)
                VALUES (?1, ?2)
            ",
            )?;

            for cipher in ciphers {
                let id = require!(cipher.id);
                let serialized = serde_json::to_string(&cipher)?;

                stmt.execute(params![id, serialized])?;
            }
        }
        tx.commit()?;

        Ok(())
    }

    fn delete_by_id(&self, id: Uuid) -> Result<(), DatabaseError> {
        let guard = self.db.lock().map_err(|_| DatabaseError::DatabaseLock)?;

        let mut stmt = guard.conn.prepare("DELETE FROM ciphers WHERE id = ?1")?;
        stmt.execute(params![id])?;

        Ok(())
    }

    fn get_all(&self) -> Result<Vec<Cipher>, DatabaseError> {
        let guard = self.db.lock().map_err(|_| DatabaseError::DatabaseLock)?;

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
        let repo = CipherSqliteRepository::new(Arc::new(Mutex::new(SqliteDatabase::new_test())));

        let cipher = mock_cipher("d55d65d7-c161-40a4-94ca-b0d20184d91a".parse().unwrap());

        repo.save(&cipher).unwrap();

        let ciphers = repo.get_all().unwrap();

        assert_eq!(ciphers.len(), 1);
        assert_eq!(ciphers[0].id, cipher.id);
    }

    #[test]
    fn test_delete_by_id() {
        let repo = CipherSqliteRepository::new(Arc::new(Mutex::new(SqliteDatabase::new_test())));

        let cipher = mock_cipher("d55d65d7-c161-40a4-94ca-b0d20184d91a".parse().unwrap());
        repo.save(&cipher).unwrap();

        let ciphers = repo.get_all().unwrap();
        assert_eq!(ciphers.len(), 1);

        repo.delete_by_id(cipher.id.unwrap()).unwrap();
        let ciphers = repo.get_all().unwrap();
        assert_eq!(ciphers.len(), 0);
    }

    #[test]
    fn test_replace_all() {
        let mut repo =
            CipherSqliteRepository::new(Arc::new(Mutex::new(SqliteDatabase::new_test())));

        let old_cipher = mock_cipher("d55d65d7-c161-40a4-94ca-b0d20184d91a".parse().unwrap());

        repo.save(&old_cipher).unwrap();

        let ciphers = repo.get_all().unwrap();
        assert_eq!(ciphers.len(), 1);
        assert_eq!(ciphers[0].id, old_cipher.id);

        let new_ciphers = vec![mock_cipher(
            "d55d65d7-c161-40a4-94ca-b0d20184d91c".parse().unwrap(),
        )];

        repo.replace_all(new_ciphers.as_slice()).unwrap();

        let ciphers = repo.get_all().unwrap();
        assert_eq!(ciphers.len(), 1);
        assert_eq!(ciphers[0].id, new_ciphers[0].id);
    }
}
