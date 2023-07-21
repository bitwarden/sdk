use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    client::auth_settings::{AuthSettings, Kdf},
    error::{Error, Result},
    Client,
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct KdfParamRequest {
    /// The user's KDF parameters, as received from the prelogin request
    pub kdf_params: Kdf,
    /// The user's email address
    pub email: String,
}

pub async fn set_kdf_params(client: &mut Client, req: KdfParamRequest) -> Result<()> {
    let auth_settings = AuthSettings {
        email: req.email,
        kdf: req.kdf_params,
    };
    client.set_auth_settings(auth_settings);

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PasswordHashRequest {
    /// The user's master password
    pub password: String,
}

pub async fn hash_password(client: &mut Client, req: PasswordHashRequest) -> Result<String> {
    let Some(auth_settings) = client.get_auth_settings() else { return Err(Error::NotAuthenticated); };
    let hash = auth_settings.make_user_password_hash(&req.password)?;
    Ok(hash)
}
