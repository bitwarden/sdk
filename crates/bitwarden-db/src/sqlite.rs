use rusqlite::Connection;
pub use rusqlite::{named_params, params, Params, Row};
use tokio::sync::Mutex;

use super::{DatabaseError, DatabaseTrait};

pub type RowError = rusqlite::Error;

#[derive(Debug)]
pub struct SqliteDatabase {
    conn: Mutex<Connection>,
}

impl SqliteDatabase {
    pub async fn default() -> Result<Self, DatabaseError> {
        let conn =
            Connection::open("test.sqlite").map_err(|_| DatabaseError::FailedToOpenConnection)?;

        Self::new_conn(conn).await
    }

    /// Helper for initializing a in-memory database for testing.
    pub fn new_test() -> Self {
        let conn =
            Connection::open_in_memory().expect("Failed to open in-memory sqlite connection");

        SqliteDatabase {
            conn: Mutex::new(conn),
        }
    }

    /// Create a new SqliteDatabase with the given connection.
    async fn new_conn(conn: Connection) -> Result<Self, DatabaseError> {
        let db = SqliteDatabase {
            conn: Mutex::new(conn),
        };

        Ok(db)
    }
}

impl DatabaseTrait for SqliteDatabase {
    async fn get_version(&self) -> Result<usize, DatabaseError> {
        let guard = self.conn.lock().await;

        let version: usize = guard
            .query_row("PRAGMA user_version", [], |row| row.get(0))
            .map_err(|_| DatabaseError::UnableToGetVersion)?;

        Ok(version)
    }

    async fn set_version(&self, version: usize) -> Result<(), DatabaseError> {
        let guard = self.conn.lock().await;

        guard
            .pragma_update(None, "user_version", version)
            .map_err(|_| DatabaseError::UnableToSetVersion)?;

        Ok(())
    }

    async fn execute_batch(&self, sql: &str) -> Result<(), DatabaseError> {
        let guard = self.conn.lock().await;

        guard.execute_batch(sql)?;

        Ok(())
    }

    async fn execute<P: Params>(&self, sql: &str, params: P) -> Result<usize, DatabaseError> {
        let guard = self.conn.lock().await;

        guard.execute(sql, params)?;

        Ok(0)
    }

    async fn query_map<P: Params, T, F>(
        &self,
        query: &str,
        params: P,
        row_to_type: F,
    ) -> Result<Vec<T>, DatabaseError>
    where
        F: Fn(&Row) -> Result<T, RowError>,
    {
        let guard = self.conn.lock().await;

        let mut stmt = guard.prepare(query)?;
        let rows: Result<Vec<T>, rusqlite::Error> =
            stmt.query_map(params, |row| row_to_type(row))?.collect();

        rows.map_err(DatabaseError::RowError)
    }
}

// From DatabaseError to rusqlite::Error
impl From<DatabaseError> for rusqlite::Error {
    fn from(err: DatabaseError) -> Self {
        match err {
            DatabaseError::Rusqlite(err) => err,
            _ => rusqlite::Error::QueryReturnedNoRows,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_version() {
        let db = SqliteDatabase::new_test();

        let version = db.get_version().await.unwrap();
        assert_eq!(version, 0);

        db.set_version(1).await.unwrap();
        let version = db.get_version().await.unwrap();
        assert_eq!(version, 1);
    }

    #[tokio::test]
    async fn test_execute_batch() {
        let db = SqliteDatabase::new_test();

        db.execute_batch("CREATE TABLE test (id INTEGER PRIMARY KEY, name TEXT)")
            .await
            .unwrap();

        db.execute_batch("INSERT INTO test (name) VALUES ('test')")
            .await
            .unwrap();

        let count: Vec<i64> = db
            .query_map("SELECT COUNT(*) FROM test", [], |row| row.get(0))
            .await
            .unwrap();

        assert_eq!(*count.first().unwrap(), 1);
    }

    #[tokio::test]
    async fn test_execute() {
        let db = SqliteDatabase::new_test();

        db.execute("CREATE TABLE test (id INTEGER PRIMARY KEY, name TEXT)", [])
            .await
            .unwrap();

        db.execute("INSERT INTO test (name) VALUES (?)", ["abc"])
            .await
            .unwrap();

        db.execute(
            "INSERT INTO test (name) VALUES (:name)",
            &[(":name", "one")],
        )
        .await
        .unwrap();

        #[derive(Debug, PartialEq)]
        struct Test {
            id: i64,
            name: String,
        }

        let rows: Vec<Test> = db
            .query_map("SELECT * FROM test", [], |row| {
                Ok(Test {
                    id: row.get(0)?,
                    name: row.get(1)?,
                })
            })
            .await
            .unwrap();

        assert_eq!(
            rows,
            vec![
                Test {
                    id: 1,
                    name: "abc".to_string()
                },
                Test {
                    id: 2,
                    name: "one".to_string()
                }
            ]
        );
    }
}
