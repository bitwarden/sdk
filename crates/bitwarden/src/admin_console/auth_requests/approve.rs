use bitwarden_api_api::models::{
    AdminAuthRequestUpdateRequestModel, OrganizationUserResetPasswordDetailsResponseModel,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    client::Client,
    crypto::{encrypt_rsa, private_key_from_bytes, public_key_from_b64, Decryptable, EncString},
    error::{Error, Result},
};

use super::{list_pending_requests, PendingAuthRequestResponse};

// TODO: what identifier should this take? e.g. org_user_id, request_id, etc
// using org_user_id for now
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AuthApproveRequest {
    pub organization_user_id: Uuid,
    pub organization_id: Uuid,
}

pub(crate) async fn approve_auth_request(
    client: &mut Client,
    input: &AuthApproveRequest,
) -> Result<()> {
    let device_request = get_pending_request(input.organization_id, input.organization_user_id, client).await;

    // Get user reset password details
    let reset_password_details =
      bitwarden_api_api::apis::organization_users_api::organizations_org_id_users_id_reset_password_details_get(
&client.get_api_configurations().await.api,
        &input.organization_id.to_string(),
        &device_request.id.to_string(),
    )
    .await?;

    let encrypted_user_key = get_encrypted_user_key(
        &client,
        input.organization_id,
        &device_request,
        reset_password_details,
    )?;

    bitwarden_api_api::apis::organization_auth_requests_api::organizations_org_id_auth_requests_request_id_post(
&client.get_api_configurations().await.api,
        input.organization_id,
        device_request.id,
        Some(AdminAuthRequestUpdateRequestModel {
          encrypted_user_key: Some(encrypted_user_key.to_string()),
          request_approved: true
        })
    )
    .await?;

    Ok(())
}

async fn get_pending_request(organization_id: Uuid, organization_user_id: Uuid, client: &mut Client) -> PendingAuthRequestResponse {
    // hack: get all approval details and then find the one we want
    // when we settle on an identifier then we should just give ourselves a better server API
    // or do we require the caller to pass all this info in?
    let all_device_requests = list_pending_requests(
        client,
        &super::PendingAuthRequestsRequest {
            organization_id: organization_id,
        },
    )
    .await;

    all_device_requests
        .unwrap()
        .data
        .into_iter()
        .find(|r| r.organization_user_id == organization_user_id)
        .unwrap() // TODO: error handling
}

fn get_encrypted_user_key(
    client: &Client,
    organization_id: Uuid,
    input: &PendingAuthRequestResponse,
    reset_password_details: OrganizationUserResetPasswordDetailsResponseModel,
) -> Result<EncString> {
    // Decrypt organization's encrypted private key with org key
    let enc = client.get_encryption_settings()?;

    let org_private_key = {
        let dec = reset_password_details
            .encrypted_private_key
            .ok_or(Error::MissingFields)?
            .parse::<EncString>()?
            .decrypt(enc, &Some(organization_id))?
            .into_bytes();

        private_key_from_bytes(&dec)?
    };

    // Decrypt user key with org private key
    let user_key = &reset_password_details
        .reset_password_key
        .ok_or(Error::MissingFields)?
        .parse::<EncString>()?;
    let dec_user_key = user_key.decrypt_with_rsa_key(&org_private_key)?;

    // re-encrypt user key with device public key
    let device_public_key = public_key_from_b64(&input.public_key_b64)?;
    let re_encrypted_user_key = encrypt_rsa(dec_user_key, &device_public_key)?;

    EncString::from_buffer(&re_encrypted_user_key)
}
