use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::vault::{Collection, CollectionView};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CollectionDecryptListRequest {
    pub collections: Vec<Collection>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CollectionDecryptListResponse {
    pub collections: Vec<CollectionView>,
}
