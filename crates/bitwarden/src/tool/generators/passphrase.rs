use crate::{
    error::{Error, Result},
    wordlist::EFF_LONG_WORD_LIST,
};
use rand::{seq::SliceRandom, Rng, RngCore};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Passphrase generator request options.
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct PassphraseGeneratorRequest {
    /// Number of words in the generated passphrase.
    /// This value must be between 3 and 20.
    pub num_words: u8,
    /// Character separator between words in the generated passphrase.
    /// If the value is set, it cannot be empty.
    pub word_separator: String,
    /// When set to true, capitalize the first letter of each word in the generated passphrase.
    pub capitalize: bool,
    /// When set to true, include a number at the end of one of the words in the generated passphrase.
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

/// Implementation of the random passphrase generator. This is not accessible to the public API.
/// See [`ClientGenerator::passphrase`](crate::ClientGenerator::passphrase) for the API function.
pub(super) fn passphrase(options: PassphraseGeneratorRequest) -> Result<String> {
    passphrase_with_rng(rand::thread_rng(), options)
}

fn passphrase_with_rng(
    mut rng: impl RngCore,
    options: PassphraseGeneratorRequest,
) -> Result<String> {
    if !(MINIMUM_PASSPHRASE_NUM_WORDS..=MAXIMUM_PASSPHRASE_NUM_WORDS).contains(&options.num_words) {
        return Err(Error::Internal("'num_words' must be between 3 and 20"));
    }

    let Some(separator) = options.word_separator.chars().next() else {
        return Err(Error::Internal("'word_separator' cannot be empty"));
    };

    let mut passphrase_words = gen_words(&mut rng, options.num_words);
    if options.include_number {
        include_number_in_words(&mut rng, &mut passphrase_words);
    }
    if options.capitalize {
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
    // Unicode case conversion can change the length of the string, so we can't capitalize in place.
    // Instead we extract the first character and convert it to uppercase. This returns
    // an iterator which we collect into a string, and then append the rest of the input.
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
            num_words: 4,
            word_separator: "-".into(),
            capitalize: true,
            include_number: true,
        };
        assert_eq!(
            passphrase_with_rng(&mut rng, input).unwrap(),
            "Subsystem4-Undertook-Silenced-Dinginess"
        );

        let input = PassphraseGeneratorRequest {
            num_words: 3,
            word_separator: " ".into(),
            capitalize: false,
            include_number: true,
        };
        assert_eq!(
            passphrase_with_rng(&mut rng, input).unwrap(),
            "drew7 hankering cabana"
        );

        let input = PassphraseGeneratorRequest {
            num_words: 5,
            word_separator: ";".into(),
            capitalize: false,
            include_number: false,
        };
        assert_eq!(
            passphrase_with_rng(&mut rng, input).unwrap(),
            "duller;backlight;factual;husked;remover"
        );
    }
}
