use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProjectGetRequest {
    /// ID of the project to retrieve
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProjectCreateRequest {
    /// Organization where the project will be created
    pub organization_id: String,

    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProjectPutRequest {
    /// ID of the project to modify
    pub id: String,
    /// Organization ID of the project to modify
    pub organization_id: String,

    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProjectsListRequest {
    /// Organization to retrieve all the projects from
    pub organization_id: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProjectsDeleteRequest {
    /// IDs of the projects to delete
    pub ids: Vec<String>,
}
