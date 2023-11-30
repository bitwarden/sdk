use std::{error, fmt};

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl<T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl<T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub mod access_policies_api;
pub mod accounts_api;
pub mod accounts_billing_api;
pub mod auth_requests_api;
pub mod ciphers_api;
pub mod collections_api;
pub mod config_api;
pub mod devices_api;
pub mod emergency_access_api;
pub mod events_api;
pub mod folders_api;
pub mod groups_api;
pub mod hibp_api;
pub mod import_ciphers_api;
pub mod info_api;
pub mod installations_api;
pub mod licenses_api;
pub mod misc_api;
pub mod organization_auth_requests_api;
pub mod organization_connections_api;
pub mod organization_domain_api;
pub mod organization_export_api;
pub mod organization_sponsorships_api;
pub mod organization_users_api;
pub mod organizations_api;
pub mod plans_api;
pub mod policies_api;
pub mod projects_api;
pub mod provider_organizations_api;
pub mod provider_users_api;
pub mod providers_api;
pub mod push_api;
pub mod secrets_api;
pub mod secrets_manager_porting_api;
pub mod self_hosted_organization_licenses_api;
pub mod self_hosted_organization_sponsorships_api;
pub mod sends_api;
pub mod service_accounts_api;
pub mod settings_api;
pub mod sync_api;
pub mod trash_api;
pub mod two_factor_api;
pub mod users_api;

pub mod configuration;
