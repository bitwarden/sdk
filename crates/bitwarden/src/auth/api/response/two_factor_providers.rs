use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

use crate::auth::api::response::two_factor_provider_data::{
    authenticator::Authenticator, duo::Duo, email::Email, organization_duo::OrganizationDuo,
    remember::Remember, web_authn::WebAuthn, yubi_key::YubiKey,
};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TwoFactorProviders {
    #[serde(rename = "0", default, deserialize_with = "double_option")]
    pub authenticator: Option<Option<Authenticator>>,
    #[serde(rename = "1")]
    pub email: Option<Email>,
    #[serde(rename = "2")]
    pub duo: Option<Duo>,
    #[serde(rename = "3")]
    pub yubi_key: Option<YubiKey>,
    // Deprecated
    // #[serde(rename = "4")]
    // u2f: Option<U2F>,
    #[serde(rename = "5")]
    pub remember: Option<Remember>,
    #[serde(rename = "6")]
    pub organization_duo: Option<OrganizationDuo>,
    #[serde(rename = "7")]
    pub web_authn: Option<WebAuthn>,

    /// Stores unknown api response fields
    extra: Option<HashMap<String, Value>>,
}

impl Default for TwoFactorProviders {
    fn default() -> Self {
        Self {
            authenticator: Some(Default::default()),
            email: Some(Default::default()),
            duo: Some(Default::default()),
            yubi_key: Some(Default::default()),
            remember: Some(Default::default()),
            organization_duo: Some(Default::default()),
            web_authn: Some(Default::default()),

            extra: Default::default(),
        }
    }
}

pub fn double_option<'de, T, D>(de: D) -> Result<Option<Option<T>>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Deserialize::deserialize(de).map(Some)
}
