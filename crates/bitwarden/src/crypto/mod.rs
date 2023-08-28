//! Cryptographic primitives used in the SDK

use crate::error::CryptoError;
use aes::cipher::{generic_array::GenericArray, typenum::U64, Unsigned};
use hmac::digest::OutputSizeUser;

#[cfg(feature = "internal")]
use aes::cipher::typenum::U32;

#[cfg(any(feature = "internal", feature = "mobile"))]
use {
    crate::{client::auth_settings::Kdf, error::Result},
    sha2::Digest,
};

mod enc_string;
pub use enc_string::EncString;
mod encryptable;
pub use encryptable::{Decryptable, Encryptable};
mod aes_opt;
pub use aes_opt::{decrypt_aes256, encrypt_aes256};
mod symmetric_crypto_key;
pub use symmetric_crypto_key::SymmetricCryptoKey;

#[cfg(feature = "internal")]
mod fingerprint;
#[cfg(feature = "internal")]
pub(crate) use fingerprint::fingerprint;

pub(crate) type PbkdfSha256Hmac = hmac::Hmac<sha2::Sha256>;
pub(crate) const PBKDF_SHA256_HMAC_OUT_SIZE: usize =
    <<PbkdfSha256Hmac as OutputSizeUser>::OutputSize as Unsigned>::USIZE;

#[cfg(any(feature = "internal", feature = "mobile"))]
pub(crate) fn hash_kdf(secret: &[u8], salt: &[u8], kdf: &Kdf) -> Result<[u8; 32]> {
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
    Ok(hash)
}

#[cfg(feature = "internal")]
pub(crate) fn stretch_key_password(
    secret: &[u8],
    salt: &[u8],
    kdf: &Kdf,
) -> Result<(GenericArray<u8, U32>, GenericArray<u8, U32>)> {
    let master_key: [u8; 32] = hash_kdf(secret, salt, kdf)?;

    let hkdf = hkdf::Hkdf::<sha2::Sha256>::from_prk(&master_key)
        .expect("Input is a valid fixed size hash");

    let mut key = GenericArray::<u8, U32>::default();
    hkdf.expand("enc".as_bytes(), &mut key)
        .expect("key is a valid fixed size buffer");
    let mut mac_key = GenericArray::<u8, U32>::default();
    hkdf.expand("mac".as_bytes(), &mut mac_key)
        .expect("mac_key is a valid fixed size buffer");

    Ok((key, mac_key))
}

pub(crate) fn stretch_key(secret: [u8; 16], name: &str, info: Option<&str>) -> SymmetricCryptoKey {
    use hmac::{Hmac, Mac};
    // Because all inputs are fixed size, we can unwrap all errors here without issue

    // TODO: Are these the final `key` and `info` parameters or should we change them? I followed the pattern used for sends
    let res = Hmac::<sha2::Sha256>::new_from_slice(format!("bitwarden-{}", name).as_bytes())
        .unwrap()
        .chain_update(secret)
        .finalize()
        .into_bytes();

    let hkdf = hkdf::Hkdf::<sha2::Sha256>::from_prk(&res).unwrap();

    let mut key = GenericArray::<u8, U64>::default();

    // TODO: Should we have a default value for info?
    //  Should it be required?
    let i = info.map(|i| i.as_bytes()).unwrap_or(&[]);
    hkdf.expand(i, &mut key).unwrap();

    SymmetricCryptoKey::try_from(key.as_slice()).unwrap()
}

pub fn decrypt(cipher: &EncString, key: &SymmetricCryptoKey) -> Result<Vec<u8>> {
    match cipher {
        EncString::AesCbc256_HmacSha256_B64 { iv, mac, data } => {
            let dec = decrypt_aes256(iv, mac, data.clone(), key.mac_key, key.key)?;
            Ok(dec)
        }
        _ => Err(CryptoError::InvalidKey.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::stretch_key;

    #[cfg(feature = "internal")]
    use {
        crate::{client::auth_settings::Kdf, crypto::stretch_key_password},
        std::num::NonZeroU32,
    };

    #[test]
    fn test_key_stretch() {
        let key = stretch_key(*b"&/$%F1a895g67HlX", "test_key", None);
        assert_eq!(key.to_base64(), "4PV6+PcmF2w7YHRatvyMcVQtI7zvCyssv/wFWmzjiH6Iv9altjmDkuBD1aagLVaLezbthbSe+ktR+U6qswxNnQ==");

        let key = stretch_key(*b"67t9b5g67$%Dh89n", "test_key", Some("test"));
        assert_eq!(key.to_base64(), "F9jVQmrACGx9VUPjuzfMYDjr726JtL300Y3Yg+VYUnVQtQ1s8oImJ5xtp1KALC9h2nav04++1LDW4iFD+infng==");
    }

    #[cfg(feature = "internal")]
    #[test]
    fn test_key_stretch_password_pbkdf2() {
        let (key, mac) = stretch_key_password(
            &b"67t9b5g67$%Dh89n"[..],
            "test_key".as_bytes(),
            &Kdf::PBKDF2 {
                iterations: NonZeroU32::new(10000).unwrap(),
            },
        )
        .unwrap();

        assert_eq!(
            key.as_slice(),
            [
                111, 31, 178, 45, 238, 152, 37, 114, 143, 215, 124, 83, 135, 173, 195, 23, 142,
                134, 120, 249, 61, 132, 163, 182, 113, 197, 189, 204, 188, 21, 237, 96
            ]
        );
        assert_eq!(
            mac.as_slice(),
            [
                221, 127, 206, 234, 101, 27, 202, 38, 86, 52, 34, 28, 78, 28, 185, 16, 48, 61, 127,
                166, 209, 247, 194, 87, 232, 26, 48, 85, 193, 249, 179, 155
            ]
        );
    }

    #[cfg(feature = "internal")]
    #[test]
    fn test_key_stretch_password_argon2() {
        let (key, mac) = stretch_key_password(
            &b"67t9b5g67$%Dh89n"[..],
            "test_key".as_bytes(),
            &Kdf::Argon2id {
                iterations: NonZeroU32::new(4).unwrap(),
                memory: NonZeroU32::new(32).unwrap(),
                parallelism: NonZeroU32::new(2).unwrap(),
            },
        )
        .unwrap();

        assert_eq!(
            key.as_slice(),
            [
                236, 253, 166, 121, 207, 124, 98, 149, 42, 141, 97, 226, 207, 71, 173, 60, 10, 0,
                184, 255, 252, 87, 62, 32, 188, 166, 173, 223, 146, 159, 222, 219
            ]
        );
        assert_eq!(
            mac.as_slice(),
            [
                214, 144, 76, 173, 225, 106, 132, 131, 173, 56, 134, 241, 223, 227, 165, 161, 146,
                37, 111, 206, 155, 24, 224, 151, 134, 189, 202, 0, 27, 149, 131, 21
            ]
        );
    }
}
