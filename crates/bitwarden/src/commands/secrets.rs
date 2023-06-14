use bitwarden_api_api::models::{SecretCreateRequestModel, SecretUpdateRequestModel};

use crate::{
    client::Client,
    error::Result,
    sdk::{
        request::secrets_request::{
            SecretCreateRequest, SecretGetRequest, SecretIdentifiersByProjectRequest,
            SecretIdentifiersRequest, SecretPutRequest, SecretsDeleteRequest,
        },
        response::secrets_response::{
            SecretIdentifiersResponse, SecretResponse, SecretsDeleteResponse,
        },
    },
};

pub(crate) async fn get_secret(
    client: &mut Client,
    input: &SecretGetRequest,
) -> Result<SecretResponse> {
    let config = client.get_api_configurations().await;
    let res = bitwarden_api_api::apis::secrets_api::secrets_id_get(&config.api, input.id).await?;

    let enc = client.get_encryption_settings()?;

    SecretResponse::process_response(res, &enc)
}

pub(crate) async fn create_secret(
    client: &mut Client,
    input: &SecretCreateRequest,
) -> Result<SecretResponse> {
    let enc = client.get_encryption_settings()?;

    let org_id = Some(input.organization_id);

    let secret = Some(SecretCreateRequestModel {
        key: enc.encrypt(input.key.as_bytes(), org_id)?.to_string(),
        value: enc.encrypt(input.value.as_bytes(), org_id)?.to_string(),
        note: enc.encrypt(input.note.as_bytes(), org_id)?.to_string(),
        project_ids: None,
    });

    let config = client.get_api_configurations().await;
    let res = bitwarden_api_api::apis::secrets_api::organizations_organization_id_secrets_post(
        &config.api,
        input.organization_id,
        secret,
    )
    .await?;

    SecretResponse::process_response(res, &enc)
}

pub(crate) async fn list_secrets(
    client: &mut Client,
    input: &SecretIdentifiersRequest,
) -> Result<SecretIdentifiersResponse> {
    let config = client.get_api_configurations().await;
    let res = bitwarden_api_api::apis::secrets_api::organizations_organization_id_secrets_get(
        &config.api,
        input.organization_id,
    )
    .await?;

    let enc = client.get_encryption_settings()?;

    SecretIdentifiersResponse::process_response(res, &enc)
}

pub(crate) async fn list_secrets_by_project(
    client: &mut Client,
    input: &SecretIdentifiersByProjectRequest,
) -> Result<SecretIdentifiersResponse> {
    let config = client.get_api_configurations().await;
    let res = bitwarden_api_api::apis::secrets_api::projects_project_id_secrets_get(
        &config.api,
        input.project_id,
    )
    .await?;

    let enc = client.get_encryption_settings()?;

    SecretIdentifiersResponse::process_response(res, &enc)
}

pub(crate) async fn update_secret(
    client: &mut Client,
    input: &SecretPutRequest,
) -> Result<SecretResponse> {
    let enc = client.get_encryption_settings()?;

    let org_id = Some(input.organization_id);

    let secret = Some(SecretUpdateRequestModel {
        key: enc.encrypt(input.key.as_bytes(), org_id)?.to_string(),
        value: enc.encrypt(input.value.as_bytes(), org_id)?.to_string(),
        note: enc.encrypt(input.note.as_bytes(), org_id)?.to_string(),
        project_ids: None,
    });

    let config = client.get_api_configurations().await;
    let res =
        bitwarden_api_api::apis::secrets_api::secrets_id_put(&config.api, input.id, secret).await?;

    let enc = client.get_encryption_settings()?;

    SecretResponse::process_response(res, &enc)
}

pub(crate) async fn delete_secrets(
    client: &mut Client,
    input: SecretsDeleteRequest,
) -> Result<SecretsDeleteResponse> {
    let config = client.get_api_configurations().await;
    let res =
        bitwarden_api_api::apis::secrets_api::secrets_delete_post(&config.api, Some(input.ids))
            .await?;

    SecretsDeleteResponse::process_response(res)
}
