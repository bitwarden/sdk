use bitwarden_api_api::models::SecretUpdateRequestModel;
use bitwarden_crypto::KeyEncryptable;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use super::SecretResponse;
use crate::{
    client::Client,
    error::{validate, validate_only_whitespaces, Error, Result},
};

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecretPutRequest {
    /// ID of the secret to modify
    pub id: Uuid,
    /// Organization ID of the secret to modify
    pub organization_id: Uuid,
    #[validate(length(min = 1, max = 500), custom(function = validate_only_whitespaces))]
    pub key: String,
    #[validate(length(min = 1, max = 25_000))]
    pub value: String,
    #[validate(length(max = 7_000), custom(function = validate_only_whitespaces))]
    pub note: String,
    pub project_ids: Option<Vec<Uuid>>,
}

pub(crate) async fn update_secret(
    client: &mut Client,
    input: &SecretPutRequest,
) -> Result<SecretResponse> {
    validate!(input);

    let key = client
        .get_encryption_settings()?
        .get_key(&Some(input.organization_id))
        .ok_or(Error::VaultLocked)?;

    let secret = Some(SecretUpdateRequestModel {
        key: input
            .key
            .trim()
            .to_string()
            .clone()
            .encrypt_with_key(key)?
            .to_string(),
        value: input.value.clone().encrypt_with_key(key)?.to_string(),
        note: input
            .note
            .trim()
            .to_string()
            .clone()
            .encrypt_with_key(key)?
            .to_string(),
        project_ids: input.project_ids.clone(),
    });

    let config = client.get_api_configurations().await;
    let res =
        bitwarden_api_api::apis::secrets_api::secrets_id_put(&config.api, input.id, secret).await?;

    let enc = client.get_encryption_settings()?;

    SecretResponse::process_response(res, enc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[warn(dead_code)]
    async fn update_secret(
        key: Option<String>,
        value: Option<String>,
        note: Option<String>,
    ) -> Result<SecretResponse> {
        let input = SecretPutRequest {
            id: Uuid::new_v4(),
            organization_id: Uuid::new_v4(),
            key: key.unwrap_or_else(|| "test key".into()),
            value: value.unwrap_or_else(|| "test value".into()),
            note: note.unwrap_or_else(|| "test note".into()),
            project_ids: Some(vec![Uuid::new_v4()]),
        };

        super::update_secret(&mut Client::new(None), &input).await
    }

    #[tokio::test]
    async fn test_update_secret_request_key_empty_string() {
        let response = update_secret(Some("".into()), None, None).await;
        assert!(response.is_err());
        assert_eq!(response.err().unwrap().to_string(), "key must not be empty");
    }

    #[tokio::test]
    async fn test_update_secret_request_key_all_whitespaces_space() {
        let response = update_secret(Some(" ".into()), None, None).await;
        assert!(response.is_err());
        assert_eq!(
            response.err().unwrap().to_string(),
            "key must not contain only whitespaces"
        );
    }

    #[tokio::test]
    async fn test_update_secret_request_key_all_whitespaces_tab() {
        let response = update_secret(Some("\t".into()), None, None).await;
        assert!(response.is_err());
        assert_eq!(
            response.err().unwrap().to_string(),
            "key must not contain only whitespaces"
        );
    }

    #[tokio::test]
    async fn test_update_secret_request_key_all_whitespaces_newline() {
        let response = update_secret(Some("\n".into()), None, None).await;
        assert!(response.is_err());
        assert_eq!(
            response.err().unwrap().to_string(),
            "key must not contain only whitespaces"
        );
    }

    #[tokio::test]
    async fn test_update_secret_request_key_all_whitespaces_combined() {
        let response = update_secret(Some(" \t\n".into()), None, None).await;
        assert!(response.is_err());
        assert_eq!(
            response.err().unwrap().to_string(),
            "key must not contain only whitespaces"
        );
    }

    #[tokio::test]
    async fn test_update_secret_request_key_501_character_length() {
        let response = update_secret(Some("a".repeat(501).into()), None, None).await;
        assert!(response.is_err());
        assert_eq!(
            response.err().unwrap().to_string(),
            "key must not exceed 500 characters in length"
        );
    }

    #[tokio::test]
    async fn test_update_secret_request_value_empty_string() {
        let response = update_secret(None, Some("".into()), None).await;
        assert!(response.is_err());
        assert_eq!(
            response.err().unwrap().to_string(),
            "value must not be empty"
        );
    }

    #[tokio::test]
    async fn test_update_secret_request_value_25001_character_length() {
        let response = update_secret(None, Some("a".repeat(25001).into()), None).await;
        assert!(response.is_err());
        assert_eq!(
            response.err().unwrap().to_string(),
            "value must not exceed 25000 characters in length"
        );
    }

    #[tokio::test]
    async fn test_update_secret_request_note_all_whitespaces_space() {
        let response = update_secret(None, None, Some(" ".into())).await;
        assert!(response.is_err());
        assert_eq!(
            response.err().unwrap().to_string(),
            "note must not contain only whitespaces"
        );
    }

    #[tokio::test]
    async fn test_update_secret_request_note_all_whitespaces_tab() {
        let response = update_secret(None, None, Some("\t".into())).await;
        assert!(response.is_err());
        assert_eq!(
            response.err().unwrap().to_string(),
            "note must not contain only whitespaces"
        );
    }

    #[tokio::test]
    async fn test_update_secret_request_note_all_whitespaces_newline() {
        let response = update_secret(None, None, Some("\n".into())).await;
        assert!(response.is_err());
        assert_eq!(
            response.err().unwrap().to_string(),
            "note must not contain only whitespaces"
        );
    }

    #[tokio::test]
    async fn test_update_secret_request_note_all_whitespaces_combined() {
        let response = update_secret(None, None, Some(" \t\n".into())).await;
        assert!(response.is_err());
        assert_eq!(
            response.err().unwrap().to_string(),
            "note must not contain only whitespaces"
        );
    }

    #[tokio::test]
    async fn test_update_secret_request_note_7001_character_length() {
        let response = update_secret(None, None, Some("a".repeat(7001).into())).await;
        assert!(response.is_err());
        assert_eq!(
            response.err().unwrap().to_string(),
            "note must not exceed 7000 characters in length"
        );
    }
}
