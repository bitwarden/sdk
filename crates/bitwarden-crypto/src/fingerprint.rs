//! # Fingerprint
//!
//! Provides a way to derive fingerprints from fingerprint material and public keys. This is most
//! commonly used for account fingerprints, where the fingerprint material is the user's id and the
//! public key is the user's public key.

use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;
use sha2::Digest;
use thiserror::Error;

use crate::{error::Result, wordlist::EFF_LONG_WORD_LIST, CryptoError};

/// Computes a fingerprint of the given `fingerprint_material` using the given `public_key`.
///
/// This is commonly used for account fingerprints. With the following arguments:
/// - `fingerprint_material`: user's id.
/// - `public_key`: user's public key.
pub fn fingerprint(fingerprint_material: &str, public_key: &[u8]) -> Result<String> {
    let mut h = sha2::Sha256::new();
    h.update(public_key);
    h.finalize();

    let hkdf =
        hkdf::Hkdf::<sha2::Sha256>::from_prk(public_key).map_err(|_| CryptoError::InvalidKeyLen)?;

    let mut user_fingerprint = [0u8; 32];
    hkdf.expand(fingerprint_material.as_bytes(), &mut user_fingerprint)
        .map_err(|_| CryptoError::InvalidKeyLen)?;

    hash_word(user_fingerprint)
}

/// Derive a 5 word phrase from a 32 byte hash.
fn hash_word(hash: [u8; 32]) -> Result<String> {
    let minimum_entropy = 64;

    let entropy_per_word = (EFF_LONG_WORD_LIST.len() as f64).log2();
    let num_words = ((minimum_entropy as f64 / entropy_per_word).ceil()).to_owned() as i64;

    let hash_arr: Vec<u8> = hash.to_vec();
    let entropy_available = hash_arr.len() * 4;
    if num_words as f64 * entropy_per_word > entropy_available as f64 {
        return Err(FingerprintError::EntropyTooSmall.into());
    }

    let mut phrase = Vec::new();

    let mut hash_number = BigUint::from_bytes_be(&hash_arr);
    for _ in 0..num_words {
        let remainder = hash_number.clone() % EFF_LONG_WORD_LIST.len();
        hash_number /= EFF_LONG_WORD_LIST.len();

        phrase.push(EFF_LONG_WORD_LIST[remainder.to_usize().unwrap()].to_string());
    }

    Ok(phrase.join("-"))
}

#[derive(Debug, Error)]
pub enum FingerprintError {
    #[error("Entropy is too small")]
    EntropyTooSmall,
}

#[cfg(test)]
mod tests {
    use super::fingerprint;

    #[test]
    fn test_fingerprint() {
        let user_id = "a09726a0-9590-49d1-a5f5-afe300b6a515";
        let key: &[u8] = &[
            48, 130, 1, 34, 48, 13, 6, 9, 42, 134, 72, 134, 247, 13, 1, 1, 1, 5, 0, 3, 130, 1, 15,
            0, 48, 130, 1, 10, 2, 130, 1, 1, 0, 187, 38, 44, 241, 110, 205, 89, 253, 25, 191, 126,
            84, 121, 202, 61, 223, 189, 244, 118, 212, 74, 139, 130, 97, 115, 164, 167, 106, 191,
            188, 233, 218, 196, 250, 187, 146, 125, 160, 150, 49, 198, 224, 176, 10, 0, 143, 99,
            230, 232, 160, 51, 104, 154, 211, 33, 80, 170, 4, 68, 80, 219, 115, 167, 114, 156, 227,
            125, 193, 128, 123, 39, 254, 191, 124, 63, 129, 44, 63, 18, 56, 161, 48, 158, 0, 27,
            146, 2, 99, 136, 75, 21, 135, 6, 118, 12, 26, 251, 184, 172, 249, 53, 78, 210, 46, 143,
            17, 104, 202, 65, 173, 229, 219, 233, 144, 163, 101, 216, 238, 152, 54, 158, 1, 195,
            50, 203, 21, 226, 12, 82, 170, 175, 170, 160, 21, 247, 248, 80, 97, 123, 0, 152, 116,
            229, 126, 221, 199, 155, 194, 192, 51, 207, 177, 240, 160, 84, 241, 41, 88, 176, 53,
            111, 28, 173, 177, 232, 158, 22, 79, 133, 152, 31, 32, 12, 196, 147, 58, 57, 50, 252,
            208, 131, 150, 179, 132, 178, 150, 234, 251, 143, 125, 163, 144, 20, 46, 71, 168, 252,
            164, 86, 120, 124, 56, 252, 206, 210, 236, 212, 139, 127, 189, 236, 40, 46, 2, 238, 13,
            216, 40, 48, 85, 133, 229, 181, 155, 176, 217, 241, 154, 153, 213, 112, 222, 72, 219,
            197, 3, 219, 56, 77, 109, 47, 72, 251, 131, 36, 240, 96, 169, 31, 82, 93, 166, 242, 3,
            33, 213, 2, 3, 1, 0, 1,
        ];

        assert_eq!(
            "turban-deftly-anime-chatroom-unselfish",
            fingerprint(user_id, key).unwrap()
        );
    }
}
