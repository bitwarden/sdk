use regex::Regex;

const VALID_POSIX_NAME_REGEX: &str = "^[a-zA-Z_][a-zA-Z0-9_]*$";

pub(crate) fn is_valid_posix_name(input_text: &str) -> bool {
    match Regex::new(VALID_POSIX_NAME_REGEX) {
        Ok(r) => r.is_match(input_text),
        Err(_) => false,
    }
}
