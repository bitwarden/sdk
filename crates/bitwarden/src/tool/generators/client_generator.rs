use crate::{
    error::Result,
    tool::generators::passphrase::{passphrase, PassphraseGeneratorRequest},
    tool::generators::password::{password, PasswordGeneratorRequest},
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
}

impl<'a> Client {
    pub fn generator(&'a self) -> ClientGenerator<'a> {
        ClientGenerator { _client: self }
    }
}
