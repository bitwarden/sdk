use bitwarden_api_identity::{
    apis::accounts_api::accounts_register_post,
    models::{KeysRequestModel, RegisterRequestModel},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    client::auth_settings::Kdf,
    crypto::{HashPurpose, MasterKey, RsaKeyPair},
    error::Result,
    util::default_pbkdf2_iterations,
    Client,
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RegisterRequest {
    pub email: String,
    pub name: Option<String>,
    pub password: String,
    pub password_hint: Option<String>,
}

pub(super) async fn register(client: &mut Client, req: &RegisterRequest) -> Result<()> {
    let config = client.get_api_configurations().await;

    let kdf = Kdf::PBKDF2 {
        iterations: default_pbkdf2_iterations(),
    };

    let keys = generate_register_keys(req.email.to_owned(), req.password.to_owned(), kdf)?;

    accounts_register_post(
        &config.identity,
        Some(RegisterRequestModel {
            name: req.name.to_owned(),
            email: req.email.to_owned(),
            master_password_hash: keys.master_password_hash,
            master_password_hint: req.password_hint.to_owned(),
            captcha_response: None, // TODO: Add
            key: Some(keys.encrypted_user_key.to_string()),
            keys: Some(Box::new(KeysRequestModel {
                public_key: Some(keys.keys.public),
                encrypted_private_key: keys.keys.private.to_string(),
            })),
            token: None,
            organization_user_id: None,
            kdf: Some(bitwarden_api_identity::models::KdfType::Variant0),
            kdf_iterations: Some(default_pbkdf2_iterations().get() as i32),
            kdf_memory: None,
            kdf_parallelism: None,
            reference_data: None, // TODO: Add
        }),
    )
    .await?;

    unimplemented!()
}

pub(super) fn generate_register_keys(
    email: String,
    password: String,
    kdf: Kdf,
) -> Result<RegisterKeyResponse> {
    let master_key = MasterKey::derive(password.as_bytes(), email.as_bytes(), &kdf)?;
    let master_password_hash =
        master_key.derive_master_key_hash(password.as_bytes(), HashPurpose::ServerAuthorization)?;
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
    master_password_hash: String,
    encrypted_user_key: String,
    keys: RsaKeyPair,
}
