use std::{collections::HashMap, fmt::Debug, path::PathBuf};

use crate::error::Result;

use super::{StateMap, Storage};

#[derive(Debug)]
pub(super) struct FileStorage {
    path: PathBuf,
}

impl FileStorage {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

#[async_trait::async_trait]
impl Storage for FileStorage {
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
