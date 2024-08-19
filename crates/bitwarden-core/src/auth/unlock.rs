use bitwarden_crypto::{CryptoError, MasterKey};
use thiserror::Error;

use crate::{platform::SettingsRepositoryError, Client};

#[derive(Debug, Error)]
pub enum UnlockError {
    #[error(transparent)]
    SettingRepository(#[from] SettingsRepositoryError),

    #[error(transparent)]
    Crypto(#[from] CryptoError),

    #[error(transparent)]
    Error(#[from] crate::Error),

    #[error("The client is not authenticated or the session has expired")]
    NotAuthenticated,
}

pub(crate) async fn unlock(
    client: &Client,
    password: String,
) -> std::result::Result<(), UnlockError> {
    let settings = client
        .auth()
        .repository
        .get()
        .await?
        .ok_or(UnlockError::NotAuthenticated)?;

    // client.internal.set_login_method(UserLoginMethod::ApiKey { client_id: (), client_secret: (),
    // email: (), kdf: () })
    let master_key = MasterKey::derive(&password, &settings.email, &settings.kdf)?;
    client.internal.initialize_user_crypto_master_key(
        master_key,
        settings.user_key.parse()?,
        settings.private_key.parse()?,
    )?;

    Ok(())
}
