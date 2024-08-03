mod secret_generator;

pub(crate) use secret_generator::generate_secret;
pub use secret_generator::{GenerateSecretError, GenerateSecretRequest, GenerateSecretResponse};
