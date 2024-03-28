use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JSFido2ClientCreateCredentialRequest {
    pub options: String,
    pub origin: String,
}
