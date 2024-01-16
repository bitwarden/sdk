use thiserror::Error;

use crate::{passphrase::PassphraseError, password::PasswordError, username::UsernameError};

#[derive(Debug, Error)]
pub enum GeneratorError {
    #[error(transparent)]
    PassphraseErrors(#[from] PassphraseError),
    #[error(transparent)]
    PasswordError(#[from] PasswordError),
    #[error(transparent)]
    UsernameError(#[from] UsernameError),
}
