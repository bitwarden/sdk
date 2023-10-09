use crate::{error::Result, wordlist::EFF_LONG_WORD_LIST};
use rand::{seq::SliceRandom, Rng, RngCore};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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

    let mut rand = rand::thread_rng();

    let mut passphrase_words = gen_words(&mut rand, words);
    if include_number {
        let number_idx = rand.gen_range(0..words as usize);
        passphrase_words[number_idx].push_str(&rand.gen_range(0..=9).to_string());
    }

    if capitalize {
        passphrase_words = passphrase_words
            .iter()
            .map(|w| capitalize_first_letter(w))
            .collect();
    }

    Ok(passphrase_words.join(&separator.to_string()))
}

fn gen_words(mut rng: impl RngCore, num_words: u8) -> Vec<String> {
    (0..num_words)
        .map(|_| {
            EFF_LONG_WORD_LIST
                .choose(&mut rng)
                .expect("slice is not empty")
                .to_string()
        })
        .collect()
}

fn capitalize_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_capitalize() {
        assert_eq!(super::capitalize_first_letter("hello"), "Hello");
        assert_eq!(super::capitalize_first_letter("1ello"), "1ello");
        assert_eq!(super::capitalize_first_letter("Hello"), "Hello");
        assert_eq!(super::capitalize_first_letter("h"), "H");
        assert_eq!(super::capitalize_first_letter(""), "");

        // Also supports non-ascii, though the EFF list doesn't have any
        assert_eq!(super::capitalize_first_letter("áéíóú"), "Áéíóú");
    }
}
