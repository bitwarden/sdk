const STRING_TO_BOOL_ERROR_MESSAGE: &str = "Could not convert string to bool";

pub(crate) fn string_to_bool(value: &str) -> Result<bool, &str> {
    match value.trim().to_lowercase().as_str() {
        "true" | "1" => Ok(true),
        "false" | "0" => Ok(false),
        _ => Err(STRING_TO_BOOL_ERROR_MESSAGE),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_bool_true_true() {
        let result = string_to_bool("true");
        assert_eq!(result, Ok(true));
    }

    #[test]
    fn test_string_to_bool_one_true() {
        let result = string_to_bool("1");
        assert_eq!(result, Ok(true));
    }

    #[test]
    fn test_string_to_bool_false_false() {
        let result = string_to_bool("false");
        assert_eq!(result, Ok(false));
    }

    #[test]
    fn test_string_to_bool_zero_false() {
        let result = string_to_bool("0");
        assert_eq!(result, Ok(false));
    }

    #[test]
    fn test_string_to_bool_bad_string_errors() {
        let result = string_to_bool("hello world");
        assert_eq!(result, Err(STRING_TO_BOOL_ERROR_MESSAGE));
    }
}
