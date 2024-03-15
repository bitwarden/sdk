use super::{
    fido2::{Fido2ClientCreateCredentialRequest, Fido2MakeCredentialUserInterface, VaultItem},
    generate_fingerprint::{generate_fingerprint, generate_user_fingerprint},
    FingerprintRequest, FingerprintResponse,
};
use crate::{
    error::Result,
    platform::fido2::{
        client_create_credential, client_get_assertion, Fido2ClientGetAssertionRequest,
        Fido2GetAssertionUserInterface,
    },
    Client,
};

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

    pub async fn client_create_credential(
        &self,
        request: Fido2ClientCreateCredentialRequest,
        user_interface: impl Fido2MakeCredentialUserInterface,
    ) -> Result<VaultItem> {
        log::debug!("client_platform.client_create_credential");
        client_create_credential(request, user_interface).await
    }
}

impl<'a> Client {
    pub fn platform(&'a mut self) -> ClientPlatform<'a> {
        ClientPlatform { client: self }
    }
}
