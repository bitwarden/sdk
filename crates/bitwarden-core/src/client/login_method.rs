#[cfg(feature = "secrets")]
use std::path::PathBuf;

use bitwarden_crypto::Kdf;
#[cfg(feature = "secrets")]
use uuid::Uuid;

#[cfg(feature = "secrets")]
use crate::auth::AccessToken;

#[derive(Debug)]
pub(crate) enum LoginMethod {
    #[allow(dead_code)]
    User(UserLoginMethod),
    // TODO: Organizations supports api key
    // Organization(OrganizationLoginMethod),
    #[cfg(feature = "secrets")]
    ServiceAccount(ServiceAccountLoginMethod),
}

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) enum UserLoginMethod {
    Username {
        client_id: String,
        email: String,
        kdf: Kdf,
    },
    ApiKey {
        client_id: String,
        client_secret: String,

        email: String,
        kdf: Kdf,
    },
}

#[cfg(feature = "secrets")]
#[derive(Debug)]
pub(crate) enum ServiceAccountLoginMethod {
    AccessToken {
        access_token: AccessToken,
        organization_id: Uuid,
        state_file: Option<PathBuf>,
    },
}
