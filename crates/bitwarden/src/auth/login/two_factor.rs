use bitwarden_api_api::models::TwoFactorEmailRequestModel;
use bitwarden_crypto::HashPurpose;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{auth::determine_password_hash, error::Result, Client};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TwoFactorEmailRequest {
    /// User Password
    pub password: String,
    /// User email
    pub email: String,
}

pub(crate) async fn send_two_factor_email(
    client: &mut Client,
    input: &TwoFactorEmailRequest,
) -> Result<()> {
    // TODO: This should be resolved from the client
    let kdf = client.auth().prelogin(input.email.clone()).await?;

    let password_hash = determine_password_hash(
        &input.email,
        &kdf,
        &input.password,
        HashPurpose::ServerAuthorization,
    )?;

    let config = client.get_api_configurations().await;
    bitwarden_api_api::apis::two_factor_api::two_factor_send_email_login_post(
        &config.api,
        Some(TwoFactorEmailRequestModel {
            master_password_hash: Some(password_hash),
            otp: None,
            auth_request_access_code: None,
            secret: None,
            email: input.email.to_owned(),
            auth_request_id: None,
            sso_email2_fa_session_token: None,
        }),
    )
    .await?;
    Ok(())
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, JsonSchema, Clone)]
#[repr(u8)]
pub enum TwoFactorProvider {
    Authenticator = 0,
    Email = 1,
    Duo = 2,
    Yubikey = 3,
    U2f = 4,
    Remember = 5,
    OrganizationDuo = 6,
    WebAuthn = 7,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TwoFactorRequest {
    /// Two-factor Token
    pub token: String,
    /// Two-factor provider
    pub provider: TwoFactorProvider,
    /// Two-factor remember
    pub remember: bool,
}
