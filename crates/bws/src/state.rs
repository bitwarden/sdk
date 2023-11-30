use std::path::PathBuf;

use directories::BaseDirs;
use serde::{Deserialize, Serialize};

pub(crate) const ROOT_DIRECTORY: &str = ".bws";
pub(crate) const STATE_DIRECTORY: &str = "state";
pub(crate) const FILENAME: &str = "state";

#[derive(Serialize, Deserialize)]
struct State {
    version: u32,
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
