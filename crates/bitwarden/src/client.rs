use bitwarden_core::{
    auth::client_auth::ClientAuth, client::client_settings::ClientSettings, mobile::ClientKdf,
};

#[cfg(feature = "secrets")]
use bitwarden_sm::{ClientProjects, ClientProjectsExt, ClientSecrets, ClientSecretsExt};

#[cfg(feature = "uniffi")]
use bitwarden_core::mobile::ClientCrypto;

#[cfg(feature = "internal")]
use bitwarden_core::{
    platform::client_platform::ClientPlatform, tool::ClientExporters, tool::ClientSends,
    vault::ClientVault,
};

#[cfg(feature = "internal")]
use bitwarden_generators::{ClientGenerator, ClientGeneratorExt};
use uuid::Uuid;

pub struct Client(bitwarden_core::Client);

impl Client {
    pub fn new(settings: Option<ClientSettings>) -> Self {
        Self(bitwarden_core::Client::new(settings))
    }

    #[cfg(feature = "internal")]
    pub fn load_flags(&mut self, flags: std::collections::HashMap<String, bool>) {
        self.0.load_flags(flags)
    }

    pub fn get_access_token_organization(&self) -> Option<Uuid> {
        self.0.get_access_token_organization()
    }

    #[cfg(feature = "internal")]
    pub fn kdf(&self) -> ClientKdf {
        self.0.kdf()
    }

    pub fn auth(&mut self) -> ClientAuth {
        self.0.auth()
    }

    #[cfg(feature = "internal")]
    pub fn vault(&mut self) -> ClientVault {
        self.0.vault()
    }

    #[cfg(feature = "internal")]
    pub fn platform(&mut self) -> ClientPlatform {
        self.0.platform()
    }

    #[cfg(feature = "internal")]
    pub fn sends(&mut self) -> ClientSends {
        self.0.sends()
    }

    #[cfg(feature = "internal")]
    pub fn generator(&self) -> ClientGenerator {
        self.0.generator()
    }

    #[cfg(feature = "internal")]
    pub fn exporters(&self) -> ClientExporters {
        self.0.exporters()
    }

    #[cfg(feature = "internal")]
    pub fn crypto(&mut self) -> ClientCrypto {
        self.0.crypto()
    }

    #[cfg(feature = "secrets")]
    pub fn secrets(&mut self) -> ClientSecrets {
        self.0.secrets()
    }

    #[cfg(feature = "secrets")]
    pub fn projects(&mut self) -> ClientProjects {
        self.0.projects()
    }
}
