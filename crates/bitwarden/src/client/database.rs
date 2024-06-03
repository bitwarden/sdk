use log::info;
use rusqlite::Connection;

use crate::error::Error;

#[derive(Debug)]
pub struct SqliteDatabase {
    pub conn: Connection,
}

impl SqliteDatabase {
    pub fn default() -> Self {
        let conn = Connection::open("test.sqlite").expect("Failed to open sqlite connection");

        Self::new_conn(conn)
    }

    pub fn new_test() -> Self {
        let conn = Connection::open_in_memory().expect("Failed to open sqlite connection");

        Self::new_conn(conn)
    }

    /// Create a new SqliteDatabase with the given connection.
    ///
    /// This will migrate the database to the latest version.
    fn new_conn(conn: Connection) -> Self {
        migrate(&conn);

        SqliteDatabase { conn }
    }
}

/// Migrate the database to the latest version
///
/// The current database version is stored in the user_version PRAGMA.
/// It will iterate through all migrations and apply up migrations.
fn migrate(conn: &Connection) {
    info!("Migrating database");
    let user_version = user_version(conn).expect("Failed to get user_version");
    info!("Current database version: {}", user_version);

    let migrations = MIGRATIONS.iter().skip(user_version as usize);
    for (i, migration) in migrations.enumerate() {
        info!("Applying migration: {}, {}", i, migration.description);
        conn.execute_batch(migration.up)
            .expect("Failed to apply migration");
    }

    set_user_version(conn, MIGRATIONS.len() as i32).expect("Failed to set user_version");
    info!("Migrations complete");
}

struct Migration {
    /// A description of the migration, used for logging
    description: &'static str,
    /// The SQL to run when migrating up
    up: &'static str,
    /// The SQL to run when migrating down
    down: &'static str,
}

const MIGRATIONS: &[Migration] = &[Migration {
    description: "Create ciphers table",
    up: "CREATE TABLE IF NOT EXISTS ciphers (
            id TEXT PRIMARY KEY,
            value TEXT NOT NULL
        )",
    down: "DROP TABLE ciphers",
}];

/// Get the user_version of the database
fn user_version(conn: &Connection) -> Result<i32, Error> {
    conn.query_row("PRAGMA user_version", [], |row| row.get(0))
        .map_err(|e| e.into())
}

/// Set the user_version of the database
fn set_user_version(conn: &Connection, version: i32) -> Result<(), Error> {
    conn.pragma_update(None, "user_version", version)?;

    Ok(())
}
