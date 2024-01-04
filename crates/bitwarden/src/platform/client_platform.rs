use super::{
    generate_fingerprint::{generate_fingerprint, generate_users_fingerprint},
    FingerprintRequest, FingerprintResponse,
};
use crate::{error::Result, Client};

pub struct ClientPlatform<'a> {
    #[allow(dead_code)]
    pub(crate) client: &'a mut Client,
}

impl<'a> ClientPlatform<'a> {
    pub fn fingerprint(&self, input: &FingerprintRequest) -> Result<FingerprintResponse> {
        generate_fingerprint(input)
    }

    pub fn users_fingerprint(self, fingerprint_material: String) -> Result<String> {
        generate_users_fingerprint(self.client, fingerprint_material)
    }
}

impl<'a> Client {
    pub fn platform(&'a mut self) -> ClientPlatform<'a> {
        ClientPlatform { client: self }
    }
}
