use crate::{crypto::EncString, error::Error};

pub trait CryptoKey {
    fn decrypt(&self, e: &EncString) -> Result<Vec<u8>, Error>;
    fn encrypt(&self, e: &[u8]) -> Result<EncString, Error>;
}
