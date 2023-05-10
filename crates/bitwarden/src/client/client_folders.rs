use crate::{
    commands::{create_folder, delete_folder, update_folder},
    error::Result,
    sdk::request::folders_request::{
        FolderCreateRequest, FolderDeleteRequest, FolderUpdateRequest,
    },
};

pub struct ClientFolders<'a> {
    pub(crate) client: &'a mut crate::Client,
}

impl<'a> ClientFolders<'a> {
    pub async fn create(&mut self, input: FolderCreateRequest) -> Result<()> {
        create_folder(self.client, input).await
    }

    pub async fn update(&mut self, input: FolderUpdateRequest) -> Result<()> {
        update_folder(self.client, input).await
    }

    pub async fn delete(&mut self, input: FolderDeleteRequest) -> Result<()> {
        delete_folder(self.client, input).await
    }
}
