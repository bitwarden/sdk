use bitwarden_api_api::models::{PendingOrganizationAuthRequestResponseModelListResponseModel, PendingOrganizationAuthRequestResponseModel, OrganizationUserResetPasswordDetailsResponseModel, AdminAuthRequestUpdateRequestModel};
use rsa::{
  pkcs8::DecodePrivateKey,
  Pkcs1v15Encrypt, pkcs1::DecodeRsaPublicKey
};
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
    pub device_public_key: String
}

pub(crate) async fn approve_auth_request(
    client: &mut Client,
    input: &AuthApproveRequest,
) -> Result<()> {
    // Get user reset password details
    let reset_password_details =
      bitwarden_api_api::apis::organization_users_api::organizations_org_id_users_id_reset_password_details_get(
&client.get_api_configurations().await.api,
        &input.organization_id.to_string(),
        &input.request_id.to_string(),
    )
    .await?;

    let encrypted_user_key = get_encrypted_user_key(&client, input, reset_password_details)?;

    // Need to create a new mutable borrow
    bitwarden_api_api::apis::organization_auth_requests_api::organizations_org_id_auth_requests_request_id_post(
&client.get_api_configurations().await.api,
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

  // Decrypt user key with org private key
  let dec_user_key = org_private_key
    .decrypt(Pkcs1v15Encrypt, &reset_password_details.reset_password_key.ok_or(Error::MissingFields)?.into_bytes())
    .map_err(|_| CryptoError::InvalidKey)?; // need better error

  // re-encrypt user key with device public key
  let device_public_key = rsa::RsaPublicKey::from_pkcs1_der(&input.device_public_key.as_bytes())
        .map_err(|_| CryptoError::InvalidKey)?;
  let mut rng = rand::thread_rng();
  let re_encrypted_user_key = device_public_key.encrypt(&mut rng, Pkcs1v15Encrypt, &dec_user_key)
        .map_err(|_| CryptoError::InvalidKey)?; // need better error

  String::from_utf8(re_encrypted_user_key)
    .map_err(|_| CryptoError::InvalidKey)? // need better error
    .parse()
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
