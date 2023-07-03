use std::collections::HashMap;

use log::info;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::{
    client::encryption_settings::EncryptionSettings,
    error::{Error, MigrationError, Result},
    Client,
};

use super::state_service::ServiceDefinition;

mod v1_rename_profile_email;

// This is a macro because MigrationManager's is full of generics and it's a pain to write in a function
macro_rules! migration_manager {
    () => {
        MigrationManager::new(v1_rename_profile_email::MigrationV1)
        // .chain(v2_another_migration::MigrationV2)
    };
}

pub const LATEST_VERSION: usize = migration_manager!().latest_version();

// Wrapper around an object that allows us to ignore extra fields when deserializing but still keep them around, note that this only works for objects and is not recursive
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "T: Serialize + DeserializeOwned")]
pub(super) struct Partial<T: 'static + Serialize + DeserializeOwned> {
    #[serde(flatten)]
    content: T,
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

// This represents a single migration, from one version to the next, that means TO = {FROM + 1}
// All migrations must implement this trait and its two functions `migrate` and `rollback`.
// Note that `try_migrate` and `try_rollback` are already implemented and should almost never be overridden by specific migrations.
// Ideally we can remove the TO generic bound when [generic_const_exprs](https://github.com/rust-lang/rust/issues/76560) is stabilized
trait Migration<const FROM: usize, const TO: usize>: Clone + Copy + Send + Sync + 'static {
    type Input: Serialize + DeserializeOwned;
    type Output: Serialize + DeserializeOwned;

    fn migrate(&self, input: Self::Input, enc: &EncryptionSettings) -> Result<Self::Output>;
    fn rollback(&self, input: Self::Output, enc: &EncryptionSettings) -> Result<Self::Input>;

    fn try_migrate(
        &self,
        input: Map<String, Value>,
        enc: &EncryptionSettings,
        version: &mut usize,
    ) -> Result<Map<String, Value>> {
        if *version == TO {
            info!("Skipping migration from {} to {}", FROM, TO);
            return Ok(input);
        }

        if *version == FROM {
            info!("Running migration from {} to {}", FROM, TO);

            let input: Partial<_> = serde_json::from_value(Value::Object(input))?;
            let output = self.migrate(input.content, enc)?;
            let output = serde_json::to_value(Partial {
                content: output,
                extra: input.extra, // Pass through any extra fields not handled by the migrations
            })?;

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
        enc: &EncryptionSettings,
        version: &mut usize,
        target_version: usize,
    ) -> Result<Map<String, Value>> {
        if *version == FROM {
            info!("Skipping rollback from {} to {}", TO, FROM);
            return Ok(input);
        }

        if *version == TO && target_version <= FROM {
            info!("Running rollback from {} to {}", TO, FROM);

            let input: Partial<_> = serde_json::from_value(Value::Object(input))?;
            let output = self.rollback(input.content, enc)?;
            let output = serde_json::to_value(Partial {
                content: output,
                extra: input.extra, // Pass through any extra fields not handled by the migrations
            })?;

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

// This represents a set of migrations, and it implements Migration itself, this way we can have a ChainedMigration<ChainedMigration, Migration>
#[derive(Clone, Copy, Debug)]
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
        U: Migration<MID, TO>,
        const FROM: usize,
        const MID: usize,
        const TO: usize,
    > Migration<FROM, TO> for ChainedMigration<T, U, FROM, MID, TO>
{
    type Input = T::Input;

    type Output = U::Output;

    fn migrate(&self, _: Self::Input, _: &EncryptionSettings) -> Result<Self::Output> {
        unreachable!("This is never called on ChainedMigration, use try_migrate instead")
    }

    fn rollback(&self, _: Self::Output, _: &EncryptionSettings) -> Result<Self::Input> {
        unreachable!("This is never called on ChainedMigration, use try_rollback instead")
    }

    fn try_migrate(
        &self,
        mut state: Map<String, Value>,
        enc: &EncryptionSettings,
        version: &mut usize,
    ) -> Result<Map<String, Value>> {
        // Run the first half of the migrations if we're in range
        if *version >= FROM && *version < MID {
            state = self.first.try_migrate(state, enc, version)?;
        }

        // Run the second half of the migrations if we're in range, this will also run if
        // the first half of the migrations succeeded, which would update the version to MID
        if *version >= MID && *version < TO {
            state = self.second.try_migrate(state, enc, version)?;
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
        enc: &EncryptionSettings,
        version: &mut usize,
        target: usize,
    ) -> Result<Map<String, Value>> {
        // Rollback the second half of the migrations if we're in range
        if *version <= TO && *version > MID {
            state = self.second.try_rollback(state, enc, version, target)?;
        }

        // Rollback the first half of the migrations if we're in range, this will also run if
        // the second half of the migrations succeeded, which would update the version to MID
        if *version <= MID && *version > target {
            state = self.first.try_rollback(state, enc, version, target)?;
        }

        // If we're at the end of the migrations, return the migrated state
        if *version == target {
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
    migration: T,
}

impl<T: Migration<0, 1>> MigrationManager<T, 0, 1> {
    const fn new(migration: T) -> MigrationManager<T, 0, 1> {
        MigrationManager {
            migration: migration,
        }
    }
}

impl<T: Migration<FROM, TO>, const FROM: usize, const TO: usize> MigrationManager<T, FROM, TO> {
    #[allow(unused)]
    const fn chain<U: Migration<TO, NEW_TO, Output = NewOutput>, const NEW_TO: usize, NewOutput>(
        self,
        migration: U,
    ) -> MigrationManager<ChainedMigration<T, U, FROM, TO, NEW_TO>, FROM, NEW_TO> {
        let result = MigrationManager {
            migration: ChainedMigration {
                first: self.migration,
                second: migration,
            },
        };
        result
    }

    fn run(
        &self,
        input: Map<String, Value>,
        enc: &EncryptionSettings,
        version: &mut usize,
    ) -> Result<Map<String, Value>> {
        self.migration.try_migrate(input, enc, version)
    }

    const fn latest_version(&self) -> usize {
        TO
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Migrations {
    version: usize,
}

impl Default for Migrations {
    fn default() -> Self {
        Self {
            version: LATEST_VERSION,
        }
    }
}

const MIGRATION_SERVICE: ServiceDefinition<Migrations> = ServiceDefinition::new("migrations");

impl Migrations {
    pub async fn get(client: &Client) -> Self {
        client.get_state_service(MIGRATION_SERVICE).get().await
    }

    pub async fn is_latest(client: &Client) -> bool {
        Self::get(client).await.version == LATEST_VERSION
    }

    pub async fn set_version(client: &Client, version: usize) -> Result<()> {
        client
            .get_state_service(MIGRATION_SERVICE)
            .modify(|m| {
                m.version = version;
                Ok(())
            })
            .await
    }

    pub async fn set_version_latest_if_missing(client: &Client) -> Result<()> {
        client
            .get_state_service(MIGRATION_SERVICE)
            .modify_opt(|m| {
                if m.is_none() {
                    *m = Some(Migrations {
                        version: LATEST_VERSION,
                    });
                }
                Ok(())
            })
            .await
    }
}

pub async fn run_migrations(client: &Client) -> Result<()> {
    let enc = client
        .get_encryption_settings()
        .as_ref()
        .ok_or(Error::VaultLocked)?;

    let mut version = Migrations::get(client).await.version;

    if version != LATEST_VERSION {
        let result = client
            .state
            .account
            .lock()
            .await
            .modify(|state| {
                let old = state.take().unwrap_or_default();
                let result = migration_manager!().run(old, enc, &mut version);
                *state = Some(result?);
                Ok(())
            })
            .await;

        Migrations::set_version(client, version).await?;

        // This result is handled after the set_version in case we have run some migrations successfully but not all
        result?;
    }

    Ok(())
}

#[cfg(test)]
pub(crate) fn json_map(v: Value) -> Map<String, Value> {
    match v {
        Value::Object(map) => map,
        _ => panic!("Expected object"),
    }
}
