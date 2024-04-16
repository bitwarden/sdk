use base64::{
    alphabet,
    engine::{DecodePaddingMode, GeneralPurpose, GeneralPurposeConfig},
};

const INDIFFERENT: GeneralPurposeConfig =
    GeneralPurposeConfig::new().with_decode_padding_mode(DecodePaddingMode::Indifferent);

pub const STANDARD_INDIFFERENT: GeneralPurpose =
    GeneralPurpose::new(&alphabet::STANDARD, INDIFFERENT);

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

    (server, crate::Client::new(Some(settings)))
}
