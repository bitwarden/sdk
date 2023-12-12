use std::path::PathBuf;

use directories::BaseDirs;

pub(crate) const BWS_DIRECTORY: &str = ".bws";
pub(crate) const STATE_DIRECTORY: &str = "state";
pub(crate) const STATE_SUFFIX: &str = "-state";
pub(crate) const DEFAULT_STATE_FILENAME: &str = "default-state";

pub(crate) fn get_state_file_path(state_file: Option<PathBuf>, profile: Option<String>) -> PathBuf {
    let state_file = state_file.unwrap_or_else(|| {
        let base_dirs = BaseDirs::new().unwrap();
        let state_filename = match profile {
            Some(p) => p + STATE_SUFFIX,
            None => DEFAULT_STATE_FILENAME.to_string(),
        };

        base_dirs
            .home_dir()
            .join(BWS_DIRECTORY)
            .join(STATE_DIRECTORY)
            .join(state_filename)
    });

    if let Some(parent_folder) = state_file.parent() {
        std::fs::create_dir_all(parent_folder).unwrap();
    }

    state_file
}
