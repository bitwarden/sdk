#[allow(hidden_glob_reexports)]
mod client;
pub use client::Client;

pub use bitwarden_core::client::client_settings::ClientSettings;
pub use bitwarden_core::*;

#[cfg(feature = "internal")]
pub mod generators {
    pub use bitwarden_generators::{
        PassphraseGeneratorRequest, PasswordGeneratorRequest, UsernameGeneratorRequest,
    };
}

#[cfg(feature = "secrets")]
pub mod secrets_manager {
    pub mod projects {
        pub use bitwarden_sm::projects::*;
    }
    pub mod secrets {
        pub use bitwarden_sm::secrets::*;
    }
}
