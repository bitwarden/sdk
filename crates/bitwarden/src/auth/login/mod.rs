#[cfg(feature = "internal")]
use {
    crate::{client::Kdf, error::Result, Client},
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

#[cfg(feature = "mobile")]
mod auth_request;
#[cfg(feature = "mobile")]
pub use auth_request::NewAuthRequestResponse;
#[cfg(feature = "mobile")]
pub(crate) use auth_request::{complete_auth_request, send_new_auth_request};

#[cfg(feature = "secrets")]
mod access_token;
#[cfg(feature = "secrets")]
pub(super) use access_token::login_access_token;
#[cfg(feature = "secrets")]
pub use access_token::{AccessTokenLoginRequest, AccessTokenLoginResponse};

#[cfg(feature = "internal")]
pub(crate) async fn request_prelogin(
    client: &mut Client,
    email: String,
) -> Result<PreloginResponseModel> {
    let request_model = PreloginRequestModel::new(email);
    let config = client.get_api_configurations().await;
    Ok(accounts_prelogin_post(&config.identity, Some(request_model)).await?)
}

#[cfg(feature = "internal")]
pub(crate) fn parse_prelogin(response: PreloginResponseModel) -> Result<Kdf> {
    use std::num::NonZeroU32;

    use bitwarden_api_identity::models::KdfType;

    use crate::util::{
        default_argon2_iterations, default_argon2_memory, default_argon2_parallelism,
        default_pbkdf2_iterations,
    };

    let kdf = response.kdf.ok_or("KDF not found")?;

    Ok(match kdf {
        KdfType::Variant0 => Kdf::PBKDF2 {
            iterations: response
                .kdf_iterations
                .and_then(|e| NonZeroU32::new(e as u32))
                .unwrap_or_else(default_pbkdf2_iterations),
        },
        KdfType::Variant1 => Kdf::Argon2id {
            iterations: response
                .kdf_iterations
                .and_then(|e| NonZeroU32::new(e as u32))
                .unwrap_or_else(default_argon2_iterations),
            memory: response
                .kdf_memory
                .and_then(|e| NonZeroU32::new(e as u32))
                .unwrap_or_else(default_argon2_memory),
            parallelism: response
                .kdf_parallelism
                .and_then(|e| NonZeroU32::new(e as u32))
                .unwrap_or_else(default_argon2_parallelism),
        },
    })
}
