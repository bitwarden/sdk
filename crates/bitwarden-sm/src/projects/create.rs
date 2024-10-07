use bitwarden_api_api::models::ProjectCreateRequestModel;
use bitwarden_core::{key_management::SymmetricKeyRef, validate_only_whitespaces, Client, Error};
use bitwarden_crypto::Encryptable;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use super::ProjectResponse;

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProjectCreateRequest {
    /// Organization where the project will be created
    pub organization_id: Uuid,
    #[validate(length(min = 1, max = 500), custom(function = validate_only_whitespaces))]
    pub name: String,
}

pub(crate) async fn create_project(
    client: &Client,
    input: &ProjectCreateRequest,
) -> Result<ProjectResponse, Error> {
    input.validate()?;

    let project = {
        // Context is not Send, so we can't use it across an await point
        let mut ctx = client.internal.get_crypto_service().context();
        let key = SymmetricKeyRef::Organization(input.organization_id);

        Some(ProjectCreateRequestModel {
            name: input
                .name
                .clone()
                .trim()
                .encrypt(&mut ctx, key)?
                .to_string(),
        })
    };

    let config = client.internal.get_api_configurations().await;
    let res = bitwarden_api_api::apis::projects_api::organizations_organization_id_projects_post(
        &config.api,
        input.organization_id,
        project,
    )
    .await?;

    let mut ctx = client.internal.get_crypto_service().context();
    ProjectResponse::process_response(res, &mut ctx)
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn create_project(name: String) -> Result<ProjectResponse, Error> {
        let input = ProjectCreateRequest {
            organization_id: Uuid::new_v4(),
            name,
        };

        super::create_project(&Client::new(None), &input).await
    }

    #[tokio::test]
    async fn test_create_project_request_name_empty_string() {
        let response = create_project("".into()).await;
        assert!(response.is_err());
        assert_eq!(
            response.err().unwrap().to_string(),
            "name must not be empty"
        );
    }

    #[tokio::test]
    async fn test_create_project_request_name_all_whitespaces_space() {
        let response = create_project(" ".into()).await;
        assert!(response.is_err());
        assert_eq!(
            response.err().unwrap().to_string(),
            "name must not contain only whitespaces"
        );
    }

    #[tokio::test]
    async fn test_create_project_request_name_all_whitespaces_tab() {
        let response = create_project("\t".into()).await;
        assert!(response.is_err());
        assert_eq!(
            response.err().unwrap().to_string(),
            "name must not contain only whitespaces"
        );
    }

    #[tokio::test]
    async fn test_create_project_request_name_all_whitespaces_newline() {
        let response = create_project("\n".into()).await;
        assert!(response.is_err());
        assert_eq!(
            response.err().unwrap().to_string(),
            "name must not contain only whitespaces"
        );
    }

    #[tokio::test]
    async fn test_create_project_request_name_all_whitespaces_combined() {
        let response = create_project(" \t\n".into()).await;
        assert!(response.is_err());
        assert_eq!(
            response.err().unwrap().to_string(),
            "name must not contain only whitespaces"
        );
    }

    #[tokio::test]
    async fn test_create_project_request_name_501_character_length() {
        let response = create_project("a".repeat(501)).await;
        assert!(response.is_err());
        assert_eq!(
            response.err().unwrap().to_string(),
            "name must not exceed 500 characters in length"
        );
    }
}
