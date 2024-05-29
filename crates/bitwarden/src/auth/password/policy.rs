use bitwarden_crypto::SensitiveString;
use schemars::JsonSchema;

/// Validate the provided password passes the provided Master Password Requirements Policy.
pub(crate) fn satisfies_policy(
    password: SensitiveString,
    strength: u8,
    policy: &MasterPasswordPolicyOptions,
) -> bool {
    if policy.min_complexity > 0 && policy.min_complexity > strength {
        return false;
    }

    if policy.min_length > 0 && usize::from(policy.min_length) > password.len() {
        return false;
    }

    if policy.require_upper && !password.any_chars(char::is_uppercase) {
        return false;
    }

    if policy.require_lower && !password.any_chars(char::is_lowercase) {
        return false;
    }

    if policy.require_numbers && !password.any_chars(char::is_numeric) {
        return false;
    }

    if policy.require_special && !password.any_chars(|c| "!@#$%^&*".contains(c)) {
        return false;
    }

    true
}

#[derive(Debug, JsonSchema)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[allow(dead_code)]
pub struct MasterPasswordPolicyOptions {
    min_complexity: u8,
    min_length: u8,
    require_upper: bool,
    require_lower: bool,
    require_numbers: bool,
    require_special: bool,

    /// Flag to indicate if the policy should be enforced on login.
    /// If true, and the user's password does not meet the policy requirements,
    /// the user will be forced to update their password.
    enforce_on_login: bool,
}

#[cfg(test)]
mod tests {

    use bitwarden_crypto::SensitiveString;

    use super::{satisfies_policy, MasterPasswordPolicyOptions};

    #[test]
    fn satisfies_policy_gives_success() {
        let password = SensitiveString::test("lkasfo!icbb$2323ALKJCO22");
        let options = MasterPasswordPolicyOptions {
            min_complexity: 3,
            min_length: 5,
            require_upper: true,
            require_lower: true,
            require_numbers: true,
            require_special: true,
            enforce_on_login: false,
        };

        let result = satisfies_policy(password, 4, &options);
        assert!(result);
    }

    #[test]
    fn satisfies_policy_evaluates_strength() {
        let password = SensitiveString::test("password123");
        let options = MasterPasswordPolicyOptions {
            min_complexity: 3,
            min_length: 0,
            require_upper: false,
            require_lower: false,
            require_numbers: false,
            require_special: false,
            enforce_on_login: false,
        };

        let result = satisfies_policy(password, 0, &options);
        assert!(!result);
    }

    #[test]
    fn satisfies_policy_evaluates_length() {
        let password = SensitiveString::test("password123");
        let options = MasterPasswordPolicyOptions {
            min_complexity: 0,
            min_length: 20,
            require_upper: false,
            require_lower: false,
            require_numbers: false,
            require_special: false,
            enforce_on_login: false,
        };

        let result = satisfies_policy(password, 0, &options);
        assert!(!result);
    }

    #[test]
    fn satisfies_policy_evaluates_upper() {
        let password = SensitiveString::test("password123");
        let options = MasterPasswordPolicyOptions {
            min_complexity: 0,
            min_length: 0,
            require_upper: true,
            require_lower: false,
            require_numbers: false,
            require_special: false,
            enforce_on_login: false,
        };

        let result = satisfies_policy(password, 0, &options);
        assert!(!result);
    }

    #[test]
    fn satisfies_policy_evaluates_lower() {
        let password = SensitiveString::test("ABCDEFG123");
        let options = MasterPasswordPolicyOptions {
            min_complexity: 0,
            min_length: 0,
            require_upper: false,
            require_lower: true,
            require_numbers: false,
            require_special: false,
            enforce_on_login: false,
        };

        let result = satisfies_policy(password, 0, &options);
        assert!(!result);
    }

    #[test]
    fn satisfies_policy_evaluates_numbers() {
        let password = SensitiveString::test("password");
        let options = MasterPasswordPolicyOptions {
            min_complexity: 0,
            min_length: 0,
            require_upper: false,
            require_lower: false,
            require_numbers: true,
            require_special: false,
            enforce_on_login: false,
        };

        let result = satisfies_policy(password, 0, &options);
        assert!(!result);
    }

    #[test]
    fn satisfies_policy_evaluates_special() {
        let password = SensitiveString::test("Password123");
        let options = MasterPasswordPolicyOptions {
            min_complexity: 0,
            min_length: 0,
            require_upper: false,
            require_lower: false,
            require_numbers: false,
            require_special: true,
            enforce_on_login: false,
        };

        let result = satisfies_policy(password, 0, &options);
        assert!(!result);
    }
}
