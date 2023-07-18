use std::{collections::HashMap, fmt::Debug};

use serde_json::Value;

use crate::error::Result;

#[cfg(not(target_arch = "wasm32"))]
mod file_storage;
mod memory_storage;
#[cfg(target_arch = "wasm32")]
mod wasm_storage;

type StateMap = HashMap<String, HashMap<String, Value>>;

#[async_trait::async_trait]
pub(super) trait Storage: Sync + Send + Debug {
    async fn read(&mut self) -> Result<StateMap>;
    async fn modify<'b>(
        &mut self,
        f: Box<dyn for<'a> FnOnce(&'a mut StateMap) -> Result<()> + Send + 'b>,
    ) -> Result<StateMap>;
}

pub(super) fn initialize_storage(path: &Option<String>) -> Box<dyn Storage> {
    if let Some(path) = path {
        #[cfg(target_arch = "wasm32")]
        {
            Box::new(wasm_storage::WasmLocalStorage::new(path.to_owned()))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            Box::new(file_storage::FileStorage::new(path.into()))
        }
    } else {
        Box::new(memory_storage::InMemoryStorage::new())
    }
}
