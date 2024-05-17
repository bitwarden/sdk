use schemars::JsonSchema;

/// Validate the provided password passes the provided Master Password Requirements Policy.
pub(crate) fn satisfies_policy(
    password: String,
    strength: u8,
    policy: &MasterPasswordPolicyOptions,
) -> bool {
    if policy.min_complexity > 0 && policy.min_complexity > strength {
        return false;
    }

    if policy.min_length > 0 && usize::from(policy.min_length) > password.len() {
        return false;
    }

    if policy.require_upper && password.to_lowercase() == password {
        return false;
    }

    if policy.require_lower && password.to_uppercase() == password {
        return false;
    }

    if policy.require_numbers && !password.chars().any(|c| c.is_numeric()) {
        return false;
    }

    if policy.require_special && !password.chars().any(|c| "!@#$%^&*".contains(c)) {
        return false;
    }

    true
}

#[derive(Debug, JsonSchema)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
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

    use super::{satisfies_policy, MasterPasswordPolicyOptions};

    #[test]
    fn satisfies_policy_gives_success() {
        let password = "lkasfo!icbb$2323ALKJCO22".to_string();
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
        let password = "password123".to_string();
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
        let password = "password123".to_string();
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
        let password = "password123".to_string();
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
        let password = "ABCDEFG123".to_string();
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
        let password = "password".to_string();
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
        let password = "Password123".to_string();
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
