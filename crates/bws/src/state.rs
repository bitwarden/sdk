use std::path::PathBuf;

use directories::BaseDirs;

pub(crate) const BWS_DIRECTORY: &str = ".bws";
pub(crate) const STATE_DIRECTORY: &str = "state";

pub(crate) fn get_state_file_path(
    state_file_dir: Option<PathBuf>,
    access_token_id: String,
) -> PathBuf {
    let state_file_path = match state_file_dir {
        Some(mut sfd) => {
            sfd.push(access_token_id);

            sfd
        }
        None => {
            let base_dirs = BaseDirs::new().unwrap();

            base_dirs
                .home_dir()
                .join(BWS_DIRECTORY)
                .join(STATE_DIRECTORY)
                .join(access_token_id)
        }
    };

    if let Some(parent_folder) = state_file_path.parent() {
        std::fs::create_dir_all(parent_folder).unwrap();
    }

    state_file_path
}
