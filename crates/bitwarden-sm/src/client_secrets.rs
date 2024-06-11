use bitwarden_core::{Client, Error};

use crate::secrets::{
    create_secret, delete_secrets, get_secret, get_secrets_by_ids, list_secrets,
    list_secrets_by_project, sync_secrets, update_secret, SecretCreateRequest, SecretGetRequest,
    SecretIdentifiersByProjectRequest, SecretIdentifiersRequest, SecretIdentifiersResponse,
    SecretPutRequest, SecretResponse, SecretsDeleteRequest, SecretsDeleteResponse,
    SecretsGetRequest, SecretsResponse, SecretsSyncRequest, SecretsSyncResponse,
};

pub struct ClientSecrets<'a> {
    client: &'a mut Client,
}

impl<'a> ClientSecrets<'a> {
    pub fn new(client: &'a mut Client) -> Self {
        Self { client }
    }

    pub async fn get(&mut self, input: &SecretGetRequest) -> Result<SecretResponse, Error> {
        get_secret(self.client, input).await
    }

    pub async fn get_by_ids(&mut self, input: SecretsGetRequest) -> Result<SecretsResponse, Error> {
        get_secrets_by_ids(self.client, input).await
    }

    pub async fn create(&mut self, input: &SecretCreateRequest) -> Result<SecretResponse, Error> {
        create_secret(self.client, input).await
    }

    pub async fn list(
        &mut self,
        input: &SecretIdentifiersRequest,
    ) -> Result<SecretIdentifiersResponse, Error> {
        list_secrets(self.client, input).await
    }

    pub async fn list_by_project(
        &mut self,
        input: &SecretIdentifiersByProjectRequest,
    ) -> Result<SecretIdentifiersResponse, Error> {
        list_secrets_by_project(self.client, input).await
    }

    pub async fn update(&mut self, input: &SecretPutRequest) -> Result<SecretResponse, Error> {
        update_secret(self.client, input).await
    }

    pub async fn delete(
        &mut self,
        input: SecretsDeleteRequest,
    ) -> Result<SecretsDeleteResponse, Error> {
        delete_secrets(self.client, input).await
    }

    pub async fn sync(&mut self, input: &SecretsSyncRequest) -> Result<SecretsSyncResponse, Error> {
        sync_secrets(self.client, input).await
    }
}

pub trait ClientSecretsExt<'a> {
    fn secrets(&'a mut self) -> ClientSecrets<'a>;
}

impl<'a> ClientSecretsExt<'a> for Client {
    fn secrets(&'a mut self) -> ClientSecrets<'a> {
        ClientSecrets::new(self)
    }
}
