#[cfg(feature = "uniffi")]
use super::ClientFido2;
use super::{
    generate_fingerprint::{generate_fingerprint, generate_user_fingerprint},
    get_user_api_key, FingerprintRequest, FingerprintResponse, SecretVerificationRequest,
    UserApiKeyResponse,
};
use crate::{error::Result, Client};

pub struct ClientPlatform<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> ClientPlatform<'a> {
    pub fn fingerprint(&self, input: &FingerprintRequest) -> Result<FingerprintResponse> {
        generate_fingerprint(input)
    }

    pub fn user_fingerprint(self, fingerprint_material: String) -> Result<String> {
        generate_user_fingerprint(self.client, fingerprint_material)
    }

    pub async fn get_user_api_key(
        &mut self,
        input: SecretVerificationRequest,
    ) -> Result<UserApiKeyResponse> {
        get_user_api_key(self.client, &input).await
    }

    #[cfg(feature = "uniffi")]
    pub fn fido2(&'a self) -> ClientFido2<'a> {
        ClientFido2 {
            client: self.client,
        }
    }
}

impl<'a> Client {
    pub fn platform(&'a self) -> ClientPlatform<'a> {
        ClientPlatform { client: self }
    }
}
