use std::num::NonZeroU32;

use base64::{
    alphabet,
    engine::{DecodePaddingMode, GeneralPurpose, GeneralPurposeConfig},
};

pub fn default_pbkdf2_iterations() -> NonZeroU32 {
    NonZeroU32::new(600_000).unwrap()
}
#[cfg(feature = "internal")]
pub fn default_argon2_iterations() -> NonZeroU32 {
    NonZeroU32::new(3).unwrap()
}
#[cfg(feature = "internal")]
pub fn default_argon2_memory() -> NonZeroU32 {
    NonZeroU32::new(64).unwrap()
}
#[cfg(feature = "internal")]
pub fn default_argon2_parallelism() -> NonZeroU32 {
    NonZeroU32::new(4).unwrap()
}

const BASE64_ENGINE_CONFIG: GeneralPurposeConfig = GeneralPurposeConfig::new()
    .with_encode_padding(true)
    .with_decode_padding_mode(DecodePaddingMode::Indifferent);

pub const BASE64_ENGINE: GeneralPurpose =
    GeneralPurpose::new(&alphabet::STANDARD, BASE64_ENGINE_CONFIG);

#[cfg(test)]
pub async fn start_mock(mocks: Vec<wiremock::Mock>) -> (wiremock::MockServer, crate::Client) {
    let server = wiremock::MockServer::start().await;

    for mock in mocks {
        server.register(mock).await;
    }

    let settings = crate::client::client_settings::ClientSettings {
        identity_url: format!("http://{}/identity", server.address()),
        api_url: format!("http://{}/api", server.address()),
        user_agent: "Bitwarden Rust-SDK [TEST]".into(),
        device_type: crate::client::client_settings::DeviceType::SDK,
    };

    (server, crate::Client::new(Some(settings), None))
}
