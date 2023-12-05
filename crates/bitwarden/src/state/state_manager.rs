use base64::Engine;
use serde::{Deserialize, Serialize};

use crate::{
    client::{AccessToken, ClientState},
    crypto::{EncString, KeyDecryptable, KeyEncryptable},
    error::{Error, Result},
    util::BASE64_ENGINE,
};
use sha2::Digest;
use std::{
    collections::HashMap,
    fmt::Debug,
    fs::OpenOptions,
    io::{Read, Seek, Write},
    path::PathBuf,
};

type AccessTokenHash = String;
type EncClientState = EncString;

const STATE_VERSION: u32 = 1;

#[derive(Serialize, Debug, Deserialize)]
pub struct StateFileData {
    pub version: u32,
    data: HashMap<AccessTokenHash, EncClientState>,
}

impl StateFileData {
    pub fn new() -> Self {
        StateFileData {
            version: STATE_VERSION,
            data: Default::default(),
        }
    }

    fn get(&self, access_token: &String) -> Result<Option<ClientState>> {
        let access_token_hash: String =
            BASE64_ENGINE.encode(sha2::Sha256::new().chain_update(access_token).finalize());
        let access_token: AccessToken = access_token.parse()?;

        let Some(enc_data) = self.data.get(&access_token_hash) else {
            return Ok(None);
        };
        let decrypted_data: String = enc_data.decrypt_with_key(&access_token.encryption_key)?;
        let state: ClientState = serde_json::from_str(&decrypted_data)?;

        if state.is_expired() {
            return Ok(None);
        }

        Ok(Some(state))
    }

    fn insert(&mut self, access_token: &String, state: ClientState) -> Result<()> {
        let access_token_hash: String =
            BASE64_ENGINE.encode(sha2::Sha256::new().chain_update(access_token).finalize());
        let access_token: AccessToken = access_token.parse()?;

        let serialized_state: String = serde_json::to_string(&state)?;
        let enc_state: EncClientState =
            serialized_state.encrypt_with_key(&access_token.encryption_key)?;

        self.data.insert(access_token_hash, enc_state);

        Ok(())
    }
}

pub struct StateManager {
    path: PathBuf,
}

impl StateManager {
    pub fn new(path: PathBuf) -> Result<Self> {
        if let Some(parent_folder) = path.parent() {
            std::fs::create_dir_all(parent_folder)?;
        }

        Ok(Self { path })
    }

    pub fn get(&self, access_token: &String) -> Result<Option<ClientState>> {
        let file_content = std::fs::read_to_string(&self.path)?;
        let state_data: StateFileData = serde_json::from_str(&file_content)?;

        if state_data.version != STATE_VERSION {
            return Err(Error::InvalidStateManagerFileVersion);
        }

        state_data.get(access_token)
    }

    pub fn insert(&self, access_token: &String, state: ClientState) -> Result<()> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.path)?;

        // TODO: lock the file (SM-1028)

        let mut file_content = String::new();
        file.read_to_string(&mut file_content)?;

        let mut state_data: StateFileData = match serde_json::from_str(&file_content) {
            Ok(data) => data,
            Err(_) => StateFileData::new(),
        };

        state_data.insert(access_token, state)?;

        let serialized_state = serde_json::to_string(&state_data)?;

        // Truncate the file and overwrite
        file.rewind()?;
        file.set_len(0)?;
        file.write_all(serialized_state.as_bytes())?;

        Ok(())
    }
}
