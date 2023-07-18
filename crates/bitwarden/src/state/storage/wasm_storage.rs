use std::{collections::HashMap, fmt::Debug};

use crate::error::Result;

use super::{StateMap, Storage};

#[derive(Debug)]
pub(super) struct WasmLocalStorage {
    key: String,
}

impl WasmLocalStorage {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

#[async_trait::async_trait]
impl Storage for WasmLocalStorage {
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
