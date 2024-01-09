use zxcvbn::zxcvbn;

const GLOBAL_INPUTS: [&str; 3] = ["bitwarden", "bit", "warden"];

pub(crate) fn password_strength(
    password: String,
    email: String,
    additional_inputs: Vec<String>,
) -> u8 {
    let mut inputs = email_to_user_inputs(&email);
    inputs.extend(additional_inputs);

    let mut arr: Vec<_> = inputs.iter().map(String::as_str).collect();
    arr.extend(GLOBAL_INPUTS);

    zxcvbn(&password, &arr).map_or(0, |e| e.score())
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
