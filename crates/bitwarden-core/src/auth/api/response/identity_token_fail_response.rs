use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct IdentityTokenFailResponse {
    pub error: String,
    pub error_description: String,
    #[serde(alias = "ErrorModel")]
    pub error_model: ErrorModel,
}

impl fmt::Display for IdentityTokenFailResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self.error_model.message.trim().is_empty() {
            true => format!("{}: {}", self.error, self.error_description),
            false => self.error_model.message.clone(),
        };

        write!(f, "{}", msg)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ErrorModel {
    #[serde(alias = "Message")]
    pub message: String,
    #[serde(alias = "Object")]
    object: String,
}

impl fmt::Display for ErrorModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
