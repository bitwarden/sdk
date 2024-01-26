use bitwarden_crypto::EFF_LONG_WORD_LIST;
use rand::{seq::SliceRandom, Rng, RngCore};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::util::capitalize_first_letter;

#[derive(Debug, Error)]
pub enum PassphraseError {
    #[error("'num_words' must be between {} and {}", minimum, maximum)]
    InvalidNumWords { minimum: u8, maximum: u8 },
    #[error("'word_separator' cannot be empty")]
    EmptyWordSeparator,
}

/// Passphrase generator request options.
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct PassphraseGeneratorRequest {
    /// Number of words in the generated passphrase.
    /// This value must be between 3 and 20.
    pub num_words: u8,
    /// Character separator between words in the generated passphrase. The value cannot be empty.
    pub word_separator: String,
    /// When set to true, capitalize the first letter of each word in the generated passphrase.
    pub capitalize: bool,
    /// When set to true, include a number at the end of one of the words in the generated
    /// passphrase.
    pub include_number: bool,
}

impl Default for PassphraseGeneratorRequest {
    fn default() -> Self {
        Self {
            num_words: 3,
            word_separator: ' '.to_string(),
            capitalize: false,
            include_number: false,
        }
    }
}

const MINIMUM_PASSPHRASE_NUM_WORDS: u8 = 3;
const MAXIMUM_PASSPHRASE_NUM_WORDS: u8 = 20;

/// Represents a set of valid options to generate a passhprase with.
/// To get an instance of it, use
/// [`PassphraseGeneratorRequest::validate_options`](PassphraseGeneratorRequest::validate_options)
struct ValidPassphraseGeneratorOptions {
    pub(super) num_words: u8,
    pub(super) word_separator: String,
    pub(super) capitalize: bool,
    pub(super) include_number: bool,
}

impl PassphraseGeneratorRequest {
    /// Validates the request and returns an immutable struct with valid options to use with the
    /// passphrase generator.
    fn validate_options(self) -> Result<ValidPassphraseGeneratorOptions, PassphraseError> {
        // TODO: Add password generator policy checks

        if !(MINIMUM_PASSPHRASE_NUM_WORDS..=MAXIMUM_PASSPHRASE_NUM_WORDS).contains(&self.num_words)
        {
            return Err(PassphraseError::InvalidNumWords {
                minimum: MINIMUM_PASSPHRASE_NUM_WORDS,
                maximum: MAXIMUM_PASSPHRASE_NUM_WORDS,
            });
        }

        if self.word_separator.chars().next().is_none() {
            return Err(PassphraseError::EmptyWordSeparator);
        };

        Ok(ValidPassphraseGeneratorOptions {
            num_words: self.num_words,
            word_separator: self.word_separator,
            capitalize: self.capitalize,
            include_number: self.include_number,
        })
    }
}

/// Implementation of the random passphrase generator.
pub fn passphrase(request: PassphraseGeneratorRequest) -> Result<String, PassphraseError> {
    let options = request.validate_options()?;
    Ok(passphrase_with_rng(rand::thread_rng(), options))
}

fn passphrase_with_rng(mut rng: impl RngCore, options: ValidPassphraseGeneratorOptions) -> String {
    let mut passphrase_words = gen_words(&mut rng, options.num_words);
    if options.include_number {
        include_number_in_words(&mut rng, &mut passphrase_words);
    }
    if options.capitalize {
        capitalize_words(&mut passphrase_words);
    }
    passphrase_words.join(&options.word_separator)
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
    fn test_separator() {
        let mut rng = rand_chacha::ChaCha8Rng::from_seed([0u8; 32]);

        let input = PassphraseGeneratorRequest {
            num_words: 4,
            word_separator: "👨🏻‍❤️‍💋‍👨🏻".into(), /* This emoji is 35 bytes long, but represented
                                                   * as a single character */
            capitalize: false,
            include_number: true,
        }
        .validate_options()
        .unwrap();
        assert_eq!(
            passphrase_with_rng(&mut rng, input),
            "subsystem4👨🏻‍❤️‍💋‍👨🏻undertook👨🏻‍❤️‍💋‍👨🏻silenced👨🏻‍❤️‍💋‍👨🏻dinginess"
        );
    }

    #[test]
    fn test_passphrase() {
        let mut rng = rand_chacha::ChaCha8Rng::from_seed([0u8; 32]);

        let input = PassphraseGeneratorRequest {
            num_words: 4,
            word_separator: "-".into(),
            capitalize: true,
            include_number: true,
        }
        .validate_options()
        .unwrap();
        assert_eq!(
            passphrase_with_rng(&mut rng, input),
            "Subsystem4-Undertook-Silenced-Dinginess"
        );

        let input = PassphraseGeneratorRequest {
            num_words: 3,
            word_separator: " ".into(),
            capitalize: false,
            include_number: true,
        }
        .validate_options()
        .unwrap();
        assert_eq!(
            passphrase_with_rng(&mut rng, input),
            "drew7 hankering cabana"
        );

        let input = PassphraseGeneratorRequest {
            num_words: 5,
            word_separator: ";".into(),
            capitalize: false,
            include_number: false,
        }
        .validate_options()
        .unwrap();
        assert_eq!(
            passphrase_with_rng(&mut rng, input),
            "duller;backlight;factual;husked;remover"
        );
    }
}
