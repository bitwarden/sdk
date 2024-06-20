use std::pin::Pin;

use generic_array::{typenum::U32, GenericArray};
use sha2::Digest;

use crate::{util::hkdf_expand, Kdf, Result, SymmetricCryptoKey};

/// Derive a generic key from a secret and salt using the provided KDF.
pub(super) fn derive_kdf_key(secret: &[u8], salt: &[u8], kdf: &Kdf) -> Result<SymmetricCryptoKey> {
    let mut hash = match kdf {
        Kdf::PBKDF2 { iterations } => crate::util::pbkdf2(secret, salt, iterations.get()),

        Kdf::Argon2id {
            iterations,
            memory,
            parallelism,
        } => {
            use argon2::*;

            let argon = Argon2::new(
                Algorithm::Argon2id,
                Version::V0x13,
                Params::new(
                    memory.get() * 1024, // Convert MiB to KiB
                    iterations.get(),
                    parallelism.get(),
                    Some(32),
                )?,
            );

            let salt_sha = sha2::Sha256::new().chain_update(salt).finalize();

            let mut hash = [0u8; 32];
            argon.hash_password_into(secret, &salt_sha, &mut hash)?;

            // Argon2 is using some stack memory that is not zeroed. Eventually some function will
            // overwrite the stack, but we use this trick to force the used stack to be zeroed.
            #[inline(never)]
            fn clear_stack() {
                std::hint::black_box([0u8; 4096]);
            }
            clear_stack();

            hash
        }
    };
    SymmetricCryptoKey::try_from(hash.as_mut_slice())
}

pub(super) fn stretch_kdf_key(k: &SymmetricCryptoKey) -> Result<SymmetricCryptoKey> {
    let key: Pin<Box<GenericArray<u8, U32>>> = hkdf_expand(&k.key, Some("enc"))?;
    let mac_key: Pin<Box<GenericArray<u8, U32>>> = hkdf_expand(&k.key, Some("mac"))?;

    Ok(SymmetricCryptoKey::new(key, Some(mac_key)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stretch_kdf_key() {
        let key = SymmetricCryptoKey::new(
            Box::pin(
                [
                    31, 79, 104, 226, 150, 71, 177, 90, 194, 80, 172, 209, 17, 129, 132, 81, 138,
                    167, 69, 167, 254, 149, 2, 27, 39, 197, 64, 42, 22, 195, 86, 75,
                ]
                .into(),
            ),
            None,
        );

        let stretched = stretch_kdf_key(&key).unwrap();

        assert_eq!(
            [
                111, 31, 178, 45, 238, 152, 37, 114, 143, 215, 124, 83, 135, 173, 195, 23, 142,
                134, 120, 249, 61, 132, 163, 182, 113, 197, 189, 204, 188, 21, 237, 96
            ],
            stretched.key.as_slice()
        );
        assert_eq!(
            [
                221, 127, 206, 234, 101, 27, 202, 38, 86, 52, 34, 28, 78, 28, 185, 16, 48, 61, 127,
                166, 209, 247, 194, 87, 232, 26, 48, 85, 193, 249, 179, 155
            ],
            stretched.mac_key.as_ref().unwrap().as_slice()
        );
    }
}
