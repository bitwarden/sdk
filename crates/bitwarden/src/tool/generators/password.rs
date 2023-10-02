use crate::error::{Error, Result};
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

const UPPER_CHARS_AMBIGUOUS: &[char] = &['I', 'O'];
const UPPER_CHARS: &[char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z',
];
const LOWER_CHARS_AMBIGUOUS: &[char] = &['l'];
const LOWER_CHARS: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't',
    'u', 'v', 'w', 'x', 'y', 'z',
];
const NUMBER_CHARS_AMBIGUOUS: &[char] = &['0', '1'];
const NUMBER_CHARS: &[char] = &['2', '3', '4', '5', '6', '7', '8', '9'];
const SPECIAL_CHARS: &[char] = &['!', '@', '#', '$', '%', '^', '&', '*'];

struct PasswordGeneratorCharSet {
    lower: Vec<char>,
    upper: Vec<char>,
    number: Vec<char>,
    special: Vec<char>,
    all: Vec<char>,
}

impl PasswordGeneratorCharSet {
    fn new(lower: bool, upper: bool, number: bool, special: bool, avoid_ambiguous: bool) -> Self {
        fn chars(
            enabled: bool,
            chars: &[char],
            ambiguous: &[char],
            avoid_ambiguous: bool,
        ) -> Vec<char> {
            if !enabled {
                return Vec::new();
            }
            let mut chars = chars.to_vec();
            if !avoid_ambiguous {
                chars.extend_from_slice(ambiguous);
            }
            chars
        }
        let lower = chars(lower, LOWER_CHARS, LOWER_CHARS_AMBIGUOUS, avoid_ambiguous);
        let upper = chars(upper, UPPER_CHARS, UPPER_CHARS_AMBIGUOUS, avoid_ambiguous);
        let number = chars(
            number,
            NUMBER_CHARS,
            NUMBER_CHARS_AMBIGUOUS,
            avoid_ambiguous,
        );
        let special = chars(special, SPECIAL_CHARS, &[], avoid_ambiguous);
        let all = lower
            .iter()
            .chain(&upper)
            .chain(&number)
            .chain(&special)
            .copied()
            .collect();

        Self {
            lower,
            upper,
            number,
            special,
            all,
        }
    }
}

pub(super) fn password(input: PasswordGeneratorRequest) -> Result<String> {
    // We always have to have at least one character set enabled
    if !input.lowercase || !input.uppercase && !input.numbers && !input.special {
        return Err(Error::Internal(
            "At least one character set must be enabled",
        ));
    }

    // Generate all character dictionaries
    let chars = PasswordGeneratorCharSet::new(
        input.lowercase,
        input.uppercase,
        input.numbers,
        input.special,
        input.avoid_ambiguous.unwrap_or(false),
    );

    // Make sure the minimum values are zero when the character
    // set is disabled, and at least one when it's enabled
    fn get_minimum(min: Option<u8>, enabled: bool) -> u8 {
        if enabled {
            u8::max(min.unwrap_or(1), 1)
        } else {
            0
        }
    }
    let min_lowercase = get_minimum(input.min_lowercase, input.lowercase);
    let min_uppercase = get_minimum(input.min_uppercase, input.uppercase);
    let min_number = get_minimum(input.min_number, input.numbers);
    let min_special = get_minimum(input.min_special, input.special);

    // Check that the minimum lengths aren't larger than the password length
    let min_length = min_lowercase + min_uppercase + min_number + min_special;
    let length = input.length.unwrap_or(DEFAULT_PASSWORD_LENGTH);
    if min_length > length {
        return Err(Error::Internal(
            "Password length can't be less than the sum of the minimums",
        ));
    }

    // Generate the minimum chars of each type, then generate the rest to fill the expected length
    let mut buf = Vec::with_capacity(length as usize);
    let mut rand = rand::thread_rng();

    for _ in 0..min_lowercase {
        buf.push(*chars.lower.choose(&mut rand).expect("slice is not empty"));
    }
    for _ in 0..min_uppercase {
        buf.push(*chars.upper.choose(&mut rand).expect("slice is not empty"));
    }
    for _ in 0..min_number {
        buf.push(*chars.number.choose(&mut rand).expect("slice is not empty"));
    }
    for _ in 0..min_special {
        buf.push(*chars.special.choose(&mut rand).expect("slice is not empty"));
    }
    for _ in min_length..length {
        buf.push(*chars.all.choose(&mut rand).expect("slice is not empty"));
    }

    buf.shuffle(&mut rand);
    Ok(buf.iter().collect())
}

pub(super) fn passphrase(_input: PassphraseGeneratorRequest) -> Result<String> {
    Ok("correct-horse-battery-staple".to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    // We convert the slices to Strings to be able to use `contains`
    // This wouldn't work if the character sets were ordered differently, but that's not the case for us
    fn to_string(chars: &[char]) -> String {
        chars.iter().collect()
    }

    #[test]
    fn test_password_characters() {
        // All characters excluding ambiguous
        let set = PasswordGeneratorCharSet::new(true, true, true, true, true);
        assert_eq!(set.lower, LOWER_CHARS);
        assert_eq!(set.upper, UPPER_CHARS);
        assert_eq!(set.number, NUMBER_CHARS);
        assert_eq!(set.special, SPECIAL_CHARS);

        // All characters including ambiguous
        let set = PasswordGeneratorCharSet::new(true, true, true, true, false);
        assert!(to_string(&set.lower).contains(&to_string(LOWER_CHARS)));
        assert!(to_string(&set.lower).contains(&to_string(LOWER_CHARS_AMBIGUOUS)));
        assert!(to_string(&set.upper).contains(&to_string(UPPER_CHARS)));
        assert!(to_string(&set.upper).contains(&to_string(UPPER_CHARS_AMBIGUOUS)));
        assert!(to_string(&set.number).contains(&to_string(NUMBER_CHARS)));
        assert!(to_string(&set.number).contains(&to_string(NUMBER_CHARS_AMBIGUOUS)));
        assert_eq!(set.special, SPECIAL_CHARS);

        // Only lowercase
        let set = PasswordGeneratorCharSet::new(true, false, false, false, true);
        assert_eq!(set.lower, LOWER_CHARS);
        assert_eq!(set.upper, Vec::new());
        assert_eq!(set.number, Vec::new());
        assert_eq!(set.special, Vec::new());

        // Only uppercase including ambiguous
        let set = PasswordGeneratorCharSet::new(false, true, false, false, false);
        assert_eq!(set.lower, Vec::new());
        assert!(to_string(&set.upper).contains(&to_string(UPPER_CHARS)));
        assert!(to_string(&set.upper).contains(&to_string(UPPER_CHARS_AMBIGUOUS)));
        assert_eq!(set.number, Vec::new());
        assert_eq!(set.special, Vec::new());
    }
}
