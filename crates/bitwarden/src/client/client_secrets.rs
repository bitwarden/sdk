use crate::{
    error::Result,
    secrets_manager::secrets::{
        create_secret, delete_secrets, get_secret, list_secrets, list_secrets_by_project,
        update_secret, SecretCreateRequest, SecretGetRequest, SecretIdentifiersByProjectRequest,
        SecretIdentifiersRequest, SecretIdentifiersResponse, SecretPutRequest, SecretResponse,
        SecretsDeleteRequest, SecretsDeleteResponse,
    },
};

pub struct ClientSecrets<'a> {
    pub(crate) client: &'a mut crate::Client,
}

impl<'a> ClientSecrets<'a> {
    pub async fn get(&mut self, input: &SecretGetRequest) -> Result<SecretResponse> {
        get_secret(self.client, input).await
    }

    pub async fn create(&mut self, input: &SecretCreateRequest) -> Result<SecretResponse> {
        create_secret(self.client, input).await
    }

    pub async fn list(
        &mut self,
        input: &SecretIdentifiersRequest,
    ) -> Result<SecretIdentifiersResponse> {
        list_secrets(self.client, input).await
    }

    pub async fn list_by_project(
        &mut self,
        input: &SecretIdentifiersByProjectRequest,
    ) -> Result<SecretIdentifiersResponse> {
        list_secrets_by_project(self.client, input).await
    }

    pub async fn update(&mut self, input: &SecretPutRequest) -> Result<SecretResponse> {
        update_secret(self.client, input).await
    }

    pub async fn delete(&mut self, input: SecretsDeleteRequest) -> Result<SecretsDeleteResponse> {
        delete_secrets(self.client, input).await
    }
}
