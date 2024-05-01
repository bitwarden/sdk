use super::{
    generate_fingerprint::{generate_fingerprint, generate_user_fingerprint},
    ClientPasskeys, FingerprintRequest, FingerprintResponse,
};
use crate::{error::Result, Client};

pub struct ClientPlatform<'a> {
    pub(crate) client: &'a mut Client,
}

impl<'a> ClientPlatform<'a> {
    pub fn fingerprint(&self, input: &FingerprintRequest) -> Result<FingerprintResponse> {
        generate_fingerprint(input)
    }

    pub fn user_fingerprint(self, fingerprint_material: String) -> Result<String> {
        generate_user_fingerprint(self.client, fingerprint_material)
    }

    pub fn passkeys(&'a mut self) -> ClientPasskeys<'a> {
        ClientPasskeys {
            client: self.client,
        }
    }
}

impl<'a> Client {
    pub fn platform(&'a mut self) -> ClientPlatform<'a> {
        ClientPlatform { client: self }
    }
}
