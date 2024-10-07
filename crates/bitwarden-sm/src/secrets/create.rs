use bitwarden_api_api::models::SecretCreateRequestModel;
use bitwarden_core::{key_management::SymmetricKeyRef, validate_only_whitespaces, Client, Error};
use bitwarden_crypto::Encryptable;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use super::SecretResponse;

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecretCreateRequest {
    /// Organization where the secret will be created
    pub organization_id: Uuid,

    #[validate(length(min = 1, max = 500), custom(function = validate_only_whitespaces))]
    pub key: String,
    #[validate(length(min = 1, max = 25_000))]
    pub value: String,
    #[validate(length(max = 7_000), custom(function = validate_only_whitespaces))]
    pub note: String,

    /// IDs of the projects that this secret will belong to
    pub project_ids: Option<Vec<Uuid>>,
}

pub(crate) async fn create_secret(
    client: &Client,
    input: &SecretCreateRequest,
) -> Result<SecretResponse, Error> {
    input.validate()?;

    let secret = {
        // Context is not Send, so we can't use it across an await point
        let mut ctx = client.internal.get_crypto_service().context();
        let key = SymmetricKeyRef::Organization(input.organization_id);

        Some(SecretCreateRequestModel {
            key: input.key.clone().trim().encrypt(&mut ctx, key)?.to_string(),
            value: input.value.clone().encrypt(&mut ctx, key)?.to_string(),
            note: input
                .note
                .clone()
                .trim()
                .encrypt(&mut ctx, key)?
                .to_string(),
            project_ids: input.project_ids.clone(),
            access_policies_requests: None,
        })
    };

    let config = client.internal.get_api_configurations().await;
    let res = bitwarden_api_api::apis::secrets_api::organizations_organization_id_secrets_post(
        &config.api,
        input.organization_id,
        secret,
    )
    .await?;

    let mut ctx = client.internal.get_crypto_service().context();
    SecretResponse::process_response(res, &mut ctx)
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn create_secret(
        key: Option<String>,
        value: Option<String>,
        note: Option<String>,
    ) -> Result<SecretResponse, Error> {
        let input = SecretCreateRequest {
            organization_id: Uuid::new_v4(),
            key: key.unwrap_or_else(|| "test key".into()),
            value: value.unwrap_or_else(|| "test value".into()),
            note: note.unwrap_or_else(|| "test note".into()),
            project_ids: Some(vec![Uuid::new_v4()]),
        };

        super::create_secret(&Client::new(None), &input).await
    }

    #[tokio::test]
    async fn test_create_secret_request_key_empty_string() {
        let response = create_secret(Some("".into()), None, None).await;
        assert!(response.is_err());
        assert_eq!(response.err().unwrap().to_string(), "key must not be empty");
    }

    #[tokio::test]
    async fn test_create_secret_request_key_all_whitespaces_space() {
        let response = create_secret(Some(" ".into()), None, None).await;
        assert!(response.is_err());
        assert_eq!(
            response.err().unwrap().to_string(),
            "key must not contain only whitespaces"
        );
    }

    #[tokio::test]
    async fn test_create_secret_request_key_all_whitespaces_tab() {
        let response = create_secret(Some("\t".into()), None, None).await;
        assert!(response.is_err());
        assert_eq!(
            response.err().unwrap().to_string(),
            "key must not contain only whitespaces"
        );
    }

    #[tokio::test]
    async fn test_create_secret_request_key_all_whitespaces_newline() {
        let response = create_secret(Some("\n".into()), None, None).await;
        assert!(response.is_err());
        assert_eq!(
            response.err().unwrap().to_string(),
            "key must not contain only whitespaces"
        );
    }

    #[tokio::test]
    async fn test_create_secret_request_key_all_whitespaces_combined() {
        let response = create_secret(Some(" \t\n".into()), None, None).await;
        assert!(response.is_err());
        assert_eq!(
            response.err().unwrap().to_string(),
            "key must not contain only whitespaces"
        );
    }

    #[tokio::test]
    async fn test_create_secret_request_key_501_character_length() {
        let response = create_secret(Some("a".repeat(501)), None, None).await;
        assert!(response.is_err());
        assert_eq!(
            response.err().unwrap().to_string(),
            "key must not exceed 500 characters in length"
        );
    }

    #[tokio::test]
    async fn test_create_secret_request_value_empty_string() {
        let response = create_secret(None, Some("".into()), None).await;
        assert!(response.is_err());
        assert_eq!(
            response.err().unwrap().to_string(),
            "value must not be empty"
        );
    }

    #[tokio::test]
    async fn test_create_secret_request_value_25001_character_length() {
        let response = create_secret(None, Some("a".repeat(25001)), None).await;
        assert!(response.is_err());
        assert_eq!(
            response.err().unwrap().to_string(),
            "value must not exceed 25000 characters in length"
        );
    }

    #[tokio::test]
    async fn test_create_secret_request_note_all_whitespaces_space() {
        let response = create_secret(None, None, Some(" ".into())).await;
        assert!(response.is_err());
        assert_eq!(
            response.err().unwrap().to_string(),
            "note must not contain only whitespaces"
        );
    }

    #[tokio::test]
    async fn test_create_secret_request_note_all_whitespaces_tab() {
        let response = create_secret(None, None, Some("\t".into())).await;
        assert!(response.is_err());
        assert_eq!(
            response.err().unwrap().to_string(),
            "note must not contain only whitespaces"
        );
    }

    #[tokio::test]
    async fn test_create_secret_request_note_all_whitespaces_newline() {
        let response = create_secret(None, None, Some("\n".into())).await;
        assert!(response.is_err());
        assert_eq!(
            response.err().unwrap().to_string(),
            "note must not contain only whitespaces"
        );
    }

    #[tokio::test]
    async fn test_create_secret_request_note_all_whitespaces_combined() {
        let response = create_secret(None, None, Some(" \t\n".into())).await;
        assert!(response.is_err());
        assert_eq!(
            response.err().unwrap().to_string(),
            "note must not contain only whitespaces"
        );
    }

    #[tokio::test]
    async fn test_create_secret_request_note_7001_character_length() {
        let response = create_secret(None, None, Some("a".repeat(7001))).await;
        assert!(response.is_err());
        assert_eq!(
            response.err().unwrap().to_string(),
            "note must not exceed 7000 characters in length"
        );
    }
}
