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
pub use register::{RegisterKeyResponse, RegisterRequest};

#[cfg(feature = "internal")]
use crate::{
    client::kdf::Kdf,
    crypto::{HashPurpose, MasterKey},
    error::Result,
};

#[cfg(feature = "internal")]
async fn determine_password_hash(
    email: &str,
    kdf: &Kdf,
    password: &str,
    purpose: HashPurpose,
) -> Result<String> {
    let master_key = MasterKey::derive(password.as_bytes(), email.as_bytes(), kdf)?;
    master_key.derive_master_key_hash(password.as_bytes(), purpose)
}
