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

mod tests {
    use super::*;

    #[test]
    fn test_is_valid_posix_name_true() {
        assert!(is_valid_posix_name("a_valid_name"));
        assert!(is_valid_posix_name("another_valid_name"));
        assert!(is_valid_posix_name("_another_valid_name"));
        assert!(is_valid_posix_name("ANOTHER_ONE"));
        assert!(is_valid_posix_name(
            "abcdefghijklmnopqrstuvwxyz__ABCDEFGHIJKLMNOPQRSTUVWXYZ__0123456789"
        ));
    }

    #[test]
    fn test_is_valid_posix_name_false() {
        assert!(!is_valid_posix_name(""));
        assert!(!is_valid_posix_name("1a"));
        assert!(!is_valid_posix_name("a bad name"));
        assert!(!is_valid_posix_name("another-bad-name"));
        assert!(!is_valid_posix_name("a\nbad\nname"));
    }

    #[test]
    fn test_uuid_to_posix_success() {
        assert_eq!(
            "_759130d0_29dd_48bd_831a_e3bdbafeeb6e",
            uuid_to_posix(
                &uuid::Uuid::parse_str("759130d0-29dd-48bd-831a-e3bdbafeeb6e").expect("valid uuid")
            )
        );
        assert!(is_valid_posix_name(&uuid_to_posix(&uuid::Uuid::new_v4())));
    }
}
