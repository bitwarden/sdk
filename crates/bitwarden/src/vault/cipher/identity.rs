use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::crypto::CipherString;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Identity {
    pub title: Option<CipherString>,
    pub first_name: Option<CipherString>,
    pub middle_name: Option<CipherString>,
    pub last_name: Option<CipherString>,
    pub address1: Option<CipherString>,
    pub address2: Option<CipherString>,
    pub address3: Option<CipherString>,
    pub city: Option<CipherString>,
    pub state: Option<CipherString>,
    pub postal_code: Option<CipherString>,
    pub country: Option<CipherString>,
    pub company: Option<CipherString>,
    pub email: Option<CipherString>,
    pub phone: Option<CipherString>,
    pub ssn: Option<CipherString>,
    pub username: Option<CipherString>,
    pub passport_number: Option<CipherString>,
    pub license_number: Option<CipherString>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IdentityView {
    pub title: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub address3: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub company: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub ssn: Option<String>,
    pub username: Option<String>,
    pub passport_number: Option<String>,
    pub license_number: Option<String>,
}
