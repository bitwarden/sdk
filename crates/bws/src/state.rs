use std::path::PathBuf;

use color_eyre::eyre::Result;

pub(crate) fn get_state_file(
    state_path: Option<PathBuf>,
    access_token_id: String,
) -> Result<Option<PathBuf>> {
    if let Some(mut state_path) = state_path {
        std::fs::create_dir_all(&state_path)?;
        state_path.push(access_token_id);

        return Ok(Some(state_path));
    }

    Ok(None)
}
