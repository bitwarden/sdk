use std::path::PathBuf;

use color_eyre::eyre::{eyre, Result};
use directories::BaseDirs;

pub(crate) const DEFAULT_STATE_DIRECTORY: &str = ".config/bws/state";

pub(crate) fn get_state_file(
    state_dir: Option<PathBuf>,
    access_token_id: String,
) -> Result<PathBuf> {
    let mut state_dir = match state_dir {
        Some(state_dir) => state_dir,
        None => {
            let Some(base_dirs) = BaseDirs::new() else {
                return Err(eyre!("A valid home directory doesn't exist"));
            };

            base_dirs.home_dir().join(DEFAULT_STATE_DIRECTORY)
        }
    };

    std::fs::create_dir_all(&state_dir)?;
    state_dir.push(access_token_id);

    Ok(state_dir)
}
