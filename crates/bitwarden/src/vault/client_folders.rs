use super::folders::{
    create_folder, delete_folder, get_folder, list_folders, update_folder, FolderCreateRequest,
    FolderDeleteRequest, FolderGetRequest, FolderResponse, FolderUpdateRequest, FoldersResponse,
};
use crate::{error::Result, Client};

pub struct ClientFolders<'a> {
    pub(crate) client: &'a mut crate::Client,
}

impl<'a> ClientFolders<'a> {
    pub async fn create(&mut self, input: FolderCreateRequest) -> Result<()> {
        create_folder(self.client, input).await
    }

    pub async fn get(&self, input: FolderGetRequest) -> Result<FolderResponse> {
        get_folder(self.client, input).await
    }

    pub async fn list(&self) -> Result<FoldersResponse> {
        list_folders(self.client).await
    }

    pub async fn update(&mut self, input: FolderUpdateRequest) -> Result<()> {
        update_folder(self.client, input).await
    }

    pub async fn delete(&mut self, input: FolderDeleteRequest) -> Result<()> {
        delete_folder(self.client, input).await
    }
}

impl<'a> Client {
    pub fn folders(&'a mut self) -> ClientFolders<'a> {
        ClientFolders { client: self }
    }
}
