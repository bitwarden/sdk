use bitwarden_api_api::models::ProjectUpdateRequestModel;
use bitwarden_core::{key_management::SymmetricKeyRef, validate_only_whitespaces, Client, Error};
use bitwarden_crypto::Encryptable;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use super::ProjectResponse;

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProjectPutRequest {
    /// ID of the project to modify
    pub id: Uuid,
    /// Organization ID of the project to modify
    pub organization_id: Uuid,
    #[validate(length(min = 1, max = 500), custom(function = validate_only_whitespaces))]
    pub name: String,
}

pub(crate) async fn update_project(
    client: &Client,
    input: &ProjectPutRequest,
) -> Result<ProjectResponse, Error> {
    input.validate()?;

    let project = {
        // Context is not Send, so we can't use it across an await point
        let mut ctx = client.internal.get_crypto_service().context();
        let key = SymmetricKeyRef::Organization(input.organization_id);

        Some(ProjectUpdateRequestModel {
            name: input
                .name
                .clone()
                .trim()
                .encrypt(&mut ctx, key)?
                .to_string(),
        })
    };

    let config = client.internal.get_api_configurations().await;
    let res =
        bitwarden_api_api::apis::projects_api::projects_id_put(&config.api, input.id, project)
            .await?;

    let mut ctx = client.internal.get_crypto_service().context();
    ProjectResponse::process_response(res, &mut ctx)
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn update_project(name: String) -> Result<ProjectResponse, Error> {
        let input = ProjectPutRequest {
            id: Uuid::new_v4(),
            organization_id: Uuid::new_v4(),
            name,
        };

        super::update_project(&Client::new(None), &input).await
    }

    #[tokio::test]
    async fn test_update_project_request_name_empty_string() {
        let response = update_project("".into()).await;
        assert!(response.is_err());
        assert_eq!(
            response.err().unwrap().to_string(),
            "name must not be empty"
        );
    }

    #[tokio::test]
    async fn test_update_project_request_name_all_whitespaces_space() {
        let response = update_project(" ".into()).await;
        assert!(response.is_err());
        assert_eq!(
            response.err().unwrap().to_string(),
            "name must not contain only whitespaces"
        );
    }

    #[tokio::test]
    async fn test_update_project_request_name_all_whitespaces_tab() {
        let response = update_project("\t".into()).await;
        assert!(response.is_err());
        assert_eq!(
            response.err().unwrap().to_string(),
            "name must not contain only whitespaces"
        );
    }

    #[tokio::test]
    async fn test_update_project_request_name_all_whitespaces_newline() {
        let response = update_project("\n".into()).await;
        assert!(response.is_err());
        assert_eq!(
            response.err().unwrap().to_string(),
            "name must not contain only whitespaces"
        );
    }

    #[tokio::test]
    async fn test_update_project_request_name_all_whitespaces_combined() {
        let response = update_project(" \t\n".into()).await;
        assert!(response.is_err());
        assert_eq!(
            response.err().unwrap().to_string(),
            "name must not contain only whitespaces"
        );
    }

    #[tokio::test]
    async fn test_update_project_request_name_501_character_length() {
        let response = update_project("a".repeat(501)).await;
        assert!(response.is_err());
        assert_eq!(
            response.err().unwrap().to_string(),
            "name must not exceed 500 characters in length"
        );
    }
}
