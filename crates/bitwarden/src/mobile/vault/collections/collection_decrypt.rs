use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::vault::{Collection, CollectionView};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CollectionDecryptRequest {
    pub collection: Collection,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CollectionDecryptResponse {
    pub collection: CollectionView,
}
