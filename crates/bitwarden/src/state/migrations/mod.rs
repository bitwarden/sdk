use log::info;
use serde_json::{Map, Value};

use crate::{
    error::{Error, MigrationError, Result},
    Client,
};

mod v1_initial_migration;
mod v2_rename_profile_email;

pub mod latest {
    use super::v2_rename_profile_email::*;
    pub type State = StateV2;
    pub type Keys = KeysV2;
    pub type Profile = ProfileV2;
    pub type Cipher = CipherV2;
    pub type Folder = FolderV2;
    pub type Auth = AuthV2;
}

// This represents a single migration, from one version to the next, that means TO = {FROM + 1}
// All migrations must implement this trait and its two functions `migrate` and `rollback`.
// Note that `try_migrate` and `try_rollback` are already implemented and should almost never be overridden by specific migrations.
// Ideally we can remove the TO generic bound when [generic_const_exprs](https://github.com/rust-lang/rust/issues/76560) is stabilized
trait Migration<const FROM: usize, const TO: usize> {
    type Input: serde::Serialize + serde::de::DeserializeOwned;
    type Output: serde::Serialize + serde::de::DeserializeOwned;

    fn migrate(&self, input: Self::Input) -> Result<Self::Output>;
    fn rollback(&self, input: Self::Output) -> Result<Self::Input>;

    fn try_migrate(
        &self,
        input: Map<String, Value>,
        version: &mut usize,
    ) -> Result<Map<String, Value>> {
        if *version == TO {
            info!("Skipping migration from {} to {}", FROM, TO);
            return Ok(input);
        }

        if *version == FROM {
            info!("Running migration from {} to {}", FROM, TO);

            let input = serde_json::from_value(Value::Object(input))?;
            let output = self.migrate(input)?;
            let output = serde_json::to_value(output)?;

            match output {
                Value::Object(output) => {
                    *version = TO;
                    return Ok(output);
                }
                _ => return Err(Error::Internal("Migration doesn't return an object")),
            }
        }

        Err(Error::Migration(MigrationError::InvalidVersion {
            current: *version,
            from: FROM,
            to: TO,
        }))
    }

    fn try_rollback(
        &self,
        input: Map<String, Value>,
        version: &mut usize,
        target_version: usize,
    ) -> Result<Map<String, Value>> {
        if *version == FROM {
            info!("Skipping rollback from {} to {}", TO, FROM);
            return Ok(input);
        }

        if *version == TO && target_version <= FROM {
            info!("Running rollback from {} to {}", TO, FROM);

            let input = serde_json::from_value(Value::Object(input))?;
            let output = self.rollback(input)?;
            let output = serde_json::to_value(output)?;

            match output {
                Value::Object(output) => {
                    *version = FROM;
                    return Ok(output);
                }
                _ => return Err(Error::Internal("Migration doesn't return an object")),
            }
        }

        Err(Error::Migration(MigrationError::InvalidVersionRollback {
            current: *version,
            from: TO,
            to: FROM,
        }))
    }
}

// This represents a set of migrations, and can contain itself, this way we can have a ChainedMigration<ChainedMigration, Migration<>>
struct ChainedMigration<
    T: Migration<FROM, MID>,
    U: Migration<MID, TO>,
    const FROM: usize,
    const MID: usize,
    const TO: usize,
> {
    first: T,
    second: U,
}

