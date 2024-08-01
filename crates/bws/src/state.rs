use std::path::PathBuf;

use color_eyre::eyre::Result;

pub(crate) const DEFAULT_STATE_DIRECTORY: &str = ".config/bws/state";

pub(crate) fn get_state_file(
    state_dir: Option<PathBuf>,
    access_token_id: String,
) -> Result<PathBuf> {
    let mut state_dir = match state_dir {
        Some(state_dir) => state_dir,
        None => PathBuf::from(DEFAULT_STATE_DIRECTORY),
    };

    std::fs::create_dir_all(&state_dir)?;
    state_dir.push(access_token_id);

    Ok(state_dir)
}
