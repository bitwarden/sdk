use std::path::PathBuf;

pub(crate) fn get_state_file_path(
    state_file_dir: Option<PathBuf>,
    access_token_id: String,
) -> Option<PathBuf> {
    if let Some(mut state_file_path) = state_file_dir {
        state_file_path.push(access_token_id);

        if let Some(parent_folder) = state_file_path.parent() {
            std::fs::create_dir_all(parent_folder).expect("Parent directory should be writable");
        }

        return Some(state_file_path);
    }

    None
}
