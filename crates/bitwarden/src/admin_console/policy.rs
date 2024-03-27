use std::collections::HashMap;

use bitwarden_api_api::models::PolicyResponseModel;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use uuid::Uuid;

use crate::error::{Error, Result};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct Policy {
    id: Uuid,
    organization_id: Uuid,
    r#type: PolicyType,
    data: Option<HashMap<String, serde_json::Value>>,
    enabled: bool,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, JsonSchema)]
#[repr(u8)]
pub enum PolicyType {
    TwoFactorAuthentication = 0, // Requires users to have 2fa enabled
    MasterPassword = 1,          // Sets minimum requirements for master password complexity
    PasswordGenerator = 2,       /* Sets minimum requirements/default type for generated
                                  * passwords/passphrases */
    SingleOrg = 3,         // Allows users to only be apart of one organization
    RequireSso = 4,        // Requires users to authenticate with SSO
    PersonalOwnership = 5, // Disables personal vault ownership for adding/cloning items
    DisableSend = 6,       // Disables the ability to create and edit Bitwarden Sends
    SendOptions = 7,       // Sets restrictions or defaults for Bitwarden Sends
    ResetPassword = 8,     /* Allows orgs to use reset password : also can enable
                            * auto-enrollment during invite flow */
    MaximumVaultTimeout = 9,         // Sets the maximum allowed vault timeout
    DisablePersonalVaultExport = 10, // Disable personal vault export
    ActivateAutofill = 11,           // Activates autofill with page load on the browser extension
}

impl TryFrom<PolicyResponseModel> for Policy {
    type Error = Error;

    fn try_from(policy: PolicyResponseModel) -> Result<Self> {
        Ok(Self {
            id: policy.id.ok_or(Error::MissingFields)?,
            organization_id: policy.organization_id.ok_or(Error::MissingFields)?,
            r#type: policy.r#type.ok_or(Error::MissingFields)?.try_into()?,
            data: policy.data,
            enabled: policy.enabled.ok_or(Error::MissingFields)?,
        })
    }
}

impl TryFrom<bitwarden_api_api::models::PolicyType> for PolicyType {
    type Error = Error;

    fn try_from(policy_type: bitwarden_api_api::models::PolicyType) -> Result<Self> {
        match policy_type {
            bitwarden_api_api::models::PolicyType::TwoFactorAuthentication => {
                Ok(PolicyType::TwoFactorAuthentication)
            }
            bitwarden_api_api::models::PolicyType::MasterPassword => Ok(PolicyType::MasterPassword),
            bitwarden_api_api::models::PolicyType::PasswordGenerator => {
                Ok(PolicyType::PasswordGenerator)
            }
            bitwarden_api_api::models::PolicyType::SingleOrg => Ok(PolicyType::SingleOrg),
            bitwarden_api_api::models::PolicyType::RequireSso => Ok(PolicyType::RequireSso),
            bitwarden_api_api::models::PolicyType::PersonalOwnership => {
                Ok(PolicyType::PersonalOwnership)
            }
            bitwarden_api_api::models::PolicyType::DisableSend => Ok(PolicyType::DisableSend),
            bitwarden_api_api::models::PolicyType::SendOptions => Ok(PolicyType::SendOptions),
            bitwarden_api_api::models::PolicyType::ResetPassword => Ok(PolicyType::ResetPassword),
            bitwarden_api_api::models::PolicyType::MaximumVaultTimeout => {
                Ok(PolicyType::MaximumVaultTimeout)
            }
            bitwarden_api_api::models::PolicyType::DisablePersonalVaultExport => {
                Ok(PolicyType::DisablePersonalVaultExport)
            }
            bitwarden_api_api::models::PolicyType::ActivateAutofill => {
                Ok(PolicyType::ActivateAutofill)
            }
            bitwarden_api_api::models::PolicyType::UnknownValue => Err(Error::MissingFields),
        }
    }
}
