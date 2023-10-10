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
    passphrase_with_rng(rand::thread_rng(), input)
}

fn passphrase_with_rng(mut rng: impl RngCore, input: PassphraseGeneratorRequest) -> Result<String> {
    let num_words = input.num_words.unwrap_or(DEFAULT_PASSPHRASE_NUM_WORDS);
    let separator = input
        .word_separator
        .and_then(|s| s.chars().next())
        .unwrap_or(DEFAULT_PASSPHRASE_SEPARATOR);
    let capitalize = input.capitalize.unwrap_or(false);
    let include_number = input.include_number.unwrap_or(false);

    let mut passphrase_words = gen_words(&mut rng, num_words);
    if include_number {
        include_number_in_words(&mut rng, &mut passphrase_words);
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
    use rand::SeedableRng;

    use super::*;

    #[test]
    fn test_gen_words() {
        let mut rng = rand_chacha::ChaCha8Rng::from_seed([0u8; 32]);
        assert_eq!(
            &gen_words(&mut rng, 4),
            &["subsystem", "undertook", "silenced", "dinginess"]
        );
        assert_eq!(&gen_words(&mut rng, 1), &["numbing"]);
        assert_eq!(&gen_words(&mut rng, 2), &["catnip", "jokester"]);
    }

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
        let mut words = vec!["hello".into(), "world".into()];
        capitalize_words(&mut words);
        assert_eq!(words, &["Hello", "World"]);
    }

    #[test]
    fn test_include_number() {
        let mut rng = rand_chacha::ChaCha8Rng::from_seed([0u8; 32]);

        let mut words = vec!["hello".into(), "world".into()];
        include_number_in_words(&mut rng, &mut words);
        assert_eq!(words, &["hello", "world7"]);

        let mut words = vec!["This".into(), "is".into(), "a".into(), "test".into()];
        include_number_in_words(&mut rng, &mut words);
        assert_eq!(words, &["This", "is", "a1", "test"]);
    }

    #[test]
    fn test_passphrase() {
        let mut rng = rand_chacha::ChaCha8Rng::from_seed([0u8; 32]);

        let input = PassphraseGeneratorRequest {
            num_words: Some(4),
            word_separator: Some("-".into()),
            capitalize: Some(true),
            include_number: Some(true),
        };
        assert_eq!(
            passphrase_with_rng(&mut rng, input).unwrap(),
            "Subsystem4-Undertook-Silenced-Dinginess"
        );

        let input = PassphraseGeneratorRequest {
            num_words: Some(2),
            word_separator: Some("".into()),
            capitalize: Some(false),
            include_number: Some(true),
        };
        assert_eq!(
            passphrase_with_rng(&mut rng, input).unwrap(),
            "drew hankering0"
        );

        let input = PassphraseGeneratorRequest {
            num_words: Some(5),
            word_separator: Some(";".into()),
            capitalize: None,
            include_number: None,
        };
        assert_eq!(
            passphrase_with_rng(&mut rng, input).unwrap(),
            "sarcasm;duller;backlight;factual;husked"
        );
    }
}
