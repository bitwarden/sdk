use bitwarden_crypto::{HashPurpose, Kdf, MasterKey, SensitiveString};

use crate::{error::Result, Client};

pub async fn hash_password(
    _client: &Client,
    email: String,
    password: SensitiveString,
    kdf_params: Kdf,
    purpose: HashPurpose,
) -> Result<SensitiveString> {
    let password_vec = password.into();
    let master_key = MasterKey::derive(&password_vec, email.as_bytes(), &kdf_params)?;

    Ok(master_key.derive_master_key_hash(&password_vec, purpose)?)
}
