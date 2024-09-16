use std::{borrow::Cow, cmp::Ordering};

use log::info;
use thiserror::Error;

use crate::{Database, DatabaseError, DatabaseTrait};

#[derive(Debug, Error)]
pub enum MigratorError {
    #[error("Internal error: {0}")]
    Internal(Cow<'static, str>),

    #[error("Failed to apply migrations")]
    MigrationFailed,

    #[error(transparent)]
    Database(#[from] DatabaseError),
}

/// Database migrator
///
/// The current database version is stored in the user_version PRAGMA.
/// It will iterate through all migrations and apply up migrations.
pub struct Migrator {
    migrations: Vec<Migration>,
}

impl Migrator {
    pub fn new(migrations: &[Migration]) -> Self {
        Self {
            migrations: migrations.to_vec(),
        }
    }

    /// Migrate the database to the target version, or the last migration if not specified
    pub async fn migrate(
        &self,
        db: &Database,
        target_version: Option<usize>,
    ) -> Result<(), MigratorError> {
        let current_version = db.get_version().await?;

        let target_version = target_version.unwrap_or(self.migrations.len());

        let migrations = filter_migrations(&self.migrations, current_version, target_version);

        info!(
            "Migrating database. Current version: {}, Target version: {}",
            current_version, target_version
        );

        for migration in migrations {
            info!("Applying migration: {}", migration.description);

            if current_version < target_version {
                db.execute_batch(migration.up)
                    .await
                    .map_err(|_| MigratorError::MigrationFailed)?;
            } else {
                db.execute_batch(migration.down)
                    .await
                    .map_err(|_| MigratorError::MigrationFailed)?;
            }
        }

        db.set_version(target_version).await?;

        Ok(())
    }
}

/// Filter the migrations to apply based on the current and target version
fn filter_migrations(
    migrations: &[Migration],
    current_version: usize,
    target_version: usize,
) -> Vec<&Migration> {
    match current_version.cmp(&target_version) {
        Ordering::Less => migrations
            .iter()
            .skip(current_version)
            .take(target_version - current_version)
            .collect(),
        Ordering::Greater => migrations
            .iter()
            .skip(target_version)
            .take(current_version - target_version)
            .rev()
            .collect(),
        Ordering::Equal => Vec::new(),
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Migration {
    /// A description of the migration, used for logging
    pub description: &'static str,
    /// The SQL to run when migrating up
    pub up: &'static str,
    /// The SQL to run when migrating down
    pub down: &'static str,
}

#[cfg(test)]
mod tests {

    use super::*;

    const MIGRATIONS: &[Migration] = &[
        Migration {
            description: "Create ciphers table",
            up: "CREATE TABLE IF NOT EXISTS ciphers (
                id TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            down: "DROP TABLE ciphers",
        },
        Migration {
            description: "Create folders table",
            up: "CREATE TABLE IF NOT EXISTS folders (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL
            )",
            down: "DROP TABLE folders",
        },
        Migration {
            description: "Create collections table",
            up: "CREATE TABLE IF NOT EXISTS collections (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL
            )",
            down: "DROP TABLE collections",
        },
    ];

    #[test]
    fn test_filter_migrations() {
        let result = filter_migrations(MIGRATIONS, 0, 3);

        assert_eq!(result[0].description, "Create ciphers table");
        assert_eq!(result[1].description, "Create folders table");
        assert_eq!(result[2].description, "Create collections table");
    }

    #[test]
    fn test_filter_migrations_less() {
        let result = filter_migrations(MIGRATIONS, 1, 2);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].description, "Create folders table");
    }

    #[test]
    fn test_filter_migrations_greater() {
        let result = filter_migrations(MIGRATIONS, 2, 0);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].description, "Create folders table");
        assert_eq!(result[1].description, "Create ciphers table");
    }

    #[test]
    fn test_filter_migrations_equal() {
        let result = filter_migrations(MIGRATIONS, 1, 1);
        assert_eq!(result.len(), 0);
    }
}
