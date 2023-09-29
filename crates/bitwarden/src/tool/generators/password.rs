use crate::{error::Result, wordlist::EFF_LONG_WORD_LIST};
use rand::{seq::SliceRandom, Rng};
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

pub(super) fn password(_input: PasswordGeneratorRequest) -> Result<String> {
    Ok("pa11w0rd".to_string())
}

const DEFAULT_PASSPHRASE_NUM_WORDS: u8 = 3;
const DEFAULT_PASSPHRASE_SEPARATOR: char = ' ';

pub(super) fn passphrase(input: PassphraseGeneratorRequest) -> Result<String> {
    let words = input.num_words.unwrap_or(DEFAULT_PASSPHRASE_NUM_WORDS);
    let separator = input
        .word_separator
        .and_then(|s| s.chars().next())
        .unwrap_or(DEFAULT_PASSPHRASE_SEPARATOR);

    let capitalize = input.capitalize.unwrap_or(false);
    let include_number = input.include_number.unwrap_or(false);

    fn capitalize_first_letter(s: &str) -> String {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }

    let mut rand = rand::thread_rng();

    let insert_number_idx = include_number.then(|| rand.gen_range(0..words));
    let mut passphrase = String::new();

    for idx in 0..words {
        let word = EFF_LONG_WORD_LIST
            .choose(&mut rand)
            .expect("slice is not empty");

        if capitalize {
            passphrase.push_str(&capitalize_first_letter(word));
        } else {
            passphrase.push_str(word);
        }

        if insert_number_idx == Some(idx) {
            passphrase.push_str(&rand.gen_range(0..=9).to_string());
        }

        if idx != words - 1 {
            passphrase.push(separator)
        }
    }

    Ok(passphrase)
}
