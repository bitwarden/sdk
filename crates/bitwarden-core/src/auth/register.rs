use bitwarden_api_identity::{
    apis::accounts_api::accounts_register_post,
    models::{KeysRequestModel, RegisterRequestModel},
};
use bitwarden_crypto::{
    default_pbkdf2_iterations, HashPurpose, MasterKey, RsaKeyPair, SensitiveString,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{client::Kdf, error::Result, Client};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RegisterRequest {
    pub email: String,
    pub name: Option<String>,
    pub password: SensitiveString,
    pub password_hint: Option<String>,
}

/// Half baked implementation of user registration
pub(super) async fn register(client: &mut Client, req: RegisterRequest) -> Result<()> {
    let config = client.get_api_configurations().await;

    let kdf = Kdf::default();

    let keys = make_register_keys(req.email.clone(), req.password, kdf)?;

    accounts_register_post(
        &config.identity,
        Some(RegisterRequestModel {
            name: req.name,
            email: req.email,
            master_password_hash: keys.master_password_hash.expose().clone(),
            master_password_hint: req.password_hint,
            captcha_response: None, // TODO: Add
            key: Some(keys.encrypted_user_key),
            keys: Some(Box::new(KeysRequestModel {
                public_key: Some(keys.keys.public),
                encrypted_private_key: keys.keys.private.to_string(),
            })),
            token: None,
            organization_user_id: None,
            kdf: Some(bitwarden_api_identity::models::KdfType::PBKDF2_SHA256),
            kdf_iterations: Some(default_pbkdf2_iterations().get() as i32),
            kdf_memory: None,
            kdf_parallelism: None,
            reference_data: None, // TODO: Add
        }),
    )
    .await?;

    Ok(())
}

pub(super) fn make_register_keys(
    email: String,
    password: SensitiveString,
    kdf: Kdf,
) -> Result<RegisterKeyResponse> {
    let password_vec = password.into();
    let master_key = MasterKey::derive(&password_vec, email.as_bytes(), &kdf)?;
    let master_password_hash =
        master_key.derive_master_key_hash(&password_vec, HashPurpose::ServerAuthorization)?;
    let (user_key, encrypted_user_key) = master_key.make_user_key()?;
    let keys = user_key.make_key_pair()?;

    Ok(RegisterKeyResponse {
        master_password_hash,
        encrypted_user_key: encrypted_user_key.to_string(),
        keys,
    })
}

#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct RegisterKeyResponse {
    pub master_password_hash: SensitiveString,
    pub encrypted_user_key: String,
    pub keys: RsaKeyPair,
}
