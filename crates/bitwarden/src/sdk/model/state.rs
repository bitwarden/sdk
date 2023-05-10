use std::{
    collections::HashMap,
    fmt::Debug,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use bitwarden_api_api::models::SyncResponseModel;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::{
    error::{Error, Result},
    sdk::request::client_settings::ClientSettings,
};

use super::domain::{copy_sync_to_domain, AccountData, GlobalData};

pub struct State {
    pub global: StateStorage<GlobalData>,
    pub account: StateStorage<AccountData>,
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("State").finish()
    }
}

impl State {
    pub(crate) fn load_state(settings: &ClientSettings) -> Self {
        let in_memory: Arc<dyn StateStorageMedium> = Arc::new(InMemoryStateStorageMedium::new());

        let medium: Arc<dyn StateStorageMedium> = if let Some(path) = &settings.state_path {
            #[cfg(target_arch = "wasm32")]
            {
                Arc::new(WasmLocalStorageMedium::new(path.to_owned()))
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                Arc::new(FileStateStorageMedium::new(path.into()))
            }
        } else {
            Arc::clone(&in_memory)
        };

        Self {
            global: StateStorage::new("global".into(), Arc::clone(&medium)),
            // The account storage will be in memory until the client is initialized
            account: StateStorage::new("temp".into(), in_memory),
        }
    }

    pub(crate) fn set_account_sync_data(
        &mut self,
        id: Uuid,
        data: SyncResponseModel,
    ) -> Result<()> {
        // Before we create the storage profile, keep a copy of the current temporary storage (tokens, kdf params, etc)
        let mut state = self.account.get().to_owned();

        self.account = StateStorage::new(id.to_string(), Arc::clone(&self.global.medium));

        copy_sync_to_domain(data, &mut state)?;

        self.account.save(state)
    }

    pub(crate) fn load_account(&mut self, id: Uuid) -> Result<AccountData> {
        self.account = StateStorage::new(id.to_string(), Arc::clone(&self.global.medium));
        self.account.read()
    }
}

pub struct StateStorage<T: Serialize + DeserializeOwned + Default + Clone> {
    cache: T,
    key: String,
    medium: Arc<dyn StateStorageMedium>,
}

impl<T: Serialize + DeserializeOwned + Default + Clone> StateStorage<T> {
    fn new(key: String, medium: Arc<dyn StateStorageMedium>) -> Self {
        Self {
            cache: T::default(),
            key,
            medium,
        }
    }

    pub fn get(&self) -> &T {
        &self.cache
    }

    pub fn read(&mut self) -> Result<T> {
        let value = if let Some(json) = self.medium.get(&self.key)? {
            serde_json::from_value::<T>(json)?
        } else {
            T::default()
        };

        self.cache = value.clone();
        Ok(value)
    }

    pub fn save(&mut self, value: T) -> Result<()> {
        self.cache = value.clone();
        self.medium.save(&self.key, serde_json::to_value(value)?)
    }

    pub fn delete(&mut self) -> Result<()> {
        self.medium.remove(&self.key)
    }

    pub fn has(&self, key: &str) -> Result<bool> {
        self.medium.has(key)
    }
}

trait StateStorageMedium: Sync + Send + Debug {
    fn get(&self, key: &str) -> Result<Option<Value>>;
    fn save(&self, key: &str, value: Value) -> Result<()>;
    fn remove(&self, key: &str) -> Result<()>;

    fn has(&self, key: &str) -> Result<bool> {
        Ok(self.get(key)?.is_some())
    }
}

#[derive(Debug)]
struct FileStateStorageMedium {
    path: PathBuf,
}

impl FileStateStorageMedium {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl StateStorageMedium for FileStateStorageMedium {
    fn get(&self, key: &str) -> Result<Option<Value>> {
        if !self.path.exists() {
            return Ok(None);
        }
        let data = std::fs::read_to_string(&self.path)?;

        Ok(serde_json::from_str::<Value>(&data)?
            .as_object()
            .ok_or(Error::Internal("State file is not a valid JSON document"))?
            .get(key)
            .cloned())
    }

    fn save(&self, key: &str, value: Value) -> Result<()> {
        let mut object = if self.path.exists() {
            let data = std::fs::read_to_string(&self.path)?;
            serde_json::from_str::<Value>(&data)?
        } else {
            Value::Object(serde_json::Map::new())
        };

        object
            .as_object_mut()
            .ok_or(Error::Internal("State file is not a valid JSON document"))?
            .insert(key.to_owned(), value);

        std::fs::write(&self.path, serde_json::to_string_pretty(&object)?)?;
        Ok(())
    }

    fn remove(&self, key: &str) -> Result<()> {
        let data = std::fs::read_to_string(&self.path)?;
        let mut object = serde_json::from_str::<Value>(&data)?;

        object
            .as_object_mut()
            .ok_or(Error::Internal("State file is not a valid JSON document"))?
            .remove(key);

        std::fs::write(&self.path, serde_json::to_string_pretty(&object)?)?;
        Ok(())
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

// TODO: Remove unwraps from the LocalStorage backend and replace with proper error handling
#[cfg(target_arch = "wasm32")]
impl StateStorageMedium for WasmLocalStorageMedium {
    fn get(&self, key: &str) -> Result<Option<Value>> {
        let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();

        let result = storage.get_item(&self.key).unwrap();

        let Some(item) = result else { return Ok(None) };

        Ok(serde_json::from_str::<Value>(&item)?
            .as_object()
            .ok_or(Error::Internal("State file is not a valid JSON document"))?
            .get(key)
            .cloned())
    }

    fn save(&self, key: &str, value: Value) -> Result<()> {
        let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();

        let mut object = if let Some(item) = storage.get_item(&self.key).unwrap() {
            serde_json::from_str::<Value>(&item)?
        } else {
            Value::Object(serde_json::Map::new())
        };

        object
            .as_object_mut()
            .ok_or(Error::Internal("State file is not a valid JSON document"))?
            .insert(key.to_owned(), value);

        storage
            .set_item(key, &serde_json::to_string_pretty(&object)?)
            .unwrap();

        Ok(())
    }

    fn remove(&self, key: &str) -> Result<()> {
        let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();

        let mut object = if let Some(item) = storage.get_item(&self.key).unwrap() {
            serde_json::from_str::<Value>(&item)?
        } else {
            Value::Object(serde_json::Map::new())
        };

        object
            .as_object_mut()
            .ok_or(Error::Internal("State file is not a valid JSON document"))?
            .remove(key);

        storage
            .set_item(key, &serde_json::to_string_pretty(&object)?)
            .unwrap();
        Ok(())
    }
}

#[derive(Debug)]
struct InMemoryStateStorageMedium {
    data: Mutex<HashMap<String, Value>>,
}

impl InMemoryStateStorageMedium {
    pub fn new() -> Self {
        Self {
            data: Mutex::new(HashMap::new()),
        }
    }
}

impl StateStorageMedium for InMemoryStateStorageMedium {
    fn get(&self, key: &str) -> Result<Option<Value>> {
        Ok(self.data.lock().unwrap().get(key).cloned())
    }

    fn save(&self, key: &str, value: Value) -> Result<()> {
        self.data.lock().unwrap().insert(key.to_owned(), value);
        Ok(())
    }

    fn remove(&self, key: &str) -> Result<()> {
        self.data.lock().unwrap().remove(key);
        Ok(())
    }
}
