use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::crypto::CipherString;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Attachment {
    pub id: Option<String>,
    pub url: Option<String>,
    pub size: Option<String>,
    pub size_name: Option<String>,
    pub file_name: Option<CipherString>,
    pub key: Option<CipherString>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AttachmentView {
    pub id: Option<String>,
    pub url: Option<String>,
    pub size: Option<String>,
    pub size_name: Option<String>,
    pub file_name: Option<String>,
    pub key: Option<String>,
}
