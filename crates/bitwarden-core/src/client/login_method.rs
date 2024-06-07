use std::path::PathBuf;

use bitwarden_crypto::Kdf;
use uuid::Uuid;

use crate::auth::AccessToken;

#[derive(Debug)]
pub(crate) enum LoginMethod {
    #[cfg(feature = "internal")]
    User(UserLoginMethod),
    // TODO: Organizations supports api key
    // Organization(OrganizationLoginMethod),
    ServiceAccount(ServiceAccountLoginMethod),
}

#[derive(Debug)]
#[cfg(feature = "internal")]
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

#[derive(Debug)]
pub(crate) enum ServiceAccountLoginMethod {
    AccessToken {
        access_token: AccessToken,
        organization_id: Uuid,
        state_file: Option<PathBuf>,
    },
}
