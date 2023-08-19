#[cfg(feature = "internal")]
use {
    crate::{
        client::{auth_settings::AuthSettings, Client},
        error::Result,
    },
    bitwarden_api_identity::{
        apis::accounts_api::accounts_prelogin_post,
        models::{PreloginRequestModel, PreloginResponseModel},
    },
};

#[cfg(feature = "internal")]
mod password;
#[cfg(feature = "internal")]
pub(crate) use password::password_login;
#[cfg(feature = "internal")]
pub use password::PasswordLoginRequest;
#[cfg(feature = "internal")]
mod api_key;
#[cfg(feature = "internal")]
pub(crate) use api_key::api_key_login;
#[cfg(feature = "internal")]
pub use api_key::ApiKeyLoginRequest;

#[cfg(feature = "secrets")]
mod access_token;
#[cfg(feature = "secrets")]
pub(crate) use access_token::access_token_login;
#[cfg(feature = "secrets")]
pub use access_token::AccessTokenLoginRequest;

#[cfg(feature = "internal")]
async fn determine_password_hash(
    client: &mut Client,
    email: &str,
    password: &str,
) -> Result<String> {
    let pre_login = request_prelogin(client, email.to_owned()).await?;
    let auth_settings = AuthSettings::new(pre_login, email.to_owned());
    let password_hash = auth_settings.make_user_password_hash(password)?;
    client.set_auth_settings(auth_settings);

    Ok(password_hash)
}

#[cfg(feature = "internal")]
async fn request_prelogin(client: &mut Client, email: String) -> Result<PreloginResponseModel> {
    let request_model = PreloginRequestModel::new(email);
    let config = client.get_api_configurations().await;
    Ok(accounts_prelogin_post(&config.identity, Some(request_model)).await?)
}
