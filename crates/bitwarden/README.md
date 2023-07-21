# Bitwarden Secrets Manager SDK

A Rust client SDK to interact with the Bitwarden Secrets Manager. This is a beta release and might
be missing some functionality.

## Usage

```toml
[dependencies]
bitwarden = "*"
```

## Minimum Supported Rust Version

Rust **1.57** or higher.

## Example

```rust
use bitwarden::{
    auth::request::AccessTokenLoginRequest,
    client::client_settings::{ClientSettings, DeviceType},
    error::Result,
    secrets_manager::secrets::SecretIdentifiersRequest,
    Client,
};
use uuid::Uuid;

async fn test() -> Result<()> {
    // Use the default values
    let mut client = Client::new(None);

    // Or set your own values
    let settings = ClientSettings {
        identity_url: "https://identity.bitwarden.com".to_string(),
        api_url: "https://api.bitwarden.com".to_string(),
        user_agent: "Bitwarden Rust-SDK".to_string(),
        device_type: DeviceType::SDK,
        internal: None,
    };
    let mut client = Client::new(Some(settings));

    // Before we operate, we need to authenticate with a token
    let token = AccessTokenLoginRequest { access_token: String::from("") };
    client.access_token_login(&token).await.unwrap();

    let org_id = SecretIdentifiersRequest { organization_id: Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap() };
    println!("Stored secrets: {:#?}", client.secrets().list(&org_id).await.unwrap());
    Ok(())
}
```
