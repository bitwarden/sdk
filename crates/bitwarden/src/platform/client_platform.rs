use super::{generate_fingerprint, FingerprintRequest, FingerprintResponse};
use crate::{error::Result, Client};

pub struct ClientPlatform<'a> {
    #[allow(dead_code)]
    pub(crate) client: &'a mut Client,
}

impl<'a> ClientPlatform<'a> {
    pub fn fingerprint(&self, input: &FingerprintRequest) -> Result<FingerprintResponse> {
        generate_fingerprint(input)
    }
}

impl<'a> Client {
    pub fn platform(&'a mut self) -> ClientPlatform<'a> {
        ClientPlatform { client: self }
    }
}
