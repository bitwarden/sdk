//! Cryptographic primitives used in the SDK

use aes::cipher::typenum::U32;
use base64::Engine;

use crate::util::BASE64_ENGINE;

use super::{
    hkdf_expand, EncString, PbkdfSha256Hmac, SymmetricCryptoKey, PBKDF_SHA256_HMAC_OUT_SIZE,
};
use {
    crate::{client::auth_settings::Kdf, error::Result},
    aes::cipher::generic_array::GenericArray,
    sha2::Digest,
};

/// A Master Key.
pub(crate) struct MasterKey(SymmetricCryptoKey);

/// Derives a users master key from their password, email and KDF.
pub(crate) fn derive_master_key(password: &[u8], email: &[u8], kdf: &Kdf) -> Result<MasterKey> {
    derive_key(password, email, kdf).map(MasterKey)
}

/// Derive a generic key from a secret and salt using the provided KDF.
fn derive_key(secret: &[u8], salt: &[u8], kdf: &Kdf) -> Result<SymmetricCryptoKey> {
    let hash = match kdf {
        Kdf::PBKDF2 { iterations } => pbkdf2::pbkdf2_array::<
            PbkdfSha256Hmac,
            PBKDF_SHA256_HMAC_OUT_SIZE,
        >(secret, salt, iterations.get())
        .unwrap(),

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
                )
                .unwrap(),
            );

            let salt_sha = sha2::Sha256::new().chain_update(salt).finalize();

            let mut hash = [0u8; 32];
            argon
                .hash_password_into(secret, &salt_sha, &mut hash)
                .unwrap();
            hash
        }
    };
    SymmetricCryptoKey::try_from(hash.as_slice())
}

/// Derive a login password hash
pub(crate) fn derive_password_hash(password: &[u8], salt: &[u8], kdf: &Kdf) -> Result<String> {
    let hash: MasterKey = derive_master_key(password, salt, kdf)?;

    let login_hash = derive_master_key_hash(hash, password, HashPurpose::ServerAuthorization);

    Ok(BASE64_ENGINE.encode(login_hash))
}

#[derive(Copy, Clone)]
enum HashPurpose {
    ServerAuthorization = 1,
    // LocalAuthorization = 2,
}

fn derive_master_key_hash(
    master_key: MasterKey,
    password: &[u8],
    purpose: HashPurpose,
) -> [u8; 32] {
    pbkdf2::pbkdf2_array::<PbkdfSha256Hmac, PBKDF_SHA256_HMAC_OUT_SIZE>(
        &master_key.0.key,
        password,
        purpose as u32,
    )
    .expect("hash is a valid fixed size")
}

fn stretch_master_key(master_key: MasterKey) -> Result<SymmetricCryptoKey> {
    let key: GenericArray<u8, U32> = hkdf_expand(&master_key.0.key, Some("enc"))?;
    let mac_key: GenericArray<u8, U32> = hkdf_expand(&master_key.0.key, Some("mac"))?;

    Ok(SymmetricCryptoKey {
        key,
        mac_key: Some(mac_key),
    })
}

pub(crate) fn decrypt_user_key(
    master_key: MasterKey,
    user_key: EncString,
) -> Result<SymmetricCryptoKey> {
    // The master key needs to be stretched before it can be used
    let stretched_key = stretch_master_key(master_key)?;

    let dec = user_key.decrypt_with_key(&stretched_key)?;
    SymmetricCryptoKey::try_from(dec.as_slice())
}

#[cfg(test)]
mod tests {
    use crate::crypto::derive_password_hash;

    use super::{derive_master_key, stretch_master_key};
    #[cfg(feature = "internal")]
    use {crate::client::auth_settings::Kdf, std::num::NonZeroU32};

    #[cfg(feature = "internal")]
    #[test]
    fn test_key_stretch_password_pbkdf2() {
        let master_key = derive_master_key(
            &b"67t9b5g67$%Dh89n"[..],
            "test_key".as_bytes(),
            &Kdf::PBKDF2 {
                iterations: NonZeroU32::new(10000).unwrap(),
            },
        )
        .unwrap();
        let stretched = stretch_master_key(master_key).unwrap();

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
            stretched.mac_key.unwrap().as_slice()
        );
    }

    #[cfg(feature = "internal")]
    #[test]
    fn test_key_stretch_password_argon2() {
        let master_key = derive_master_key(
            &b"67t9b5g67$%Dh89n"[..],
            "test_key".as_bytes(),
            &Kdf::Argon2id {
                iterations: NonZeroU32::new(4).unwrap(),
                memory: NonZeroU32::new(32).unwrap(),
                parallelism: NonZeroU32::new(2).unwrap(),
            },
        )
        .unwrap();
        let stretched = stretch_master_key(master_key).unwrap();

        assert_eq!(
            [
                236, 253, 166, 121, 207, 124, 98, 149, 42, 141, 97, 226, 207, 71, 173, 60, 10, 0,
                184, 255, 252, 87, 62, 32, 188, 166, 173, 223, 146, 159, 222, 219
            ],
            stretched.key.as_slice()
        );
        assert_eq!(
            [
                214, 144, 76, 173, 225, 106, 132, 131, 173, 56, 134, 241, 223, 227, 165, 161, 146,
                37, 111, 206, 155, 24, 224, 151, 134, 189, 202, 0, 27, 149, 131, 21
            ],
            stretched.mac_key.unwrap().as_slice()
        );
    }

    #[test]
    fn test_password_hash_pbkdf2() {
        assert_eq!(
            "ZF6HjxUTSyBHsC+HXSOhZoXN+UuMnygV5YkWXCY4VmM=",
            derive_password_hash(
                "asdfasdf".as_bytes(),
                "test_salt".as_bytes(),
                &Kdf::PBKDF2 {
                    iterations: NonZeroU32::new(100_000).unwrap()
                }
            )
            .unwrap(),
        );
    }

    #[test]
    fn test_password_hash_argon2id() {
        assert_eq!(
            "PR6UjYmjmppTYcdyTiNbAhPJuQQOmynKbdEl1oyi/iQ=",
            derive_password_hash(
                "asdfasdf".as_bytes(),
                "test_salt".as_bytes(),
                &Kdf::Argon2id {
                    iterations: NonZeroU32::new(4).unwrap(),
                    memory: NonZeroU32::new(32).unwrap(),
                    parallelism: NonZeroU32::new(2).unwrap()
                }
            )
            .unwrap(),
        );
    }
}
