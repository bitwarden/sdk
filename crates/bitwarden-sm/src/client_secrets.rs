use bitwarden_core::{Client, Error};

use crate::secrets::{
    create_secret, delete_secrets, get_secret, get_secrets_by_ids, list_secrets,
    list_secrets_by_project, sync_secrets, update_secret, SecretCreateRequest, SecretGetRequest,
    SecretIdentifiersByProjectRequest, SecretIdentifiersRequest, SecretIdentifiersResponse,
    SecretPutRequest, SecretResponse, SecretsDeleteRequest, SecretsDeleteResponse,
    SecretsGetRequest, SecretsResponse, SecretsSyncRequest, SecretsSyncResponse,
};

pub struct ClientSecrets<'a> {
    client: &'a Client,
}

impl<'a> ClientSecrets<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn get(&self, input: &SecretGetRequest) -> Result<SecretResponse, Error> {
        get_secret(self.client, input).await
    }

    pub async fn get_by_ids(&self, input: SecretsGetRequest) -> Result<SecretsResponse, Error> {
        get_secrets_by_ids(self.client, input).await
    }

    pub async fn create(&self, input: &SecretCreateRequest) -> Result<SecretResponse, Error> {
        create_secret(self.client, input).await
    }

    pub async fn list(
        &self,
        input: &SecretIdentifiersRequest,
    ) -> Result<SecretIdentifiersResponse, Error> {
        list_secrets(self.client, input).await
    }

    pub async fn list_by_project(
        &self,
        input: &SecretIdentifiersByProjectRequest,
    ) -> Result<SecretIdentifiersResponse, Error> {
        list_secrets_by_project(self.client, input).await
    }

    pub async fn update(&self, input: &SecretPutRequest) -> Result<SecretResponse, Error> {
        update_secret(self.client, input).await
    }

    pub async fn delete(
        &self,
        input: SecretsDeleteRequest,
    ) -> Result<SecretsDeleteResponse, Error> {
        delete_secrets(self.client, input).await
    }

    pub async fn sync(&self, input: &SecretsSyncRequest) -> Result<SecretsSyncResponse, Error> {
        sync_secrets(self.client, input).await
    }
}

pub trait ClientSecretsExt<'a> {
    fn secrets(&'a self) -> ClientSecrets<'a>;
}

impl<'a> ClientSecretsExt<'a> for Client {
    fn secrets(&'a self) -> ClientSecrets<'a> {
        ClientSecrets::new(self)
    }
}
