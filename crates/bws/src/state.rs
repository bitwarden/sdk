use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use bitwarden::{
    client::{AccessToken, ClientState},
    crypto::{EncString, KeyDecryptable, KeyEncryptable},
    error::{Error, Result},
    state::StateManager,
};
use directories::BaseDirs;
use serde_json::json;
use sha2::Digest;

type AccessTokenHash = String;
type EncClientState = EncString;

pub(crate) const ROOT_DIRECTORY: &str = ".bws";
pub(crate) const STATE_DIRECTORY: &str = "state";
pub(crate) const FILENAME: &str = "state";

pub struct State {
    state_manager: StateManager,
    data: HashMap<AccessTokenHash, EncClientState>,
    access_token: AccessToken,
    access_token_hash: AccessTokenHash,
}

impl State {
    pub fn new(path: &Path, access_token: String) -> Result<Self> {
        let state_manager = StateManager::new(path)?;
        let data: HashMap<AccessTokenHash, EncClientState> = if state_manager.has_data() {
            serde_json::from_str(state_manager.data.to_string().as_str())?
        } else {
            Default::default()
        };
        let access_token_hash: String = format!(
            "{:X}",
            sha2::Sha256::new()
                .chain_update(access_token.clone())
                .finalize()
        );
        let access_token: AccessToken = access_token.parse()?;

        Ok(Self {
            state_manager,
            data,
            access_token,
            access_token_hash,
        })
    }

    pub fn get(&self) -> Option<Result<ClientState>> {
        match self.data.get(&self.access_token_hash) {
            Some(encrypted_data) => {
                let decrypted_data: Result<String> =
                    encrypted_data.decrypt_with_key(&self.access_token.encryption_key);
                match decrypted_data {
                    Ok(decrypted_data) => match serde_json::from_str(decrypted_data.as_str()) {
                        Ok(state) => Some(Ok(state)),
                        Err(e) => Some(Err(Error::Serde(e))),
                    },
                    Err(e) => Some(Err(e)),
                }
            }
            None => None,
        }
    }

    pub fn upsert(&mut self, new_state: ClientState) -> Result<()> {
        let serialized_state = json!(new_state).to_string();
        let enc_state = serialized_state.encrypt_with_key(&self.access_token.encryption_key)?;
        self.data.insert(self.access_token_hash.clone(), enc_state);
        self.state_manager.data = json!(self.data);

        Ok(())
    }

    pub fn save(&mut self, path: &Path) -> Result<()> {
        self.state_manager.data = json!(self.data);
        self.state_manager.save(path)
    }
}

pub(crate) fn get_state_file_path(
    state_file: Option<PathBuf>,
    profile: Option<String>,
    ensure_folder_exists: bool,
) -> PathBuf {
    let state_file = state_file.unwrap_or_else(|| {
        let base_dirs = BaseDirs::new().unwrap();
        let state_filename = match profile {
            Some(p) => p + "-" + FILENAME,
            None => "default-".to_string() + FILENAME,
        };

        base_dirs
            .home_dir()
            .join(ROOT_DIRECTORY)
            .join(STATE_DIRECTORY)
            .join(state_filename)
    });

    if ensure_folder_exists {
        if let Some(parent_folder) = state_file.parent() {
            std::fs::create_dir_all(parent_folder).unwrap();
        }
    }

    state_file
}
