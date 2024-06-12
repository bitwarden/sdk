use regex::Regex;
use uuid::Uuid;

const VALID_POSIX_NAME_REGEX: &str = "^[a-zA-Z_][a-zA-Z0-9_]*$";

pub(crate) fn is_valid_posix_name(input_text: &str) -> bool {
    match Regex::new(VALID_POSIX_NAME_REGEX) {
        Ok(r) => r.is_match(input_text),
        Err(_) => false,
    }
}

pub(crate) fn uuid_to_posix(uuid: &Uuid) -> String {
    // POSIX environment variable names must start with a letter or an underscore
    // and can only contain letters, numbers, and underscores.
    format!("_{}", uuid.to_string().replace('-', "_"))
}
