use uuid::Uuid;

use crate::{models::folder::{FolderFromDisk, FolderToSave}, client::encryption_settings::EncryptionSettings, error::Result};



pub struct FolderView {
    id: Uuid,
    name: String,
    revision_date: String
}

impl FolderView {
    pub fn decrypt(folder: &FolderFromDisk, enc: &EncryptionSettings) -> Result<FolderView> {
        let name = enc.decrypt(folder.name(), None)?;
        Ok(FolderView {
            id: *folder.id(),
            name,
            revision_date: folder.revision_date().clone()
        })
    }

    pub fn encrypt(self, enc: &EncryptionSettings) -> Result<FolderToSave> {
        let name = enc.encrypt(&self.name.as_bytes(), None)?;
        Ok(FolderToSave {
            id: Some(self.id),
            name,
            revision_date: self.revision_date
        })
    }
}
