use super::{
    password::{password_strength, satisfies_policy, MasterPasswordPolicyOptions},
    register::generate_register_keys,
    RegisterResponse,
};
use crate::{client::auth_settings::Kdf, error::Result, Client};

pub struct ClientAuth<'a> {
    pub(crate) _client: &'a crate::Client,
}

impl<'a> ClientAuth<'a> {
    pub async fn password_strength(
        &self,
        password: String,
        email: String,
        additional_inputs: Vec<String>,
    ) -> u8 {
        password_strength(password, email, additional_inputs)
    }

    pub async fn satisfies_policy(
        &self,
        password: String,
        strength: u8,
        policy: &MasterPasswordPolicyOptions,
    ) -> bool {
        satisfies_policy(password, strength, policy)
    }

    pub fn generate_register_keys(
        &self,
        email: String,
        password: String,
        kdf: Kdf,
    ) -> Result<RegisterResponse> {
        generate_register_keys(email, password, kdf)
    }
}

impl<'a> Client {
    pub fn auth(&'a self) -> ClientAuth<'a> {
        ClientAuth { _client: self }
    }
}
