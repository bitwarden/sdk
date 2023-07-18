use std::{collections::HashMap, fmt::Debug};

use crate::error::Result;

use super::{StateMap, Storage};

#[derive(Debug)]
pub(super) struct InMemoryStorage {
    data: StateMap,
}

impl InMemoryStorage {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

#[async_trait::async_trait]
impl Storage for InMemoryStorage {
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
