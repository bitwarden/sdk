mod client_projects;
mod client_secrets;
pub mod projects;
pub mod secrets;

pub use client_projects::{ClientProjects, ClientProjectsExt};
pub use client_secrets::{ClientSecrets, ClientSecretsExt};

macro_rules! require {
    ($val:expr) => {
        match $val {
            Some(val) => val,
            None => {
                return Err(bitwarden_core::error::Error::MissingFields(stringify!(
                    $val
                )))
            }
        }
    };
}
pub(crate) use require;
