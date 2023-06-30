use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    auth::{
        api::response::IdentityTokenResponse,
        response::{
            password_login_response::PasswordLoginResponse,
            two_factor_login_response::TwoFactorProviders,
        },
    },
    error::Result,
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ApiKeyLoginResponse {
    pub authenticated: bool,
    /// TODO: What does this do?
    pub reset_master_password: bool,
    /// Whether or not the user is required to update their master password
    pub force_password_reset: bool,
    two_factor: Option<TwoFactorProviders>,
}

impl ApiKeyLoginResponse {
    pub(crate) fn process_response(response: IdentityTokenResponse) -> Result<ApiKeyLoginResponse> {
        let password_response = PasswordLoginResponse::process_response(response)?;

        Ok(ApiKeyLoginResponse {
            authenticated: password_response.authenticated,
            reset_master_password: password_response.reset_master_password,
            force_password_reset: password_response.force_password_reset,
            two_factor: password_response.two_factor,
        })
    }
}
