use crate::{
    client::{LoginMethod, UserLoginMethod},
    crypto::HashPurpose,
    error::{Error, Result},
    Client,
};
use schemars::JsonSchema;

use super::determine_password_hash;

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

/// Validate if the provided password matches the password hash stored in the client.
pub(super) async fn validate_password(
    client: &Client,
    password: String,
    password_hash: String,
) -> Result<bool> {
    let login_method = client
        .login_method
        .as_ref()
        .ok_or(Error::NotAuthenticated)?;

    if let LoginMethod::User(login_method) = login_method {
        match login_method {
            UserLoginMethod::Username { email, kdf, .. }
            | UserLoginMethod::ApiKey { email, kdf, .. } => {
                let hash =
                    determine_password_hash(email, kdf, &password, HashPurpose::LocalAuthorization)
                        .await?;

                Ok(hash == password_hash)
            }
        }
    } else {
        Err(Error::NotAuthenticated)
    }
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
