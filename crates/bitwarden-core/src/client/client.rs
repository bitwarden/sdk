use std::sync::{Arc, RwLock};

use bitwarden_crypto::service::CryptoService;
use reqwest::header::{self, HeaderValue};

use super::internal::InternalClient;
#[cfg(feature = "internal")]
use crate::client::flags::Flags;
use crate::client::{
    client_settings::ClientSettings,
    internal::{ApiConfigurations, Tokens},
};

/// The main struct to interact with the Bitwarden SDK.
#[derive(Debug)]
pub struct Client {
    #[doc(hidden)]
    pub internal: InternalClient,
}

impl Client {
    pub fn new(settings_input: Option<ClientSettings>) -> Self {
        let settings = settings_input.unwrap_or_default();

        fn new_client_builder() -> reqwest::ClientBuilder {
            #[allow(unused_mut)]
            let mut client_builder = reqwest::Client::builder();

            #[cfg(not(target_arch = "wasm32"))]
            {
                client_builder =
                    client_builder.use_preconfigured_tls(rustls_platform_verifier::tls_config());
            }

            client_builder
        }

        let external_client = new_client_builder().build().expect("Build should not fail");

        let mut headers = header::HeaderMap::new();
        headers.append(
            "Device-Type",
            HeaderValue::from_str(&(settings.device_type as u8).to_string())
                .expect("All numbers are valid ASCII"),
        );
        let client_builder = new_client_builder().default_headers(headers);

        let client = client_builder.build().expect("Build should not fail");

        let identity = bitwarden_api_identity::apis::configuration::Configuration {
            base_path: settings.identity_url,
            user_agent: Some(settings.user_agent.clone()),
            client: client.clone(),
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: None,
        };

        let api = bitwarden_api_api::apis::configuration::Configuration {
            base_path: settings.api_url,
            user_agent: Some(settings.user_agent),
            client,
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: None,
        };

        Self {
            internal: InternalClient {
                tokens: RwLock::new(Tokens::default()),
                login_method: RwLock::new(None),
                #[cfg(feature = "internal")]
                flags: RwLock::new(Flags::default()),
                __api_configurations: RwLock::new(Arc::new(ApiConfigurations {
                    identity,
                    api,
                    device_type: settings.device_type,
                })),
                external_client,
                crypto_service: CryptoService::new(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn test_reqwest_rustls_platform_verifier_are_compatible() {
        // rustls-platform-verifier is generating a rustls::ClientConfig,
        // which reqwest accepts as a &dyn Any and then downcasts it to a
        // rustls::ClientConfig.

        // This means that if the rustls version of the two crates don't match,
        // the downcast will fail and we will get a runtime error.

        // This tests is added to ensure that it doesn't happen.

        let _ = reqwest::ClientBuilder::new()
            .use_preconfigured_tls(rustls_platform_verifier::tls_config())
            .build()
            .unwrap();
    }
}
