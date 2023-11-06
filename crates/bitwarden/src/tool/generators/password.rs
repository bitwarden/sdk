use crate::error::{Error, Result};
use rand::{distributions::Distribution, seq::SliceRandom, RngCore};
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
    pub length: u8,

    /// When set to true, the generated password will not contain ambiguous characters.
    /// The ambiguous characters are: I, O, l, 0, 1
    pub avoid_ambiguous: bool, // TODO: Should we rename this to include_all_characters?

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

const DEFAULT_PASSWORD_LENGTH: u8 = 16;

// We need to implement this manually so we can set one character set to true.
// Otherwise the default implementation will fail to generate a password.
impl Default for PasswordGeneratorRequest {
    fn default() -> Self {
        Self {
            lowercase: true,
            uppercase: false,
            numbers: false,
            special: false,
            length: DEFAULT_PASSWORD_LENGTH,
            avoid_ambiguous: false,
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

const UPPER_CHARS_AMBIGUOUS: &[char] = &['I', 'O'];
const LOWER_CHARS_AMBIGUOUS: &[char] = &['l'];
const NUMBER_CHARS_AMBIGUOUS: &[char] = &['0', '1'];
const SPECIAL_CHARS: &[char] = &['!', '@', '#', '$', '%', '^', '&', '*'];

// We don't want the validated struct to be accessible, yet at the same time it needs to be public
// to be used as a return type, so we define it in a private module to make it innaccessible.
mod private {
    use std::collections::BTreeSet;

    use rand::prelude::Distribution;

    /// A set of characters used to generate a password. This set is backed by a BTreeSet
    /// to have consistent ordering between runs. This is notimportant during normal execution,
    /// but it's necessary for the tests to be repeatable.
    /// To create an instance, use [`CharSet::default()`](CharSet::default)
    #[derive(Clone, Default)]
    pub struct CharSet(BTreeSet<char>);
    impl CharSet {
        /// Includes the given characters in the set. Any duplicate items will be ignored
        pub fn include(self, other: impl IntoIterator<Item = char>) -> Self {
            self.include_if(true, other)
        }

        /// Includes the given characters in the set if the predicate is true. Any duplicate items will be ignored
        pub fn include_if(
            mut self,
            predicate: bool,
            other: impl IntoIterator<Item = char>,
        ) -> Self {
            if predicate {
                self.0.extend(other);
            }
            self
        }

        /// Excludes the given characters from the set. Any missing items will be ignored
        pub fn exclude_if<'a>(
            self,
            predicate: bool,
            other: impl IntoIterator<Item = &'a char>,
        ) -> Self {
            if predicate {
                let other: BTreeSet<_> = other.into_iter().copied().collect();
                Self(self.0.difference(&other).copied().collect())
            } else {
                self
            }
        }
    }
    impl<'a> IntoIterator for &'a CharSet {
        type Item = char;
        type IntoIter = std::iter::Copied<std::collections::btree_set::Iter<'a, char>>;
        fn into_iter(self) -> Self::IntoIter {
            self.0.iter().copied()
        }
    }
    impl Distribution<char> for CharSet {
        fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> char {
            let idx = rng.gen_range(0..self.0.len());
            *self.0.iter().nth(idx).expect("Valid index")
        }
    }

    /// Represents a set of valid options to generate a password with.
    /// To get an instance of it, use [`PasswordGeneratorRequest::validate_options`](PasswordGeneratorRequest::validate_options)
    pub struct PasswordGeneratorOptions {
        pub(super) lower: (CharSet, usize),
        pub(super) upper: (CharSet, usize),
        pub(super) number: (CharSet, usize),
        pub(super) special: (CharSet, usize),
        pub(super) all: (CharSet, usize),

        pub(super) length: usize,
    }
}
use private::{CharSet, PasswordGeneratorOptions};

impl PasswordGeneratorRequest {
    /// Validates the request and returns an immutable struct with valid options to use with [`password`](password).
    pub fn validate_options(self) -> Result<PasswordGeneratorOptions> {
        // TODO: Add password generator policy checks

        // We always have to have at least one character set enabled
        if !self.lowercase && !self.uppercase && !self.numbers && !self.special {
            return Err(Error::Internal(
                "At least one character set must be enabled",
            ));
        }

        // Make sure the minimum values are zero when the character
        // set is disabled, and at least one when it's enabled
        fn get_minimum(min: Option<u8>, enabled: bool) -> usize {
            if enabled {
                usize::max(min.unwrap_or(1) as usize, 1)
            } else {
                0
            }
        }

        let length = self.length as usize;
        let min_lowercase = get_minimum(self.min_lowercase, self.lowercase);
        let min_uppercase = get_minimum(self.min_uppercase, self.uppercase);
        let min_number = get_minimum(self.min_number, self.numbers);
        let min_special = get_minimum(self.min_special, self.special);

        // Check that the minimum lengths aren't larger than the password length
        let minimum_length = min_lowercase + min_uppercase + min_number + min_special;
        if minimum_length > length {
            return Err(Error::Internal(
                "Password length can't be less than the sum of the minimums",
            ));
        }

        let lower = (
            CharSet::default()
                .include_if(self.lowercase, 'a'..='z')
                .exclude_if(self.avoid_ambiguous, LOWER_CHARS_AMBIGUOUS),
            min_lowercase,
        );

        let upper = (
            CharSet::default()
                .include_if(self.uppercase, 'A'..='Z')
                .exclude_if(self.avoid_ambiguous, UPPER_CHARS_AMBIGUOUS),
            min_uppercase,
        );

        let number = (
            CharSet::default()
                .include_if(self.numbers, '0'..='9')
                .exclude_if(self.avoid_ambiguous, NUMBER_CHARS_AMBIGUOUS),
            min_number,
        );

        let special = (
            CharSet::default().include_if(self.special, SPECIAL_CHARS.iter().copied()),
            min_special,
        );

        let all = (
            CharSet::default()
                .include(&lower.0)
                .include(&upper.0)
                .include(&number.0)
                .include(&special.0),
            length - minimum_length,
        );

        Ok(PasswordGeneratorOptions {
            lower,
            upper,
            number,
            special,
            all,
            length,
        })
    }
}

/// Implementation of the random password generator. This is not accessible to the public API.
/// See [`ClientGenerator::password`](crate::ClientGenerator::password) for the API function.
pub(super) fn password(input: PasswordGeneratorOptions) -> String {
    password_with_rng(rand::thread_rng(), input)
}

pub(super) fn password_with_rng(mut rng: impl RngCore, input: PasswordGeneratorOptions) -> String {
    let mut buf: Vec<char> = Vec::with_capacity(input.length);

    let (set, qty) = &input.all;
    buf.extend(set.sample_iter(&mut rng).take(*qty));

    let (set, qty) = &input.upper;
    buf.extend(set.sample_iter(&mut rng).take(*qty));

    let (set, qty) = &input.lower;
    buf.extend(set.sample_iter(&mut rng).take(*qty));

    let (set, qty) = &input.number;
    buf.extend(set.sample_iter(&mut rng).take(*qty));

    let (set, qty) = &input.special;
    buf.extend(set.sample_iter(&mut rng).take(*qty));

    buf.shuffle(&mut rng);

    buf.iter().collect()
}

pub(super) fn passphrase(_input: PassphraseGeneratorRequest) -> Result<String> {
    Ok("correct-horse-battery-staple".to_string())
}

#[cfg(test)]
mod test {
    use std::collections::BTreeSet;

    use rand::SeedableRng;

    use super::*;

    // We convert the slices to BTreeSets to be able to use `is_subset`
    fn ref_to_set<'a>(chars: impl IntoIterator<Item = &'a char>) -> BTreeSet<char> {
        chars.into_iter().copied().collect()
    }
    fn to_set(chars: impl IntoIterator<Item = char>) -> BTreeSet<char> {
        chars.into_iter().collect()
    }

    #[test]
    fn test_password_gen_all_charsets_enabled() {
        let mut rng = rand_chacha::ChaCha8Rng::from_seed([0u8; 32]);

        let options = PasswordGeneratorRequest {
            lowercase: true,
            uppercase: true,
            numbers: true,
            special: true,
            avoid_ambiguous: false,
            ..Default::default()
        }
        .validate_options()
        .unwrap();

        assert_eq!(to_set(&options.lower.0), to_set('a'..='z'));
        assert_eq!(to_set(&options.upper.0), to_set('A'..='Z'));
        assert_eq!(to_set(&options.number.0), to_set('0'..='9'));
        assert_eq!(to_set(&options.special.0), ref_to_set(SPECIAL_CHARS));

        let pass = password_with_rng(&mut rng, options);
        assert_eq!(pass, "Z!^B5r%hUa23dFM@");
    }

    #[test]
    fn test_password_gen_only_letters_enabled() {
        let mut rng = rand_chacha::ChaCha8Rng::from_seed([0u8; 32]);

        let options = PasswordGeneratorRequest {
            lowercase: true,
            uppercase: true,
            numbers: false,
            special: false,
            avoid_ambiguous: false,
            ..Default::default()
        }
        .validate_options()
        .unwrap();

        assert_eq!(to_set(&options.lower.0), to_set('a'..='z'));
        assert_eq!(to_set(&options.upper.0), to_set('A'..='Z'));
        assert_eq!(to_set(&options.number.0), to_set([]));
        assert_eq!(to_set(&options.special.0), to_set([]));

        let pass = password_with_rng(&mut rng, options);
        assert_eq!(pass, "NQiFrGufQMiNUAmj");
    }

    #[test]
    fn test_password_gen_only_numbers_and_lower_enabled_no_ambiguous() {
        let mut rng = rand_chacha::ChaCha8Rng::from_seed([0u8; 32]);

        let options = PasswordGeneratorRequest {
            lowercase: true,
            uppercase: false,
            numbers: true,
            special: false,
            avoid_ambiguous: true,
            ..Default::default()
        }
        .validate_options()
        .unwrap();

        assert!(to_set(&options.lower.0).is_subset(&to_set('a'..='z')));
        assert!(to_set(&options.lower.0).is_disjoint(&ref_to_set(LOWER_CHARS_AMBIGUOUS)));

        assert!(to_set(&options.number.0).is_subset(&to_set('0'..='9')));
        assert!(to_set(&options.number.0).is_disjoint(&ref_to_set(NUMBER_CHARS_AMBIGUOUS)));

        assert_eq!(to_set(&options.upper.0), to_set([]));
        assert_eq!(to_set(&options.special.0), to_set([]));

        let pass = password_with_rng(&mut rng, options);
        assert_eq!(pass, "mnjabfz5ct272prf");
    }

    #[test]
    fn test_password_gen_only_upper_and_special_enabled_no_ambiguous() {
        let mut rng = rand_chacha::ChaCha8Rng::from_seed([0u8; 32]);

        let options = PasswordGeneratorRequest {
            lowercase: false,
            uppercase: true,
            numbers: false,
            special: true,
            avoid_ambiguous: true,
            ..Default::default()
        }
        .validate_options()
        .unwrap();

        assert!(to_set(&options.upper.0).is_subset(&to_set('A'..='Z')));
        assert!(to_set(&options.upper.0).is_disjoint(&ref_to_set(UPPER_CHARS_AMBIGUOUS)));

        assert_eq!(to_set(&options.special.0), ref_to_set(SPECIAL_CHARS));

        assert_eq!(to_set(&options.lower.0), to_set([]));
        assert_eq!(to_set(&options.number.0), to_set([]));

        let pass = password_with_rng(&mut rng, options);
        assert_eq!(pass, "B*GBQANS%UZPQD!K");
    }

    #[test]
    fn test_password_gen_minimum_limits() {
        let mut rng = rand_chacha::ChaCha8Rng::from_seed([0u8; 32]);

        let options = PasswordGeneratorRequest {
            lowercase: true,
            uppercase: true,
            numbers: true,
            special: true,
            avoid_ambiguous: false,
            length: 24,
            min_lowercase: Some(5),
            min_uppercase: Some(5),
            min_number: Some(5),
            min_special: Some(5),
        }
        .validate_options()
        .unwrap();

        assert_eq!(to_set(&options.lower.0), to_set('a'..='z'));
        assert_eq!(to_set(&options.upper.0), to_set('A'..='Z'));
        assert_eq!(to_set(&options.number.0), to_set('0'..='9'));
        assert_eq!(to_set(&options.special.0), ref_to_set(SPECIAL_CHARS));

        assert_eq!(options.lower.1, 5);
        assert_eq!(options.upper.1, 5);
        assert_eq!(options.number.1, 5);
        assert_eq!(options.special.1, 5);

        let pass = password_with_rng(&mut rng, options);
        assert_eq!(pass, "236q5!a#R%PG5rI%k1!*@uRt");
    }
}
