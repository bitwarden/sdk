//! # Bitwarden
//!
//! A Rust client SDK to interact with the Bitwarden Secrets Manager.
//! This is a beta release and might be missing some functionality.
//!
//! To use this crate, add it to your `Cargo.toml`:
//!
//! ```ini
//! [dependencies]
//! bitwarden = { "*", features = ["secrets"] }
//! ```
//!
//! # Basic setup
//!
//! All operations in this crate are done via a [Client]:
//!
//! ```rust
//! use bitwarden::{
//!     auth::login::AccessTokenLoginRequest,
//!     error::Result,
//!     secrets_manager::{secrets::SecretIdentifiersRequest, ClientSecretsExt},
//!     Client, ClientSettings, DeviceType,
//! };
//! use uuid::Uuid;
//!
//! async fn test() -> Result<()> {
//!     // Use the default values
//!     let mut client = Client::new(None);
//!
//!     // Or set your own values
//!     let settings = ClientSettings {
//!         identity_url: "https://identity.bitwarden.com".to_string(),
//!         api_url: "https://api.bitwarden.com".to_string(),
//!         user_agent: "Bitwarden Rust-SDK".to_string(),
//!         device_type: DeviceType::SDK,
//!     };
//!     let mut client = Client::new(Some(settings));
//!
//!     // Before we operate, we need to authenticate with a token
//!     let token = AccessTokenLoginRequest {
//!         access_token: String::from(""),
//!         state_file: None,
//!     };
//!     client.auth().login_access_token(&token).await.unwrap();
//!
//!     let org_id = SecretIdentifiersRequest {
//!         organization_id: Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap(),
//!     };
//!     println!(
//!         "Stored secrets: {:#?}",
//!         client.secrets().list(&org_id).await.unwrap()
//!     );
//!     Ok(())
//! }
//! ```

// Ensure the readme docs compile
#[doc = include_str!("../README.md")]
mod readme {}

pub use bitwarden_core::*;
pub mod error;

#[cfg(feature = "internal")]
pub mod internal {
    pub mod generators {
        pub use bitwarden_generators::*;
    }

    pub mod exporters {
        pub use bitwarden_exporters::*;
    }

    pub mod send {
        pub use bitwarden_send::*;
    }

    pub mod vault {
        pub use bitwarden_vault::*;
    }

    #[cfg(feature = "uniffi")]
    pub mod fido {
        pub use bitwarden_fido::*;
    }
}
#[cfg(feature = "internal")]
pub use internal::*;

// Re-export generators used for secrets-manager, internal flag already exports all generators
#[cfg(all(feature = "secrets", not(feature = "internal")))]
pub mod generators {
    pub use bitwarden_generators::{ClientGeneratorExt, PasswordError, PasswordGeneratorRequest};
}

#[cfg(feature = "secrets")]
pub mod secrets_manager {
    pub use bitwarden_sm::*;
}
