use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    client::auth_settings::{AuthSettings, Kdf},
    error::Result,
    Client,
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct PasswordHashRequest {
    /// The user's KDF parameters, as received from the prelogin request
    pub kdf_params: Kdf,
    /// The user's email address
    pub email: String,
    /// The user's master password
    pub password: String,
}

pub async fn hash_password(_client: &Client, req: PasswordHashRequest) -> Result<String> {
    let auth_settings = AuthSettings {
        email: req.email,
        kdf: req.kdf_params,
    };
    let hash = auth_settings.make_user_password_hash(&req.password)?;
    Ok(hash)
}
