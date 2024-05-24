use std::collections::HashMap;

use bitwarden_api_api::models::PolicyResponseModel;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use uuid::Uuid;

use crate::error::{require, Error, Result};

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
            id: require!(policy.id),
            organization_id: require!(policy.organization_id),
            r#type: require!(policy.r#type).into(),
            data: policy.data,
            enabled: require!(policy.enabled),
        })
    }
}

impl From<bitwarden_api_api::models::PolicyType> for PolicyType {
    fn from(policy_type: bitwarden_api_api::models::PolicyType) -> Self {
        match policy_type {
            bitwarden_api_api::models::PolicyType::TwoFactorAuthentication => {
                PolicyType::TwoFactorAuthentication
            }
            bitwarden_api_api::models::PolicyType::MasterPassword => PolicyType::MasterPassword,
            bitwarden_api_api::models::PolicyType::PasswordGenerator => {
                PolicyType::PasswordGenerator
            }
            bitwarden_api_api::models::PolicyType::SingleOrg => PolicyType::SingleOrg,
            bitwarden_api_api::models::PolicyType::RequireSso => PolicyType::RequireSso,
            bitwarden_api_api::models::PolicyType::PersonalOwnership => {
                PolicyType::PersonalOwnership
            }
            bitwarden_api_api::models::PolicyType::DisableSend => PolicyType::DisableSend,
            bitwarden_api_api::models::PolicyType::SendOptions => PolicyType::SendOptions,
            bitwarden_api_api::models::PolicyType::ResetPassword => PolicyType::ResetPassword,
            bitwarden_api_api::models::PolicyType::MaximumVaultTimeout => {
                PolicyType::MaximumVaultTimeout
            }
            bitwarden_api_api::models::PolicyType::DisablePersonalVaultExport => {
                PolicyType::DisablePersonalVaultExport
            }
            bitwarden_api_api::models::PolicyType::ActivateAutofill => PolicyType::ActivateAutofill,
        }
    }
}
