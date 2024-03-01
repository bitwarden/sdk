use super::{
    client_get_assertion,
    fido2::Fido2GetAssertionUserInterface,
    generate_fingerprint::{generate_fingerprint, generate_user_fingerprint},
    Fido2ClientGetAssertionRequest, FingerprintRequest, FingerprintResponse,
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

    pub async fn client_get_assertion(
        &self,
        request: Fido2ClientGetAssertionRequest,
        user_interface: impl Fido2GetAssertionUserInterface,
    ) -> Result<String> {
        log::debug!("client_platform.client_get_assertion");
        client_get_assertion(request, user_interface).await
    }
}

impl<'a> Client {
    pub fn platform(&'a mut self) -> ClientPlatform<'a> {
        ClientPlatform { client: self }
    }
}
