pub use bitwarden_core::*;

#[cfg(feature = "internal")]
pub use bitwarden_generators::ClientGeneratorExt;
#[cfg(feature = "internal")]
pub mod generators {
    pub use bitwarden_generators::{
        PassphraseGeneratorRequest, PasswordGeneratorRequest, UsernameGeneratorRequest,
    };
}

#[cfg(feature = "secrets")]
pub use bitwarden_sm::ClientProjectsExt;
#[cfg(feature = "secrets")]
pub use bitwarden_sm::ClientSecretsExt;
#[cfg(feature = "secrets")]
pub mod secrets_manager {
    pub mod projects {
        pub use bitwarden_sm::projects::*;
    }
    pub mod secrets {
        pub use bitwarden_sm::secrets::*;
    }
}
