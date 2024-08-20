use bitwarden_core::Client;
pub use bitwarden_generators::{
    password as generate_secret, PasswordError as GenerateSecretError,
    PasswordGeneratorRequest as GenerateSecretRequest,
};

pub struct ClientGenerators<'a> {
    pub client: &'a Client,
}

impl<'a> ClientGenerators<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn generate_secret(
        &self,
        input: GenerateSecretRequest,
    ) -> Result<String, GenerateSecretError> {
        generate_secret(input)
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
