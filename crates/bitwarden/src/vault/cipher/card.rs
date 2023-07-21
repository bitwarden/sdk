use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::crypto::CipherString;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Card {
    pub cardholder_name: Option<CipherString>,
    pub exp_month: Option<CipherString>,
    pub exp_year: Option<CipherString>,
    pub code: Option<CipherString>,
    pub brand: Option<CipherString>,
    pub number: Option<CipherString>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CardView {
    pub cardholder_name: Option<String>,
    pub exp_month: Option<String>,
    pub exp_year: Option<String>,
    pub code: Option<String>,
    pub brand: Option<String>,
    pub number: Option<String>,
}
