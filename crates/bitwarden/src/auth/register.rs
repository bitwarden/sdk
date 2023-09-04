use bitwarden_api_identity::{
    apis::accounts_api::accounts_register_post,
    models::{KeysRequestModel, RegisterRequestModel},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    client::auth_settings::Kdf,
    crypto::{HashPurpose, MasterKey},
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

pub(crate) async fn register(
    client: &mut Client,
    req: &RegisterRequest,
) -> Result<RegisterResponse> {
    let config = client.get_api_configurations().await;

    let kdf = Kdf::PBKDF2 {
        iterations: default_pbkdf2_iterations(),
    };

    let master_key = MasterKey::derive(req.password.as_bytes(), req.email.as_bytes(), &kdf)?;
    let master_password_hash = master_key
        .derive_master_key_hash(req.password.as_bytes(), HashPurpose::ServerAuthorization)?;
    let (user_key, encrypted_user_key) = master_key.make_user_key()?;
    let keys = user_key.make_key_pair()?;

    accounts_register_post(
        &config.identity,
        Some(RegisterRequestModel {
            name: req.name.to_owned(),
            email: req.email.to_owned(),
            master_password_hash,
            master_password_hint: req.password_hint.to_owned(),
            captcha_response: None, // TODO: Add
            key: Some(encrypted_user_key.to_string()),
            keys: Some(Box::new(KeysRequestModel {
                public_key: Some(keys.0),
                encrypted_private_key: keys.1.to_string(),
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

pub struct RegisterResponse {}
