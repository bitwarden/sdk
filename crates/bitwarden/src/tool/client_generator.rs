use super::password::{PassphraseGeneratorRequest, PasswordGeneratorRequest};
use crate::{error::Result, Client};

pub struct ClientGenerator<'a> {
    pub(crate) _client: &'a crate::Client,
}

impl<'a> ClientGenerator<'a> {
    pub async fn password(&self, _input: PasswordGeneratorRequest) -> Result<String> {
        todo!()
    }

    pub async fn passphrase(&self, _input: PassphraseGeneratorRequest) -> Result<String> {
        todo!()
    }
}

impl<'a> Client {
    pub fn generator(&'a self) -> ClientGenerator<'a> {
        ClientGenerator { _client: self }
    }
}
