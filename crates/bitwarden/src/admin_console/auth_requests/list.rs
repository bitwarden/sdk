use bitwarden_api_api::models::{
    PendingOrganizationAuthRequestResponseModel,
    PendingOrganizationAuthRequestResponseModelListResponseModel,
};
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    client::Client,
    error::{Error, Result},
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PendingAuthRequestsRequest {
    /// Organization to retrieve pending auth requests for
    pub organization_id: Uuid,
}

pub(crate) async fn list_pending_requests(
    client: &mut Client,
    input: &PendingAuthRequestsRequest,
) -> Result<PendingAuthRequestsResponse> {
    let config = client.get_api_configurations().await;
    let res = bitwarden_api_api::apis::organization_auth_requests_api::organizations_org_id_auth_requests_get(
        &config.api,
        input.organization_id,
    )
    .await?;

    PendingAuthRequestsResponse::process_response(res)
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PendingAuthRequestsResponse {
    pub data: Vec<PendingAuthRequestResponse>,
}

impl PendingAuthRequestsResponse {
    pub(crate) fn process_response(
        response: PendingOrganizationAuthRequestResponseModelListResponseModel,
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
    pub public_key_b64: String,
    pub request_device_identifier: String,
    pub request_device_type: String,
    pub request_ip_address: String,
    pub creation_date: DateTime<Utc>,
}

impl PendingAuthRequestResponse {
    pub(crate) fn process_response(
        response: PendingOrganizationAuthRequestResponseModel,
    ) -> Result<Self> {
        Ok(PendingAuthRequestResponse {
            id: response.id.ok_or(Error::MissingFields)?,
            user_id: response.user_id.ok_or(Error::MissingFields)?,
            organization_user_id: response.organization_user_id.ok_or(Error::MissingFields)?,
            email: response.email.ok_or(Error::MissingFields)?,
            public_key_b64: response.public_key.ok_or(Error::MissingFields)?,
            request_device_identifier: response
                .request_device_identifier
                .ok_or(Error::MissingFields)?,
            request_device_type: response.request_device_type.ok_or(Error::MissingFields)?,
            request_ip_address: response.request_ip_address.ok_or(Error::MissingFields)?,
            creation_date: response
                .creation_date
                .ok_or(Error::MissingFields)?
                .parse()?,
        })
    }
}
