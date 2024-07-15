use bitwarden_crypto::{HashPurpose, Kdf, MasterKey};

use crate::error::Result;

pub async fn hash_password(
    email: String,
    password: String,
    kdf_params: Kdf,
    purpose: HashPurpose,
) -> Result<String> {
    let master_key = MasterKey::derive(&password, &email, &kdf_params)?;

    Ok(master_key.derive_master_key_hash(password.as_bytes(), purpose)?)
}
