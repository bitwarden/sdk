use crate::{
    error::Result,
    tool::generators::{
        password::{passphrase, password, PassphraseGeneratorRequest, PasswordGeneratorRequest},
        username::{username, UsernameGeneratorRequest},
    },
    Client,
};

pub struct ClientGenerator<'a> {
    pub(crate) _client: &'a crate::Client,
}

impl<'a> ClientGenerator<'a> {
    pub async fn password(&self, input: PasswordGeneratorRequest) -> Result<String> {
        password(input)
    }

    pub async fn passphrase(&self, input: PassphraseGeneratorRequest) -> Result<String> {
        passphrase(input)
    }

    pub async fn username(&self, input: UsernameGeneratorRequest) -> Result<String> {
        username(input).await
    }
}

impl<'a> Client {
    pub fn generator(&'a self) -> ClientGenerator<'a> {
        ClientGenerator { _client: self }
    }
}