impl<
        T: Migration<FROM, MID>,
        U: Migration<MID, TO, Input = T::Output>,
        const FROM: usize,
        const MID: usize,
        const TO: usize,
    > Migration<FROM, TO> for ChainedMigration<T, U, FROM, MID, TO>
{
    type Input = T::Input;

    type Output = U::Output;

    fn migrate(&self, input: Self::Input) -> Result<Self::Output> {
        let first = self.first.migrate(input)?;
        let second = self.second.migrate(first)?;
        Ok(second)
    }

    fn rollback(&self, input: Self::Output) -> Result<Self::Input> {
        let first = self.second.rollback(input)?;
        let second = self.first.rollback(first)?;
        Ok(second)
    }

    fn try_migrate(
        &self,
        mut state: Map<String, Value>,
        version: &mut usize,
    ) -> Result<Map<String, Value>> {
        // Run the first half of the migrations if we're in range
        if *version >= FROM && *version < MID {
            state = self.first.try_migrate(state, version)?;
        }

        // Run the second half of the migrations if we're in range, this will also run if
        // the first half of the migrations succeeded, which would update the version to MID
        if *version >= MID && *version < TO {
            state = self.second.try_migrate(state, version)?;
        }

        // If we're at the end of the migrations, return the migrated state
        if *version == TO {
            return Ok(state);
        }

        // If we're not at the end of the migrations, we're probably outside the migration range, so return an error
        return Err(Error::Migration(MigrationError::InvalidVersion {
            current: *version,
            from: FROM,
            to: TO,
        }));
    }

    fn try_rollback(
        &self,
        mut state: Map<String, Value>,
        version: &mut usize,
        target_version: usize,
    ) -> Result<Map<String, Value>> {
        // Rollback the second half of the migrations if we're in range
        if *version <= TO && *version > MID {
            state = self.second.try_rollback(state, version, target_version)?;
        }

        // Rollback the first half of the migrations if we're in range, this will also run if
        // the second half of the migrations succeeded, which would update the version to MID
        if *version <= MID && *version > target_version {
            state = self.first.try_rollback(state, version, target_version)?;
        }

        // If we're at the end of the migrations, return the migrated state
        if *version == target_version {
            return Ok(state);
        }

        // If we're not at the end of the migrations, we're probably outside the migration range, so return an error
        return Err(Error::Migration(MigrationError::InvalidVersion {
            current: *version,
            from: TO,
            to: FROM,
        }));
    }
}

struct MigrationManager<T: Migration<FROM, TO>, const FROM: usize, const TO: usize> {
    migration: Box<T>,
}

impl<T: Migration<0, 1>> MigrationManager<T, 0, 1> {
    fn new(migration: T) -> MigrationManager<T, 0, 1> {
        MigrationManager {
            migration: Box::new(migration),
        }
    }
}

impl<T: Migration<FROM, TO>, const FROM: usize, const TO: usize> MigrationManager<T, FROM, TO> {
    fn chain<
        U: Migration<TO, NEW_TO, Input = T::Output, Output = NewOutput>,
        const NEW_TO: usize,
        NewOutput,
    >(
        self,
        migration: U,
    ) -> MigrationManager<ChainedMigration<T, U, FROM, TO, NEW_TO>, FROM, NEW_TO> {
        MigrationManager {
            migration: Box::new(ChainedMigration {
                first: *self.migration,
                second: migration,
            }),
        }
    }
}

#[allow(dead_code)]
pub async fn run_migrations(client: &Client, mut version: usize) -> Result<()> {
    let migrations = MigrationManager::new(v1_initial_migration::MigrationV1)
        .chain(v2_rename_profile_email::MigrationV2);

    client
        .state
        .account
        .lock()
        .await
        .modify(|state| {
            let old = state.take().unwrap_or_default();
            let result = migrations.migration.try_migrate(old, &mut version);
            *state = Some(result?);
            Ok(())
        })
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_migration() {
        let migration_manager = MigrationManager::new(v1_initial_migration::MigrationV1)
            .chain(v2_rename_profile_email::MigrationV2);

        let mut version = 0;
        let result = migration_manager
            .migration
            .try_migrate(Default::default(), &mut version)
            .unwrap();

        assert_eq!(version, 2);
        assert!(result.contains_key("ciphers"));

        let result = migration_manager
            .migration
            .try_rollback(result, &mut version, 1)
            .unwrap();

        assert_eq!(version, 1);
    }
}
