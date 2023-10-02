use schemars::JsonSchema;

pub(super) fn password_strength(
    _password: String,
    _email: String,
    _additional_inputs: Vec<String>,
) -> u8 {
    2
}

pub(super) fn satisfies_policy(
    _password: String,
    _strength: u8,
    _policy: &MasterPasswordPolicyOptions,
) -> bool {
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
