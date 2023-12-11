use crate::{
    client::kdf::Kdf,
    crypto::{HashPurpose, MasterKey},
    error::Result,
    Client,
};

pub async fn hash_password(
    _client: &Client,
    email: String,
    password: String,
    kdf_params: Kdf,
    purpose: HashPurpose,
) -> Result<String> {
    let master_key = MasterKey::derive(password.as_bytes(), email.as_bytes(), &kdf_params)?;

    master_key.derive_master_key_hash(password.as_bytes(), purpose)
}
