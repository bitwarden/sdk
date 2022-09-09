use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecretGetRequest {
    /// ID of the secret to retrieve
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecretCreateRequest {
    /// Organization where the secret will be created
    pub organization_id: String,

    pub key: String,
    pub value: String,
    pub note: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecretPutRequest {
    /// ID of the secret to modify
    pub id: String,
    /// Organization ID of the secret to modify
    pub organization_id: String,

    pub key: String,
    pub value: String,
    pub note: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecretIdentifiersRequest {
    /// Organization to retrieve all the secrets from
    pub organization_id: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecretIdentifiersByProjectRequest {
    /// Project to retrieve all the secrets from
    pub project_id: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecretsDeleteRequest {
    /// IDs of the secrets to delete
    pub ids: Vec<String>,
}
