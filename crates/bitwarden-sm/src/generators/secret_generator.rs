use bitwarden_generators::{password, PasswordError, PasswordGeneratorRequest};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GenerateSecretError {
    #[error("No character set enabled")]
    NoCharacterSetEnabled,
    #[error("Invalid secret length")]
    InvalidLength,
}

pub struct GenerateSecretRequest {
    /// Include lowercase characters (a-z).
    pub include_lowercase: bool,
    /// Include uppercase characters (A-Z).
    pub include_uppercase: bool,
    /// Include numbers (0-9).
    pub include_numbers: bool,
    /// Include special characters: ! @ # $ % ^ & *
    pub include_special: bool,

    /// The length of the generated secret.
    /// Note that the secret length must be greater than the sum of all the minimums.
    pub length: u8,

    /// When set to true, the generated secret will not contain ambiguous characters.
    /// The ambiguous characters are: I, O, l, 0, 1
    pub avoid_ambiguous: bool, // TODO: Should we rename this to include_all_characters?

    /// The minimum number of lowercase characters in the generated secret.
    /// When set, the value must be between 1 and 9. This value is ignored if include_lowercase is
    /// false.
    pub min_lowercase: Option<u8>,
    /// The minimum number of uppercase characters in the generated secret.
    /// When set, the value must be between 1 and 9. This value is ignored if include_uppercase is
    /// false.
    pub min_uppercase: Option<u8>,
    /// The minimum number of numbers in the generated secret.
    /// When set, the value must be between 1 and 9. This value is ignored if include_numbers is
    /// false.
    pub min_number: Option<u8>,
    /// The minimum number of special characters in the generated secret.
    /// When set, the value must be between 1 and 9. This value is ignored if include_special is
    /// false.
    pub min_special: Option<u8>,
}

pub struct GenerateSecretResponse {
    pub secret: String,
}

pub(crate) async fn generate_secret(
    request: GenerateSecretRequest,
) -> Result<GenerateSecretResponse, GenerateSecretError> {
    let password_generator_request = PasswordGeneratorRequest {
        lowercase: request.include_lowercase,
        uppercase: request.include_uppercase,
        numbers: request.include_numbers,
        special: request.include_numbers,
        length: request.length,
        avoid_ambiguous: request.avoid_ambiguous,
        min_lowercase: request.min_lowercase,
        min_uppercase: request.min_uppercase,
        min_number: request.min_number,
        min_special: request.min_special,
    };

    let response = password(password_generator_request);

    match response {
        Ok(secret) => Ok(GenerateSecretResponse { secret }),
        Err(e) => match e {
            PasswordError::NoCharacterSetEnabled => Err(GenerateSecretError::NoCharacterSetEnabled),
            PasswordError::InvalidLength => Err(GenerateSecretError::InvalidLength),
        },
    }
}
