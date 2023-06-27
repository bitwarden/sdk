use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct IdentityTokenPayloadResponse {
    pub access_token: String,
    pub expires_in: u64,
    pub refresh_token: Option<String>,
    token_type: String,
    scope: String,

    pub(crate) encrypted_payload: String,
}

#[cfg(test)]
mod test {
    use super::*;

    impl Default for IdentityTokenPayloadResponse {
        fn default() -> Self {
            Self {
                access_token: Default::default(),
                expires_in: Default::default(),
                refresh_token: Default::default(),
                token_type: Default::default(),
                scope: Default::default(),
                encrypted_payload: Default::default(),
            }
        }
    }
}
