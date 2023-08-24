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

/// Passphrase generator request.
///
/// The default separator is `-` and default number of words is 3.
#[derive(Serialize, Deserialize, Debug, JsonSchema, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct PassphraseGeneratorRequest {
    pub num_words: Option<u8>,
    pub word_separator: Option<String>,
    pub capitalize: Option<bool>,
    pub include_number: Option<bool>,
}
