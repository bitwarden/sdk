use std::num::NonZeroU32;

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

#[cfg(feature = "mobile")]
pub(crate) fn capitalize_first_letter(s: &str) -> String {
    // Unicode case conversion can change the length of the string, so we can't capitalize in place.
    // Instead we extract the first character and convert it to uppercase. This returns
    // an iterator which we collect into a string, and then append the rest of the input.
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

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
