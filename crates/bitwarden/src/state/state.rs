use std::{collections::HashMap, fmt::Debug, path::PathBuf};

use async_lock::Mutex;
use bitwarden_api_api::models::SyncResponseModel;
use serde_json::Value;
use uuid::Uuid;

use crate::{
    client::client_settings::ClientSettings,
    error::{Error, Result},
};

pub struct State {
    pub(crate) account: Mutex<StateStorage>,
    path: Option<String>,
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("State").finish()
    }
}

impl State {
    pub(crate) fn load_state(settings: &ClientSettings) -> Self {
        Self {
            // The account storage will be in memory until the client is initialized
            account: Mutex::new(StateStorage::new(String::new(), load_medium(&None))),
            path: settings.state_path.clone(),
        }
    }

    #[cfg(feature = "internal")]
    pub(crate) async fn set_account_sync_data(
        &self,
        id: Uuid,
        data: SyncResponseModel,
    ) -> Result<()> {
        // Before we create the storage profile, keep a copy of the current temporary storage (tokens, kdf params, etc)

        use crate::client::{keys::store_keys_from_sync, profile::store_profile_from_sync};
        let state = self.account.lock().await.get();

        // Create the new account state, and load the temporary storage into it
        let mut account = self.account.lock().await;
        *account = StateStorage::new(id.to_string(), load_medium(&self.path));
        account
            .modify(|acc| {
                *acc = Some(state);
                Ok(())
            })
            .await?;
        drop(account);

        // Save the new data
        let profile = data.profile.ok_or(Error::MissingFields)?;

        store_keys_from_sync(profile.as_ref(), self).await?;
        store_profile_from_sync(profile.as_ref(), self).await?;

        Ok(())
    }

    pub(crate) async fn load_account(&self, id: Uuid) -> Result<()> {
        let mut account = self.account.lock().await;
        *account = StateStorage::new(id.to_string(), load_medium(&self.path));
        account.read().await?;
        Ok(())
    }
}

pub struct StateStorage {
    cache: HashMap<String, Value>,
    account_id: String,
    medium: Box<dyn StateStorageMedium>,
}

impl StateStorage {
    fn new(account_id: String, medium: Box<dyn StateStorageMedium>) -> Self {
        Self {
            cache: HashMap::new(),
            account_id,
            medium,
        }
    }

    pub fn get(&self) -> HashMap<String, Value> {
        self.cache.clone()
    }

    pub async fn read(&mut self) -> Result<HashMap<String, Value>> {
        let value = self
            .medium
            .read()
            .await?
            .remove(&self.account_id)
            .unwrap_or_default();
        self.cache = value.clone();
        Ok(value)
    }

    pub async fn modify<'b>(
        &mut self,
        modify_fn: impl FnOnce(&mut Option<HashMap<String, Value>>) -> Result<()> + Send + 'b,
    ) -> Result<()> {
        let account_id = self.account_id.clone();

        let result = self
            .medium
            .modify(Box::new(move |state| {
                let mut map = state.remove(&account_id);
                modify_fn(&mut map)?;
                if let Some(map) = map {
                    state.insert(account_id, map);
                }

                Ok(())
            }))
            .await?;

        self.cache = result.get(&self.account_id).cloned().unwrap_or_default();
        Ok(())
    }
}

fn load_medium(path: &Option<String>) -> Box<dyn StateStorageMedium> {
    if let Some(path) = path {
        #[cfg(target_arch = "wasm32")]
        {
            Box::new(WasmLocalStorageMedium::new(path.to_owned()))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            Box::new(FileStateStorageMedium::new(path.into()))
        }
    } else {
        Box::new(InMemoryStateStorageMedium::new())
    }
}

type StateMap = HashMap<String, HashMap<String, Value>>;

#[async_trait::async_trait]
trait StateStorageMedium: Sync + Send + Debug {
    async fn read(&mut self) -> Result<StateMap>;
    async fn modify<'b>(
        &mut self,
        f: Box<dyn for<'a> FnOnce(&'a mut StateMap) -> Result<()> + Send + 'b>,
    ) -> Result<StateMap>;
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug)]
struct FileStateStorageMedium {
    path: PathBuf,
}

#[cfg(not(target_arch = "wasm32"))]
impl FileStateStorageMedium {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[async_trait::async_trait]
impl StateStorageMedium for FileStateStorageMedium {
    async fn read(&mut self) -> Result<StateMap> {
        if !self.path.exists() {
            return Ok(HashMap::new());
        }
        let data = tokio::fs::read_to_string(&self.path).await?;

        Ok(serde_json::from_str(&data)?)
    }

    async fn modify<'b>(
        &mut self,
        modify_fn: Box<dyn for<'a> FnOnce(&'a mut StateMap) -> Result<()> + Send + 'b>,
    ) -> Result<StateMap> {
        use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};

        let mut f = tokio::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.path)
            .await?;

        // Try locking the file so no other process modifies it at the same time
        use fs4::tokio::AsyncFileExt;
        f.lock_exclusive()?;

        let mut object_string = String::new();
        let read_count = f.read_to_string(&mut object_string).await?;

        let mut object: StateMap = if read_count == 0 {
            // File was just created, use an empty map
            HashMap::new()
        } else {
            serde_json::from_str(&object_string)?
        };

        modify_fn(&mut object)?;

        // Truncate the file and overwrite
        f.rewind().await?;
        f.set_len(0).await?;
        f.write_all(serde_json::to_string_pretty(&object)?.as_bytes())
            .await?;

        Ok(object.clone())
    }
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug)]
struct WasmLocalStorageMedium {
    key: String,
}

#[cfg(target_arch = "wasm32")]
impl WasmLocalStorageMedium {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

#[cfg(target_arch = "wasm32")]
#[async_trait::async_trait]
impl StateStorageMedium for WasmLocalStorageMedium {
    async fn read(&mut self) -> Result<StateMap> {
        let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();

        let result = storage.get_item(&self.key).unwrap();
        let Some(item) = result else { return Ok(HashMap::new()) };

        Ok(serde_json::from_str(&item)?)
    }

    async fn modify<'b>(
        &mut self,
        modify_fn: Box<dyn for<'a> FnOnce(&'a mut StateMap) -> Result<()> + Send + 'b>,
    ) -> Result<StateMap> {
        let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();

        let mut object = if let Some(item) = storage.get_item(&self.key).unwrap() {
            serde_json::from_str(&item)?
        } else {
            HashMap::new()
        };

        modify_fn(&mut object)?;

        storage
            .set_item(&self.key, &serde_json::to_string_pretty(&object)?)
            .unwrap();

        Ok(object.clone())
    }
}

#[derive(Debug)]
struct InMemoryStateStorageMedium {
    data: StateMap,
}

impl InMemoryStateStorageMedium {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

#[async_trait::async_trait]
impl StateStorageMedium for InMemoryStateStorageMedium {
    async fn read(&mut self) -> Result<StateMap> {
        Ok(self.data.clone())
    }

    async fn modify<'b>(
        &mut self,
        modify_fn: Box<dyn for<'a> FnOnce(&'a mut StateMap) -> Result<()> + Send + 'b>,
    ) -> Result<StateMap> {
        modify_fn(&mut self.data)?;
        Ok(self.data.clone())
    }
}
