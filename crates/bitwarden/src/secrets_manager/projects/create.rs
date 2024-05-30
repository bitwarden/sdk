use bitwarden_api_api::models::ProjectCreateRequestModel;
use bitwarden_crypto::KeyEncryptable;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::ProjectResponse;
use crate::{
    client::Client,
    error::{validate, Error, Result, validate_only_whitespaces},
};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProjectCreateRequest {
    /// Organization where the project will be created
    pub organization_id: Uuid,
    #[validate(length(min = 1, max = 500), custom(function = validate_only_whitespaces))]
    pub name: String,
}

pub(crate) async fn create_project(
    client: &mut Client,
    input: &ProjectCreateRequest,
) -> Result<ProjectResponse> {
    validate!(input);

    let key = client
        .get_encryption_settings()?
        .get_key(&Some(input.organization_id))
        .ok_or(Error::VaultLocked)?;

    let project = Some(ProjectCreateRequestModel {
        name: input.name.trim().to_string().clone().encrypt_with_key(key)?.to_string(),
    });

    let config = client.get_api_configurations().await;
    let res = bitwarden_api_api::apis::projects_api::organizations_organization_id_projects_post(
        &config.api,
        input.organization_id,
        project,
    )
    .await?;

    let enc = client.get_encryption_settings()?;

    ProjectResponse::process_response(res, enc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[warn(dead_code)]
    async fn create_project_with_name(name: String) -> Result<ProjectResponse> {
        let input = ProjectCreateRequest {
            organization_id: Uuid::new_v4(),
            name,
        };

        create_project(&mut Client::new(None), &input).await
    }

    #[tokio::test]
    async fn test_create_project_request_name_empty_string() {
        let response = create_project_with_name("".to_string()).await;
        assert!(response.is_err());
        assert_eq!(response.err().unwrap().to_string(), "name must not be empty");
    }

    #[tokio::test]
    async fn test_create_project_request_name_all_whitespaces_space() {
        let response = create_project_with_name(" ".to_string()).await;
        assert!(response.is_err());
        assert_eq!(response.err().unwrap().to_string(), "name must not contain only whitespaces");
    }

    #[tokio::test]
    async fn test_create_project_request_name_all_whitespaces_tab() {
        let response = create_project_with_name("\t".to_string()).await;
        assert!(response.is_err());
        assert_eq!(response.err().unwrap().to_string(), "name must not contain only whitespaces");
    }

    #[tokio::test]
    async fn test_create_project_request_name_all_whitespaces_newline() {
        let response = create_project_with_name("\n".to_string()).await;
        assert!(response.is_err());
        assert_eq!(response.err().unwrap().to_string(), "name must not contain only whitespaces");
    }

    #[tokio::test]
    async fn test_create_project_request_name_all_whitespaces_all() {
        let response = create_project_with_name(" \t\n".to_string()).await;
        assert!(response.is_err());
        assert_eq!(response.err().unwrap().to_string(), "name must not contain only whitespaces");
    }

    #[tokio::test]
    async fn test_create_project_request_name_501_character_length() {
        let response = create_project_with_name("a".repeat(501)).await;
        assert!(response.is_err());
        assert_eq!(response.err().unwrap().to_string(),
                   "name must not exceed 500 characters in length");
    }
}
