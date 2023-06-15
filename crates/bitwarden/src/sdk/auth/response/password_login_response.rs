use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    api::response::IdentityTokenResponse,
    error::Result,
    sdk::{
        auth::response::two_factor_login_response::TwoFactorProviders,
        response::captcha_response::CaptchaResponse,
    },
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PasswordLoginResponse {
    pub authenticated: bool,
    /// TODO: What does this do?
    pub reset_master_password: bool,
    /// Whether or not the user is required to update their master password
    pub force_password_reset: bool,
    /// The available two factor authentication options. Present only when authentication fails due to requiring a second authentication factor.
    pub two_factor: Option<TwoFactorProviders>,
    /// The information required to present the user with a captcha challenge. Only present when authentication fails due to requiring validation of a captcha challenge.
    pub captcha: Option<CaptchaResponse>,
}

impl PasswordLoginResponse {
    pub(crate) fn process_response(
        response: IdentityTokenResponse,
    ) -> Result<PasswordLoginResponse> {
        match response {
            IdentityTokenResponse::Authenticated(success) => Ok(PasswordLoginResponse {
                authenticated: true,
                reset_master_password: success.reset_master_password,
                force_password_reset: success.force_password_reset,
                two_factor: None,
                captcha: None,
            }),
            IdentityTokenResponse::Payload(_) => Ok(PasswordLoginResponse {
                authenticated: true,
                reset_master_password: false,
                force_password_reset: false,
                two_factor: None,
                captcha: None,
            }),
            IdentityTokenResponse::TwoFactorRequired(two_factor) => Ok(PasswordLoginResponse {
                authenticated: false,
                reset_master_password: false,
                force_password_reset: false,
                two_factor: Some(two_factor.two_factor_providers.into()),
                captcha: two_factor.captcha_token.map(Into::into),
            }),
            IdentityTokenResponse::CaptchaRequired(captcha) => Ok(PasswordLoginResponse {
                authenticated: false,
                reset_master_password: false,
                force_password_reset: false,
                two_factor: None,
                captcha: Some(captcha.site_key.into()),
            }),
            IdentityTokenResponse::Refreshed(_) => {
                unreachable!("Got a `refresh_token` answer to a login request")
            }
        }
    }
}
