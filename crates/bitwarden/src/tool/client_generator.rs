use super::password::{PassphraseGeneratorRequest, PasswordGeneratorRequest};
use crate::{error::Result, Client};

pub struct ClientGenerator<'a> {
    pub(crate) _client: &'a mut crate::Client,
}

impl<'a> ClientGenerator<'a> {
    pub async fn password(&mut self, _input: &PasswordGeneratorRequest) -> Result<String> {
        todo!()
    }

    pub async fn passphrase(&mut self, _input: &PassphraseGeneratorRequest) -> Result<String> {
        todo!()
    }
}

impl<'a> Client {
    pub fn generator(&'a mut self) -> ClientGenerator<'a> {
        ClientGenerator { _client: self }
    }
}
