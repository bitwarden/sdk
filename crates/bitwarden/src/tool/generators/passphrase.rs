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
    let num_words = input.num_words.unwrap_or(DEFAULT_PASSPHRASE_NUM_WORDS);
    let separator = input
        .word_separator
        .and_then(|s| s.chars().next())
        .unwrap_or(DEFAULT_PASSPHRASE_SEPARATOR);
    let capitalize = input.capitalize.unwrap_or(false);
    let include_number = input.include_number.unwrap_or(false);

    let mut rand = rand::thread_rng();

    let mut passphrase_words = gen_words(&mut rand, num_words);
    if include_number {
        include_number_in_words(&mut rand, &mut passphrase_words);
    }
    if capitalize {
        capitalize_words(&mut passphrase_words);
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

fn include_number_in_words(mut rng: impl RngCore, words: &mut [String]) {
    let number_idx = rng.gen_range(0..words.len());
    words[number_idx].push_str(&rng.gen_range(0..=9).to_string());
}

fn capitalize_words(words: &mut [String]) {
    words
        .iter_mut()
        .for_each(|w| *w = capitalize_first_letter(w));
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
    use super::*;

    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize_first_letter("hello"), "Hello");
        assert_eq!(capitalize_first_letter("1ello"), "1ello");
        assert_eq!(capitalize_first_letter("Hello"), "Hello");
        assert_eq!(capitalize_first_letter("h"), "H");
        assert_eq!(capitalize_first_letter(""), "");

        // Also supports non-ascii, though the EFF list doesn't have any
        assert_eq!(capitalize_first_letter("áéíóú"), "Áéíóú");
    }

    #[test]
    fn test_capitalize_words() {
        let mut words = vec!["hello".to_string(), "world".to_string()];
        capitalize_words(&mut words);
        assert_eq!(words, &["Hello", "World"]);
    }

    #[test]
    fn test_include_number() {
        let mut rng = rand::thread_rng();

        fn count_numbers(words: &[String]) -> usize {
            words
                .iter()
                .map(|w| w.chars().filter(|c| c.is_numeric()).count())
                .sum()
        }

        let mut words = vec!["hello".to_string(), "world".to_string()];
        assert_eq!(count_numbers(&words), 0);
        include_number_in_words(&mut rng, &mut words);
        assert_eq!(count_numbers(&words), 1);
    }
}
