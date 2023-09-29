use crate::error::Result;
use rand::seq::SliceRandom;
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
    pub min_lowercase: Option<u8>,
    pub min_uppercase: Option<u8>,
    pub min_number: Option<u8>,
    pub min_special: Option<u8>,
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

const DEFAULT_PASSWORD_LENGTH: u8 = 16;

const UPPER_CHARS_AMBIGUOUS: [char; 2] = ['I', 'O'];
const UPPER_CHARS: [char; 24] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z',
];
const LOWER_CHARS_AMBIGUOUS: [char; 1] = ['l'];
const LOWER_CHARS: [char; 25] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't',
    'u', 'v', 'w', 'x', 'y', 'z',
];
const NUMBER_CHARS_AMBIGUOUS: [char; 2] = ['0', '1'];
const NUMBER_CHARS: [char; 8] = ['2', '3', '4', '5', '6', '7', '8', '9'];
const SPECIAL_CHARS: [char; 8] = ['!', '@', '#', '$', '%', '^', '&', '*'];

pub(super) fn password(input: PasswordGeneratorRequest) -> Result<String> {
    // Generate all character dictionaries
    fn gen_chars(chars: &[char], ambiguous: &[char], avoid_ambiguous: bool) -> Vec<char> {
        let mut chars = chars.to_vec();
        if !avoid_ambiguous {
            chars.extend_from_slice(ambiguous);
        }
        chars
    }
    let avoid_ambiguous = input.avoid_ambiguous.unwrap_or(false);
    let lower_chars = gen_chars(&LOWER_CHARS, &LOWER_CHARS_AMBIGUOUS, avoid_ambiguous);
    let upper_chars = gen_chars(&UPPER_CHARS, &UPPER_CHARS_AMBIGUOUS, avoid_ambiguous);
    let number_chars = gen_chars(&NUMBER_CHARS, &NUMBER_CHARS_AMBIGUOUS, avoid_ambiguous);
    let all_chars = lower_chars
        .iter()
        .chain(&upper_chars)
        .chain(&number_chars)
        .chain(&SPECIAL_CHARS)
        .collect::<Vec<_>>();

    // We always have to have at least one character type
    let lowercase = input.lowercase || (!input.uppercase && !input.numbers && !input.special);

    // Sanitize the minimum values
    fn get_minimum(min: Option<u8>, enabled: bool) -> u8 {
        if enabled {
            // Make sure there's at least one
            u8::max(min.unwrap_or(1), 1)
        } else {
            0
        }
    }
    let min_lowercase = get_minimum(input.min_lowercase, lowercase);
    let min_uppercase = get_minimum(input.min_uppercase, input.uppercase);
    let min_number = get_minimum(input.min_number, input.numbers);
    let min_special = get_minimum(input.min_special, input.special);

    // Sanitize the length value
    let min_length = min_lowercase + min_uppercase + min_number + min_special;
    let length = u8::max(input.length.unwrap_or(DEFAULT_PASSWORD_LENGTH), min_length);

    // Generate the minimum chars of each type, then generate the rest to fill the expected length
    let mut chars = Vec::with_capacity(length as usize);
    let mut rand = rand::thread_rng();

    for _ in 0..min_lowercase {
        chars.push(*lower_chars.choose(&mut rand).expect("slice is not empty"));
    }
    for _ in 0..min_uppercase {
        chars.push(*upper_chars.choose(&mut rand).expect("slice is not empty"));
    }
    for _ in 0..min_number {
        chars.push(*number_chars.choose(&mut rand).expect("slice is not empty"));
    }
    for _ in 0..min_special {
        chars.push(*SPECIAL_CHARS.choose(&mut rand).expect("slice is not empty"));
    }
    for _ in min_length..length {
        chars.push(**all_chars.choose(&mut rand).expect("slice is not empty"));
    }

    chars.shuffle(&mut rand);
    Ok(chars.iter().collect())
}

pub(super) fn passphrase(_input: PassphraseGeneratorRequest) -> Result<String> {
    Ok("correct-horse-battery-staple".to_string())
}
