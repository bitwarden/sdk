use crate::error::{Error, Result};
use rand::{seq::SliceRandom, RngCore};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Password generator request options. If all options are false, the default is to
/// generate a password with:
/// - lowercase
/// - uppercase
/// - numbers
///
/// The default length is 16.
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct PasswordGeneratorRequest {
    /// When set to true, the generated password will contain lowercase characters (a-z).
    pub lowercase: bool,
    /// When set to true, the generated password will contain uppercase characters (A-Z).
    pub uppercase: bool,
    /// When set to true, the generated password will contain numbers (0-9).
    pub numbers: bool,
    /// When set to true, the generated password will contain special characters.
    /// The supported characters are: ! @ # $ % ^ & *
    pub special: bool,

    /// The length of the generated password.
    /// Note that the password length must be greater than the sum of all the minimums.
    /// The default value when unset is 16.
    pub length: Option<u8>,

    /// When set to true, the generated password will not contain ambiguous characters.
    /// The ambiguous characters are: I, O, l, 0, 1
    pub avoid_ambiguous: Option<bool>, // TODO: Should we rename this to include_all_characters?

    /// The minimum number of lowercase characters in the generated password.
    /// When set, the value must be between 1 and 9. This value is ignored is lowercase is false
    pub min_lowercase: Option<u8>,
    /// The minimum number of uppercase characters in the generated password.
    /// When set, the value must be between 1 and 9. This value is ignored is uppercase is false  
    pub min_uppercase: Option<u8>,
    /// The minimum number of numbers in the generated password.
    /// When set, the value must be between 1 and 9. This value is ignored is numbers is false
    pub min_number: Option<u8>,
    /// The minimum number of special characters in the generated password.
    /// When set, the value must be between 1 and 9. This value is ignored is special is false
    pub min_special: Option<u8>,
}

// We need to implement this manually so we can set one character set to true.
// Otherwise the default implementation will fail to generate a password.
impl Default for PasswordGeneratorRequest {
    fn default() -> Self {
        Self {
            lowercase: true,
            uppercase: false,
            numbers: false,
            special: false,
            length: None,
            avoid_ambiguous: None,
            min_lowercase: None,
            min_uppercase: None,
            min_number: None,
            min_special: None,
        }
    }
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

/// Implementation of the random password generator. This is not accessible to the public API.
/// See [`ClientGenerator::password`](crate::ClientGenerator::password) for the API function.
pub(super) fn password(input: PasswordGeneratorRequest) -> Result<String> {
    password_with_rng(rand::thread_rng(), input)
}

pub(super) fn password_with_rng(
    mut rng: impl RngCore,
    input: PasswordGeneratorRequest,
) -> Result<String> {
    // We always have to have at least one character set enabled
    if !input.lowercase && !input.uppercase && !input.numbers && !input.special {
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

    for _ in 0..min_lowercase {
        buf.push(*chars.lower.choose(&mut rng).expect("slice is not empty"));
    }
    for _ in 0..min_uppercase {
        buf.push(*chars.upper.choose(&mut rng).expect("slice is not empty"));
    }
    for _ in 0..min_number {
        buf.push(*chars.number.choose(&mut rng).expect("slice is not empty"));
    }
    for _ in 0..min_special {
        buf.push(*chars.special.choose(&mut rng).expect("slice is not empty"));
    }
    for _ in min_length..length {
        buf.push(*chars.all.choose(&mut rng).expect("slice is not empty"));
    }

    buf.shuffle(&mut rng);
    Ok(buf.iter().collect())
}

pub(super) fn passphrase(_input: PassphraseGeneratorRequest) -> Result<String> {
    Ok("correct-horse-battery-staple".to_string())
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use rand::SeedableRng;

    use super::*;

    // We convert the slices to HashSets to be able to use `is_subset`
    fn to_set(chars: &[char]) -> HashSet<char> {
        chars.iter().copied().collect()
    }

    #[test]
    fn test_password_characters_all() {
        let set = PasswordGeneratorCharSet::new(true, true, true, true, true);
        assert_eq!(set.lower, LOWER_CHARS);
        assert_eq!(set.upper, UPPER_CHARS);
        assert_eq!(set.number, NUMBER_CHARS);
        assert_eq!(set.special, SPECIAL_CHARS);
    }
    #[test]
    fn test_password_characters_all_ambiguous() {
        let set = PasswordGeneratorCharSet::new(true, true, true, true, false);
        assert!(to_set(&set.lower).is_superset(&to_set(LOWER_CHARS)));
        assert!(to_set(&set.lower).is_superset(&to_set(LOWER_CHARS_AMBIGUOUS)));
        assert!(to_set(&set.upper).is_superset(&to_set(UPPER_CHARS)));
        assert!(to_set(&set.upper).is_superset(&to_set(UPPER_CHARS_AMBIGUOUS)));
        assert!(to_set(&set.number).is_superset(&to_set(NUMBER_CHARS)));
        assert!(to_set(&set.number).is_superset(&to_set(NUMBER_CHARS_AMBIGUOUS)));
        assert_eq!(set.special, SPECIAL_CHARS);
    }
    #[test]
    fn test_password_characters_lower() {
        let set = PasswordGeneratorCharSet::new(true, false, false, false, true);
        assert_eq!(set.lower, LOWER_CHARS);
        assert_eq!(set.upper, Vec::new());
        assert_eq!(set.number, Vec::new());
        assert_eq!(set.special, Vec::new());
    }
    #[test]
    fn test_password_characters_upper_ambiguous() {
        // Only uppercase including ambiguous
        let set = PasswordGeneratorCharSet::new(false, true, false, false, false);
        assert_eq!(set.lower, Vec::new());
        assert!(to_set(&set.upper).is_superset(&to_set(UPPER_CHARS)));
        assert!(to_set(&set.upper).is_superset(&to_set(UPPER_CHARS_AMBIGUOUS)));
        assert_eq!(set.number, Vec::new());
        assert_eq!(set.special, Vec::new());
    }

    #[test]
    fn test_password_gen() {
        let mut rng = rand_chacha::ChaCha8Rng::from_seed([0u8; 32]);

        let pass = password_with_rng(
            &mut rng,
            PasswordGeneratorRequest {
                lowercase: true,
                uppercase: true,
                numbers: true,
                special: true,
                ..Default::default()
            },
        )
        .unwrap();
        assert_eq!(pass, "xfZPr&wXCiFta8DM");

        let pass = password_with_rng(
            &mut rng,
            PasswordGeneratorRequest {
                lowercase: true,
                uppercase: true,
                numbers: false,
                special: false,
                length: Some(20),
                avoid_ambiguous: Some(false),
                min_lowercase: Some(1),
                min_uppercase: Some(1),
                min_number: None,
                min_special: None,
            },
        )
        .unwrap();
        assert_eq!(pass, "jvpFStaIdRUoENAeTmJw");

        let pass = password_with_rng(
            &mut rng,
            PasswordGeneratorRequest {
                lowercase: false,
                uppercase: false,
                numbers: true,
                special: true,
                length: Some(5),
                avoid_ambiguous: Some(true),
                min_lowercase: None,
                min_uppercase: None,
                min_number: Some(3),
                min_special: Some(2),
            },
        )
        .unwrap();
        assert_eq!(pass, "^878%");
    }
}
