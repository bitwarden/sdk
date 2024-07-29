use std::path::PathBuf;

use color_eyre::eyre::Result;

pub(crate) fn get_state_file(
    state_dir: Option<PathBuf>,
    access_token_id: String,
) -> Result<Option<PathBuf>> {
    if let Some(mut state_dir) = state_dir {
        std::fs::create_dir_all(&state_dir)?;
        state_dir.push(access_token_id);

        return Ok(Some(state_dir));
    }

    Ok(None)
}
