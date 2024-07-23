use rusqlite::Connection;

use super::{migrator::Migrator, DatabaseError, DatabaseTrait};

#[derive(Debug)]
pub struct SqliteDatabase {
    pub conn: Connection,
}

impl SqliteDatabase {
    pub async fn default() -> Result<Self, DatabaseError> {
        let conn =
            Connection::open("test.sqlite").map_err(|_| DatabaseError::FailedToOpenConnection)?;

        Self::new_conn(conn).await
    }

    /// Helper for initializing a in-memory database for testing.
    pub async fn new_test() -> Self {
        let conn =
            Connection::open_in_memory().expect("Failed to open in-memory sqlite connection");

        SqliteDatabase { conn }
    }

    /// Create a new SqliteDatabase with the given connection.
    ///
    /// This will migrate the database to the latest version.
    async fn new_conn(conn: Connection) -> Result<Self, DatabaseError> {
        let db = SqliteDatabase { conn };

        let migrator = Migrator::new();
        migrator
            .migrate(&db, None)
            .await
            .map_err(DatabaseError::Migrator)?;

        Ok(db)
    }
}

impl DatabaseTrait for SqliteDatabase {
    /*


    /// Create a new SqliteDatabase with the given connection.
    ///
    /// This will migrate the database to the latest version.
    fn new_conn(conn: Connection) -> Result<Self, DatabaseError> {
        let migrator = Migrator::new();
        migrator
            .migrate(&conn, None)
            .map_err(DatabaseError::Migrator)?;

        Ok(SqliteDatabase { conn })
    }
    */

    async fn get_version(&self) -> Result<usize, DatabaseError> {
        let version: usize = self
            .conn
            .query_row("PRAGMA user_version", [], |row| row.get(0))
            .map_err(|_| DatabaseError::UnableToGetVersion)?;

        Ok(version)
    }

    async fn set_version(&self, version: usize) -> Result<(), DatabaseError> {
        self.conn
            .pragma_update(None, "user_version", version)
            .map_err(|_| DatabaseError::UnableToSetVersion)?;

        Ok(())
    }

    async fn execute_batch(&self, sql: &str) -> Result<(), DatabaseError> {
        self.conn.execute_batch(sql)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_version() {
        let db = SqliteDatabase::new_test().await;

        let version = db.get_version().await.unwrap();
        assert_eq!(version, 0);

        db.set_version(1).await.unwrap();
        let version = db.get_version().await.unwrap();
        assert_eq!(version, 1);
    }

    #[tokio::test]
    async fn test_execute_batch() {
        let db = SqliteDatabase::new_test().await;

        db.execute_batch("CREATE TABLE test (id INTEGER PRIMARY KEY, name TEXT)")
            .await
            .unwrap();

        db.execute_batch("INSERT INTO test (name) VALUES ('test')")
            .await
            .unwrap();

        let count: i64 = db
            .conn
            .query_row("SELECT COUNT(*) FROM test", [], |row| row.get(0))
            .unwrap();

        assert_eq!(count, 1);
    }
}
