use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// An empty request that needs no parameters
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct EmptyRequest {}
