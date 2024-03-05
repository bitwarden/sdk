use rusqlite::{params, Connection};
use uuid::Uuid;

use crate::error::Error;

use super::Cipher;

struct CipherRow {
    id: Uuid,
    value: String,
}

struct CipherSqliteRepository {
    conn: Connection,
}

impl CipherSqliteRepository {
    pub fn new(conn: Connection) -> Self {
        // TODO: Handle schema migrations
        conn.execute(
            "CREATE TABLE IF NOT EXISTS ciphers (
                id TEXT PRIMARY KEY,
                value TEXT NOT NULL
         )",
            (),
        )
        .unwrap();

        Self { conn }
    }

    pub fn save(&self, cipher: &Cipher) -> Result<(), Error> {
        let id = cipher.id.unwrap();
        let serialized = serde_json::to_string(cipher)?;

        let mut stmt = self.conn.prepare(
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

    /// Replace all ciphers in the repository with the given ciphers.
    ///
    /// Typically used during a sync operation.
    pub fn replace_all(&mut self, ciphers: &[&Cipher]) -> Result<(), Error> {
        let tx = self.conn.transaction()?;
        {
            tx.execute("DELETE FROM ciphers", [])?;

            let mut stmt = tx.prepare(
                "
                INSERT INTO ciphers (id, value)
                VALUES (?1, ?2)
            ",
            )?;

            for cipher in ciphers {
                let id = cipher.id.unwrap();
                let serialized = serde_json::to_string(&cipher)?;

                stmt.execute(params![id, serialized])?;
            }
        }
        tx.commit()?;

        Ok(())
    }

    pub fn delete_by_id(&self, id: Uuid) -> Result<(), Error> {
        let mut stmt = self.conn.prepare("DELETE FROM ciphers WHERE id = ?1")?;
        stmt.execute(params![id])?;

        Ok(())
    }

    pub fn get_all(&self) -> Result<Vec<Cipher>, Error> {
        let mut stmt = self.conn.prepare("SELECT id, value FROM ciphers")?;
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
    use crate::vault::{CipherRepromptType, CipherType};

    use super::*;
    use rusqlite::Connection;

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
        let conn = Connection::open_in_memory().unwrap();
        let repo = CipherSqliteRepository::new(conn);

        let cipher = mock_cipher("d55d65d7-c161-40a4-94ca-b0d20184d91a".parse().unwrap());

        repo.save(&cipher).unwrap();

        let ciphers = repo.get_all().unwrap();

        assert_eq!(ciphers.len(), 1);
        assert_eq!(ciphers[0].id, cipher.id);
    }

    #[test]
    fn test_delete_by_id() {
        let conn = Connection::open_in_memory().unwrap();
        let repo = CipherSqliteRepository::new(conn);

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
        let conn = Connection::open_in_memory().unwrap();
        let mut repo = CipherSqliteRepository::new(conn);

        let old_cipher = mock_cipher("d55d65d7-c161-40a4-94ca-b0d20184d91a".parse().unwrap());

        repo.save(&old_cipher).unwrap();

        let ciphers = repo.get_all().unwrap();
        assert_eq!(ciphers.len(), 1);
        assert_eq!(ciphers[0].id, old_cipher.id);

        let new_cipher = mock_cipher("d55d65d7-c161-40a4-94ca-b0d20184d91c".parse().unwrap());
        repo.replace_all(&[&new_cipher]).unwrap();

        let ciphers = repo.get_all().unwrap();
        assert_eq!(ciphers.len(), 1);
        assert_eq!(ciphers[0].id, new_cipher.id);
    }
}
