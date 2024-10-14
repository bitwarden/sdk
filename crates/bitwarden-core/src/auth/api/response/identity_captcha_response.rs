use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct IdentityCaptchaResponse {
    pub error: String,
    pub error_description: String,
    #[serde(rename = "HCaptcha_SiteKey")]
    pub site_key: String,

    /// Stores unknown api response fields
    extra: Option<HashMap<String, Value>>,
}

#[cfg(test)]
mod test {
    use super::*;

    impl Default for IdentityCaptchaResponse {
        fn default() -> Self {
            Self {
                error: "invalid_grant".into(),
                error_description: "Captcha required.".into(),
                site_key: Default::default(),
                extra: Default::default(),
            }
        }
    }
}
