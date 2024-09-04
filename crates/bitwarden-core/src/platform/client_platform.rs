use std::time::Duration;

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

    #[cfg(debug_assertions)]
    pub async fn cancellation_test(&mut self, duration_millis: u64) -> Result<i32> {
        tokio::time::sleep(Duration::from_millis(duration_millis)).await;
        println!("After wait #1");
        tokio::time::sleep(Duration::from_millis(duration_millis)).await;
        println!("After wait #2");
        tokio::time::sleep(Duration::from_millis(duration_millis)).await;
        println!("After wait #3");
        Ok(42)
    }

    #[cfg(debug_assertions)]
    pub async fn error_test(&mut self) -> Result<i32> {
        use crate::Error;

        Err(Error::Internal(std::borrow::Cow::Borrowed(
            "This is an error.",
        )))
    }
}

impl<'a> Client {
    pub fn platform(&'a self) -> ClientPlatform<'a> {
        ClientPlatform { client: self }
    }
}
