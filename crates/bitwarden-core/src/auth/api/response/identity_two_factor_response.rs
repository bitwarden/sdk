use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::auth::api::response::two_factor_providers::TwoFactorProviders;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct IdentityTwoFactorResponse {
    pub error: String,
    pub error_description: String,
    #[serde(rename = "twoFactorProviders2", alias = "TwoFactorProviders2")]
    pub two_factor_providers: TwoFactorProviders,
    #[serde(rename = "captchaBypassToken", alias = "CaptchaBypassToken")]
    pub captcha_token: Option<String>,

    /// Stores unknown api response fields
    extra: Option<HashMap<String, Value>>,
}

#[cfg(test)]
mod test {
    use super::*;

    impl Default for IdentityTwoFactorResponse {
        fn default() -> Self {
            Self {
                error: "invalid_grant".into(),
                error_description: "Two factor required.".into(),
                two_factor_providers: Default::default(),
                captcha_token: Default::default(),
                extra: Default::default(),
            }
        }
    }
}
