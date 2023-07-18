use std::{collections::HashMap, fmt::Debug};

use async_lock::Mutex;
use serde_json::Value;
#[cfg(feature = "internal")]
use uuid::Uuid;

use crate::{client::client_settings::ClientSettings, error::Result};

#[cfg(feature = "internal")]
use crate::error::Error;

use super::storage::{initialize_storage, Storage};

pub struct State {
    pub(crate) account: Mutex<StateStorage>,
    #[cfg(feature = "internal")]
    path: Option<String>,
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("State").finish()
    }
}

impl State {
    pub(crate) fn initialize_state(_settings: &ClientSettings) -> Self {
        Self {
            // The account storage will be in memory until the client is initialized
            account: Mutex::new(StateStorage::new(String::new(), initialize_storage(&None))),
            #[cfg(feature = "internal")]
            path: _settings.state_path.clone(),
        }
    }

    #[cfg(feature = "internal")]
    pub(crate) async fn set_account_sync_data(
        &self,
        id: Uuid,
        data: bitwarden_api_api::models::SyncResponseModel,
    ) -> Result<()> {
        // Before we create the storage profile, keep a copy of the current temporary storage (tokens, kdf params, etc)

        use crate::client::{keys::store_keys_from_sync, profile::store_profile_from_sync};
        let state = self.account.lock().await.get();

        // Create the new account state, and load the temporary storage into it
        let mut account = self.account.lock().await;
        *account = StateStorage::new(id.to_string(), initialize_storage(&self.path));
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

    #[cfg(feature = "internal")]
    pub(crate) async fn load_account(&self, id: Uuid) -> Result<()> {
        let mut account = self.account.lock().await;
        *account = StateStorage::new(id.to_string(), initialize_storage(&self.path));
        account.read().await?;
        Ok(())
    }
}

pub struct StateStorage {
    cache: HashMap<String, Value>,
    account_id: String,
    medium: Box<dyn Storage>,
}

impl StateStorage {
    fn new(account_id: String, medium: Box<dyn Storage>) -> Self {
        Self {
            cache: HashMap::new(),
            account_id,
            medium,
        }
    }

    pub fn get(&self) -> HashMap<String, Value> {
        self.cache.clone()
    }

    #[cfg(feature = "internal")]
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
