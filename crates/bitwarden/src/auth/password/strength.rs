use std::collections::HashSet;

use zxcvbn::zxcvbn;

const GLOBAL_INPUTS: [&str; 3] = ["bitwarden", "bit", "warden"];

pub(crate) fn password_strength(
    password: String,
    email: String,
    additional_inputs: Vec<String>,
) -> u8 {
    let email_parts = email_to_user_inputs(&email);
    let email_ref: Vec<_> = email_parts.iter().map(String::as_str).collect();

    let mut additional_inputs: HashSet<_> = additional_inputs.iter().map(String::as_str).collect();
    additional_inputs.extend(&GLOBAL_INPUTS);
    additional_inputs.extend(&email_ref);
    let arr: Vec<_> = additional_inputs.into_iter().collect();

    let estimate = zxcvbn(&password, &arr);

    match estimate {
        Ok(estimate) => estimate.score(),
        _ => 0,
    }
}

fn email_to_user_inputs(email: &str) -> Vec<String> {
    let parts = email.split_once('@');
    match parts {
        Some((prefix, _)) => prefix
            .trim()
            .to_lowercase()
            .split(|c: char| !c.is_alphanumeric())
            .map(str::to_owned)
            .collect(),
        None => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::{email_to_user_inputs, password_strength};

    #[test]
    fn test_password_strength() {
        let cases = vec![
            ("password", "random@bitwarden.com", 0),
            ("password11", "random@bitwarden.com", 1),
            ("Weakpass2", "random@bitwarden.com", 2),
            ("GoodPass3!", "random@bitwarden.com", 3),
            ("VeryStrong123@#", "random@bitwarden.com", 4),
        ];

        for (password, email, expected) in cases {
            let result = password_strength(password.to_owned(), email.to_owned(), vec![]);
            assert_eq!(expected, result, "{email}: {password}");
        }
    }

    #[test]
    fn test_password_strength_4() {
        let password = "VeryStrong123@#";
        let email = "random@bitwarden.com";

        let result = password_strength(password.to_owned(), email.to_owned(), vec![]);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_penalize_email() {
        let password = "asdfjkhkjwer!";

        let result = password_strength(
            password.to_owned(),
            "random@bitwarden.com".to_owned(),
            vec![],
        );
        assert_eq!(result, 4);

        let result = password_strength(
            password.to_owned(),
            "asdfjkhkjwer@bitwarden.com".to_owned(),
            vec![],
        );
        assert_eq!(result, 1);
    }

    #[test]
    fn test_email_to_user_inputs() {
        let email = "random@bitwarden.com";
        let result = email_to_user_inputs(email);

        assert_eq!(result, vec!["random"]);
    }
}
