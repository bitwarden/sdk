use crate::{
    client::auth_settings::{AuthSettings, Kdf},
    error::Result,
    Client,
};

pub async fn hash_password(
    _client: &Client,
    email: String,
    password: String,
    kdf_params: Kdf,
) -> Result<String> {
    let auth_settings = AuthSettings {
        email,
        kdf: kdf_params,
    };
    let hash = auth_settings.make_user_password_hash(&password)?;
    Ok(hash)
}
