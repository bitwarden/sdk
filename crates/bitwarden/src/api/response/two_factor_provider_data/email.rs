use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Email {
    #[serde(alias = "Email")]
    pub email: String,

    /// Stores unknown api response fields
    extra: Option<HashMap<String, Value>>,
}
