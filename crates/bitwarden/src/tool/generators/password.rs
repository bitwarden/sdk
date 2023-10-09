use crate::error::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Password generator request. If all options are false, the default is to
/// generate a password with:
/// - lowercase
/// - uppercase
/// - numbers
///
/// The default length is 16.
#[derive(Serialize, Deserialize, Debug, JsonSchema, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct PasswordGeneratorRequest {
    pub lowercase: bool,
    pub uppercase: bool,
    pub numbers: bool,
    pub special: bool,

    pub length: Option<u8>,

    pub avoid_ambiguous: Option<bool>, // TODO: Should we rename this to include_all_characters?
    pub min_lowercase: Option<bool>,
    pub min_uppercase: Option<bool>,
    pub min_number: Option<bool>,
    pub min_special: Option<bool>,
}

pub(super) fn password(_input: PasswordGeneratorRequest) -> Result<String> {
    Ok("pa11w0rd".to_string())
}
