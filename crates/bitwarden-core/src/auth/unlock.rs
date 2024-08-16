use bitwarden_crypto::{CryptoError, MasterKey};
use thiserror::Error;

use super::auth_repository::AuthRepositoryError;
use crate::Client;

#[derive(Debug, Error)]
pub enum UnlockError {
    #[error(transparent)]
    AuthRepository(#[from] AuthRepositoryError),

    #[error(transparent)]
    Crypto(#[from] CryptoError),

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
    );

    Ok(())
}
