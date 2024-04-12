use bitwarden_crypto::{EncString, HashPurpose, MasterKey};

use crate::{
    client::{LoginMethod, UserLoginMethod},
    error::{Error, Result},
    Client,
};

#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct SetPasswordResponse {
    // Password hash for server authentication
    password_hash: String,
    // MasterKey (password) protected user key
    protected_user_key: EncString,
}

pub(crate) async fn set_password(client: &Client, password: String) -> Result<SetPasswordResponse> {
    let user_key = client
        .get_encryption_settings()?
        .get_key(&None)
        .ok_or(Error::VaultLocked)?;

    let login_method = client
        .get_login_method()
        .as_ref()
        .ok_or(Error::NotAuthenticated)?;

    let (email, kdf) = match login_method {
        LoginMethod::User(
            UserLoginMethod::Username { email, kdf, .. }
            | UserLoginMethod::ApiKey { email, kdf, .. },
        ) => (email, kdf),
        _ => return Err(Error::NotAuthenticated),
    };

    let master_key = MasterKey::derive(password.as_bytes(), email.as_bytes(), kdf)?;

    Ok(SetPasswordResponse {
        password_hash: master_key
            .derive_master_key_hash(password.as_bytes(), HashPurpose::ServerAuthorization)?,
        protected_user_key: master_key.encrypt_user_key(user_key)?,
    })
}
