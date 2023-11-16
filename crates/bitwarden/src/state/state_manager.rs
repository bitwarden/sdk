use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    client::{AccessToken, AccessTokenState, ClientState},
    error::{Error, Result},
};
use std::{
    fmt::Debug,
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

const STATE_VERSION: u32 = 1;

#[derive(Serialize, Debug, Deserialize)]
pub struct StateManager {
    pub version: u32,
    pub data: serde_json::Value,
}

impl StateManager {
    pub fn new(path: &Path) -> Result<Self> {
        match fs::canonicalize(path) {
            Ok(p) => {
                match p.try_exists() {
                    Ok(exists) => {
                        if exists {
                            println!("Path: {:?}", path);
                            println!("Path attempt: {:?}", p);
                            let file_content = fs::read_to_string(&p)?;
                            let file_state: Self = serde_json::from_str(file_content.as_str())?;
        
                            if file_state.version != STATE_VERSION {
                                return Err(Error::InvalidStateManagerFileVersion);
                            }
        
                            return Ok(file_state);
                        }
                    }
                    Err(e) => return Err(Error::Io(e)),
                }
            },
            Err(e) => {
                println!("Error attempting to canonicalize... {:?}", e);
                return Err(Error::Io(e))
            },
        }

        Ok(Self {
            version: STATE_VERSION,
            data: serde_json::Value::Null,
        })
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let file_content = serde_json::to_string(&self)?;
        println!("save is hit, here is the path: {:?}", path);
        println!("save is hit, here is the data: {:?}", file_content);
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)?;
        file.write_all(&file_content.as_bytes())?;
        Ok(())
    }

    pub fn has_data(&self) -> bool {
        self.data != serde_json::Value::Null
    }

    pub fn get_client_state(&self) -> Result<ClientState> {
        if !self.has_data() {
            return Err(Error::NoData);
        }

        Ok(ClientState {
            token: serde_json::from_value(self.data["token"].clone())?,
            token_expiry_timestamp: serde_json::from_value(
                self.data["token_expiry_timestamp"].clone(),
            )?,
            refresh_token: serde_json::from_value(self.data["refresh_token"].clone())?,
            access_token: serde_json::from_value(self.data["access_token"].clone())?,
        })
    }
}
