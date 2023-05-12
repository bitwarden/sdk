use bitwarden_api_api::models::{SecretCreateRequestModel, SecretUpdateRequestModel};

use crate::{
    client::Client,
    crypto::Encryptable,
    error::{Error, Result},
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

    let enc = client
        .get_encryption_settings()
        .as_ref()
        .ok_or(Error::VaultLocked)?;

    SecretResponse::process_response(res, enc)
}

pub(crate) async fn create_secret(
    client: &mut Client,
    input: SecretCreateRequest,
) -> Result<SecretResponse> {
    let enc = client
        .get_encryption_settings()
        .as_ref()
        .ok_or(Error::VaultLocked)?;

    let org_id = Some(input.organization_id);

    let secret = Some(SecretCreateRequestModel {
        key: input.key.encrypt(enc, &org_id)?.to_string(),
        value: input.value.encrypt(enc, &org_id)?.to_string(),
        note: input.note.encrypt(enc, &org_id)?.to_string(),
        project_ids: None,
    });

    let config = client.get_api_configurations().await;
    let res = bitwarden_api_api::apis::secrets_api::organizations_organization_id_secrets_post(
        &config.api,
        input.organization_id,
        secret,
    )
    .await?;

    let enc = client
        .get_encryption_settings()
        .as_ref()
        .ok_or(Error::VaultLocked)?;

    SecretResponse::process_response(res, enc)
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

    let enc = client
        .get_encryption_settings()
        .as_ref()
        .ok_or(Error::VaultLocked)?;

    SecretIdentifiersResponse::process_response(res, enc)
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

    let enc = client
        .get_encryption_settings()
        .as_ref()
        .ok_or(Error::VaultLocked)?;

    SecretIdentifiersResponse::process_response(res, enc)
}

pub(crate) async fn update_secret(
    client: &mut Client,
    input: SecretPutRequest,
) -> Result<SecretResponse> {
    let enc = client
        .get_encryption_settings()
        .as_ref()
        .ok_or(Error::VaultLocked)?;

    let org_id = Some(input.organization_id);

    let secret = Some(SecretUpdateRequestModel {
        key: input.key.encrypt(enc, &org_id)?.to_string(),
        value: input.value.encrypt(enc, &org_id)?.to_string(),
        note: input.note.encrypt(enc, &org_id)?.to_string(),
        project_ids: None,
    });

    let config = client.get_api_configurations().await;
    let res =
        bitwarden_api_api::apis::secrets_api::secrets_id_put(&config.api, input.id, secret).await?;

    let enc = client
        .get_encryption_settings()
        .as_ref()
        .ok_or(Error::VaultLocked)?;

    SecretResponse::process_response(res, enc)
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
