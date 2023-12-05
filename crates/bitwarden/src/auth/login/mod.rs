#[cfg(feature = "internal")]
use {
    crate::{
        client::{kdf::Kdf, Client},
        error::Result,
    },
    bitwarden_api_identity::{
        apis::accounts_api::accounts_prelogin_post,
        models::{PreloginRequestModel, PreloginResponseModel},
    },
};

pub mod response;

mod password;
#[cfg(feature = "internal")]
pub(crate) use password::login_password;
#[cfg(feature = "internal")]
pub use password::PasswordLoginRequest;
pub use password::PasswordLoginResponse;
#[cfg(feature = "internal")]
mod two_factor;
#[cfg(feature = "internal")]
pub(crate) use two_factor::send_two_factor_email;
#[cfg(feature = "internal")]
pub use two_factor::{TwoFactorEmailRequest, TwoFactorProvider, TwoFactorRequest};

#[cfg(feature = "internal")]
mod api_key;
#[cfg(feature = "internal")]
pub(crate) use api_key::login_api_key;
#[cfg(feature = "internal")]
pub use api_key::{ApiKeyLoginRequest, ApiKeyLoginResponse};

#[cfg(feature = "secrets")]
mod access_token;
#[cfg(feature = "secrets")]
pub(super) use access_token::{login_access_token, login_access_token_from_state};
#[cfg(feature = "secrets")]
pub use access_token::{AccessTokenLoginRequest, AccessTokenLoginResponse};

#[cfg(feature = "internal")]
async fn determine_password_hash(email: &str, kdf: &Kdf, password: &str) -> Result<String> {
    use crate::crypto::{HashPurpose, MasterKey};

    let master_key = MasterKey::derive(password.as_bytes(), email.as_bytes(), kdf)?;
    master_key.derive_master_key_hash(password.as_bytes(), HashPurpose::ServerAuthorization)
}

#[cfg(feature = "internal")]
pub(crate) async fn request_prelogin(
    client: &mut Client,
    email: String,
) -> Result<PreloginResponseModel> {
    let request_model = PreloginRequestModel::new(email);
    let config = client.get_api_configurations().await;
    Ok(accounts_prelogin_post(&config.identity, Some(request_model)).await?)
}
