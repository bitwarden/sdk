use std::{
    fmt::Debug,
    path::{Path, PathBuf},
};

use bitwarden_crypto::{EncString, KeyDecryptable, KeyEncryptable};
use serde::{Deserialize, Serialize};

use crate::{
    auth::{login::AccessTokenLoginState, AccessToken},
    error::{Error, Result},
};

const STATE_VERSION: u32 = 1;
const DEFAULT_RELATIVE_STATE_DIR: &str = "bitwarden/sdk/state";

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

    std::fs::create_dir_all(state_file.parent().unwrap())?;

    std::fs::write(state_file, state_string)
        .map_err(|_| "Failure writing to the state file.".into())
}

pub fn build_state_file_path(
    access_token: &AccessToken,
    state: AccessTokenLoginState,
) -> Result<Option<PathBuf>> {
    match state {
        AccessTokenLoginState::Default => {
            if let Some(mut home_dir) = dirs::config_dir() {
                home_dir.push(DEFAULT_RELATIVE_STATE_DIR);
                home_dir.push(access_token.access_token_id.to_string());

                println!("Using state file: {:?}", home_dir);

                return Ok(Some(home_dir));
            }

            Err(Error::Internal(
                "Unable to find home directory, please specify a custom state directory when logging in with an access token".into(),
            ))
        }
        AccessTokenLoginState::CustomDirectory(mut state_dir) => {
            state_dir.push(access_token.access_token_id.to_string());

            Ok(Some(state_dir))
        }
        AccessTokenLoginState::OptOut => Ok(None),
    }
}
