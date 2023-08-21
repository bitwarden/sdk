use std::fmt::{Display, Formatter};

// Name is converted from *Error to *Exception, so we can't just name the enum Error because Exception already exists
#[derive(uniffi::Error)]
#[uniffi(flat_error)]
pub enum BitwardenError {
    E(bitwarden::error::Error),
}

impl From<bitwarden::error::Error> for BitwardenError {
    fn from(e: bitwarden::error::Error) -> Self {
        Self::E(e)
    }
}

impl Display for BitwardenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::E(e) => Display::fmt(e, f),
        }
    }
}

pub type Result<T, E = BitwardenError> = std::result::Result<T, E>;
