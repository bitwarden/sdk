use bitwarden_core::Client;

use crate::generators::{
    generate_secret, GenerateSecretError, GenerateSecretRequest, GenerateSecretResponse,
};

pub struct ClientGenerators<'a> {
    pub client: &'a Client,
}

impl<'a> ClientGenerators<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn generate(
        &self,
        input: GenerateSecretRequest,
    ) -> Result<GenerateSecretResponse, GenerateSecretError> {
        generate_secret(input).await
    }
}

pub trait ClientGeneratorsExt<'a> {
    fn generators(&'a self) -> ClientGenerators<'a>;
}

impl<'a> ClientGeneratorsExt<'a> for Client {
    fn generators(&'a self) -> ClientGenerators<'a> {
        ClientGenerators::new(self)
    }
}
