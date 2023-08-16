use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::vault::{Collection, CollectionView};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct CollectionDecryptListRequest {
    pub collections: Vec<Collection>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct CollectionDecryptListResponse {
    pub collections: Vec<CollectionView>,
}
