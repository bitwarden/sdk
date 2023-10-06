use bitwarden_api_api::models::{PendingOrganizationAuthRequestResponseModelListResponseModel, PendingOrganizationAuthRequestResponseModel, OrganizationUserResetPasswordDetailsResponseModel, AdminAuthRequestUpdateRequestModel};
use rsa::pkcs8::DecodePrivateKey;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
  client::Client,
  error::{Result, Error, CryptoError},
  crypto::{EncString, Decryptable}
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AuthApproveRequest {
    /// ID of the auth request to approve
    pub request_id: Uuid,
    pub organization_user_id: Uuid,
    pub organization_id: Uuid,
    pub device_public_key: Uuid
}

pub(crate) async fn approve_auth_request(
    client: &mut Client,
    input: &AuthApproveRequest,
) -> Result<()> {
    let config = client.get_api_configurations().await;

    // Get user reset password details
    let reset_password_details =
      bitwarden_api_api::apis::organization_users_api::organizations_org_id_users_id_reset_password_details_get(
        &config.api,
        &input.organization_id.to_string(),
        &input.request_id.to_string(),
    )
    .await?;

    let encrypted_user_key = get_encrypted_user_key(client, input, reset_password_details)?;

    bitwarden_api_api::apis::organization_auth_requests_api::organizations_org_id_auth_requests_request_id_post(
        &config.api,
        input.organization_id,
        input.request_id,
        Some(AdminAuthRequestUpdateRequestModel {
          encrypted_user_key: Some(encrypted_user_key.to_string()),
          request_approved: true
        })
    )
    .await?;

    Ok(())
}

fn get_encrypted_user_key(
  client: &Client,
  input: &AuthApproveRequest,
  reset_password_details: OrganizationUserResetPasswordDetailsResponseModel) -> Result<EncString> {

  // Decrypt organization's encrypted private key with org key
  let enc = client.get_encryption_settings()?;

  let org_private_key = {
    let dec = reset_password_details.encrypted_private_key
      .ok_or(Error::MissingFields)?
      .parse::<EncString>()?
      .decrypt(enc, &Some(input.organization_id))?
      .into_bytes();

    rsa::RsaPrivateKey::from_pkcs8_der(&dec)
        .map_err(|_| CryptoError::InvalidKey)?
  };


  // Decrypt user key with decrypted org private key
  // TODO
  //   let user_key = user_reset_password_details_res.reset_password_key
  //     .ok_or(Error::MissingFields)?
  //     .parse::<EncString>()?
  // .decrypt_with_key(org_private_key)
  //     .decrypt(enc, &Some(input.organization_id))?
  //     .into_bytes();

  // Re-encrypt the User Key with the Device Public Key
  // return re-encrypted user key
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PendingAuthRequestsResponse {
    pub data: Vec<PendingAuthRequestResponse>,
}

impl PendingAuthRequestsResponse {
    pub(crate) fn process_response(
        response: PendingOrganizationAuthRequestResponseModelListResponseModel
    ) -> Result<PendingAuthRequestsResponse> {
        Ok(PendingAuthRequestsResponse {
            data: response
                .data
                .unwrap_or_default()
                .into_iter()
                .map(|r| PendingAuthRequestResponse::process_response(r))
                .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PendingAuthRequestResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub organization_user_id: Uuid,
    pub email: String,
    // TODO: map rest of fields
}

impl PendingAuthRequestResponse {
    pub(crate) fn process_response(
        response: PendingOrganizationAuthRequestResponseModel
    ) -> Result<PendingAuthRequestResponse> {
        Ok(PendingAuthRequestResponse {
          id: response.id.ok_or(Error::MissingFields)?,
          user_id: response.user_id.ok_or(Error::MissingFields)?,
          organization_user_id: response.organization_user_id.ok_or(Error::MissingFields)?,
          email: response.email.ok_or(Error::MissingFields)?,
        })
    }
}
