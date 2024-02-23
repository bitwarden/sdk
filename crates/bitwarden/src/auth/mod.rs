pub(super) mod api;
pub mod client_auth;
mod jwt_token;
pub mod login;
#[cfg(feature = "internal")]
pub mod password;
pub mod renew;
pub use jwt_token::JWTToken;
#[cfg(feature = "internal")]
mod register;
#[cfg(feature = "internal")]
use bitwarden_crypto::{HashPurpose, MasterKey};
#[cfg(feature = "internal")]
pub use register::{RegisterKeyResponse, RegisterRequest};
#[cfg(feature = "internal")]
mod auth_request;
#[cfg(feature = "internal")]
pub use auth_request::AuthRequestResponse;
#[cfg(feature = "mobile")]
pub(crate) use auth_request::{auth_request_decrypt_master_key, auth_request_decrypt_user_key};

#[cfg(feature = "internal")]
use crate::{client::Kdf, error::Result};

#[cfg(feature = "internal")]
fn determine_password_hash(
    email: &str,
    kdf: &Kdf,
    password: &str,
    purpose: HashPurpose,
) -> Result<String> {
    let master_key = MasterKey::derive(password.as_bytes(), email.as_bytes(), kdf)?;
    Ok(master_key.derive_master_key_hash(password.as_bytes(), purpose)?)
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU32;

    use super::*;

    #[cfg(feature = "internal")]
    #[test]
    fn test_determine_password_hash() {
        use super::determine_password_hash;

        let password = "password123";
        let email = "test@bitwarden.com";
        let kdf = Kdf::PBKDF2 {
            iterations: NonZeroU32::new(100_000).unwrap(),
        };
        let purpose = HashPurpose::LocalAuthorization;

        let result = determine_password_hash(email, &kdf, password, purpose).unwrap();

        assert_eq!(result, "7kTqkF1pY/3JeOu73N9kR99fDDe9O1JOZaVc7KH3lsU=");
    }
}
