//! Cryptographic primitives used in the SDK

#[cfg(feature = "internal")]
use aes::cipher::typenum::U32;
use aes::cipher::Unsigned;
use hmac::digest::OutputSizeUser;
#[cfg(any(feature = "internal", feature = "mobile"))]
use {
    crate::{client::auth_settings::Kdf, error::Result},
    aes::cipher::generic_array::GenericArray,
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
mod shareable_key;
pub(crate) use shareable_key::stretch_key;

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

#[cfg(test)]
mod tests {
    #[cfg(feature = "internal")]
    use {
        crate::{client::auth_settings::Kdf, crypto::stretch_key_password},
        std::num::NonZeroU32,
    };

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
