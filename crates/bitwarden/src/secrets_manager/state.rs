use std::{fmt::Debug, path::Path};

use bitwarden_crypto::{EncString, KeyDecryptable, KeyEncryptable};
use serde::{Deserialize, Serialize};

use crate::{
    client::AccessToken,
    error::{Error, Result},
};

const STATE_VERSION: u32 = 1;

#[cfg(feature = "secrets")]
#[derive(Serialize, Deserialize, Debug)]
pub struct ClientState {
    pub(crate) version: u32,
    pub(crate) token: String,
    pub(crate) encryption_key: String,
}

impl ClientState {
    pub fn new(token: String, encryption_key: String) -> Self {
        Self {
            version: STATE_VERSION,
            token,
            encryption_key,
        }
    }
}

pub fn get(state_file: &Path, access_token: &AccessToken) -> Result<ClientState> {
    let file_content = std::fs::read_to_string(state_file)?;

    let encrypted_state: EncString = file_content.parse()?;
    let decrypted_state: String = encrypted_state.decrypt_with_key(&access_token.encryption_key)?;
    let client_state: ClientState = serde_json::from_str(&decrypted_state)?;

    if client_state.version != STATE_VERSION {
        return Err(Error::InvalidStateFileVersion);
    }

    Ok(client_state)
}

pub fn set(state_file: &Path, access_token: &AccessToken, state: ClientState) -> Result<()> {
    let serialized_state: String = serde_json::to_string(&state)?;
    let encrypted_state: EncString =
        serialized_state.encrypt_with_key(&access_token.encryption_key)?;
    let state_string: String = encrypted_state.to_string();

    std::fs::write(state_file, state_string)
        .map_err(|_| "Failure writing to the state file.".into())
}
