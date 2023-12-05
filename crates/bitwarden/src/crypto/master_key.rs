use aes::cipher::{generic_array::GenericArray, typenum::U32};
use base64::Engine;
use rand::Rng;
use sha2::Digest;

use super::{
    hkdf_expand, EncString, KeyDecryptable, PbkdfSha256Hmac, SymmetricCryptoKey, UserKey,
    PBKDF_SHA256_HMAC_OUT_SIZE,
};
use crate::{client::kdf::Kdf, error::Result, util::BASE64_ENGINE};

#[derive(Copy, Clone)]
#[cfg_attr(feature = "mobile", derive(uniffi::Enum))]
pub enum HashPurpose {
    ServerAuthorization = 1,
    LocalAuthorization = 2,
}

/// A Master Key.
pub(crate) struct MasterKey(SymmetricCryptoKey);

impl MasterKey {
    /// Derives a users master key from their password, email and KDF.
    pub fn derive(password: &[u8], email: &[u8], kdf: &Kdf) -> Result<Self> {
        derive_key(password, email, kdf).map(Self)
    }

    /// Derive the master key hash, used for server authorization.
    pub(crate) fn derive_master_key_hash(
        &self,
        password: &[u8],
        purpose: HashPurpose,
    ) -> Result<String> {
        let hash = pbkdf2::pbkdf2_array::<PbkdfSha256Hmac, PBKDF_SHA256_HMAC_OUT_SIZE>(
            &self.0.key,
            password,
            purpose as u32,
        )
        .expect("hash is a valid fixed size");

        Ok(BASE64_ENGINE.encode(hash))
    }

    pub(crate) fn make_user_key(&self) -> Result<(UserKey, EncString)> {
        make_user_key(rand::thread_rng(), self)
    }

    pub(crate) fn decrypt_user_key(&self, user_key: EncString) -> Result<SymmetricCryptoKey> {
        let stretched_key = stretch_master_key(self)?;

        let dec: Vec<u8> = user_key.decrypt_with_key(&stretched_key)?;
        SymmetricCryptoKey::try_from(dec.as_slice())
    }
}

/// Generate a new random user key and encrypt it with the master key.
fn make_user_key(
    mut rng: impl rand::RngCore,
    master_key: &MasterKey,
) -> Result<(UserKey, EncString)> {
    let mut user_key = [0u8; 64];
    rng.fill(&mut user_key);

    let stretched_key = stretch_master_key(master_key)?;
    let protected = EncString::encrypt_aes256_hmac(
        &user_key,
        stretched_key.mac_key.unwrap(),
        stretched_key.key,
    )?;

    let u: &[u8] = &user_key;
    Ok((UserKey::new(SymmetricCryptoKey::try_from(u)?), protected))
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

fn stretch_master_key(master_key: &MasterKey) -> Result<SymmetricCryptoKey> {
    let key: GenericArray<u8, U32> = hkdf_expand(&master_key.0.key, Some("enc"))?;
    let mac_key: GenericArray<u8, U32> = hkdf_expand(&master_key.0.key, Some("mac"))?;

    Ok(SymmetricCryptoKey {
        key,
        mac_key: Some(mac_key),
    })
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU32;

    use rand::SeedableRng;

    use super::{make_user_key, stretch_master_key, HashPurpose, MasterKey};
    use crate::{client::kdf::Kdf, crypto::SymmetricCryptoKey};

    #[test]
    fn test_master_key_derive_pbkdf2() {
        let master_key = MasterKey::derive(
            &b"67t9b5g67$%Dh89n"[..],
            "test_key".as_bytes(),
            &Kdf::PBKDF2 {
                iterations: NonZeroU32::new(10000).unwrap(),
            },
        )
        .unwrap();

        assert_eq!(
            [
                31, 79, 104, 226, 150, 71, 177, 90, 194, 80, 172, 209, 17, 129, 132, 81, 138, 167,
                69, 167, 254, 149, 2, 27, 39, 197, 64, 42, 22, 195, 86, 75
            ],
            master_key.0.key.as_slice()
        );
        assert_eq!(None, master_key.0.mac_key);
    }

    #[test]
    fn test_master_key_derive_argon2() {
        let master_key = MasterKey::derive(
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
            [
                207, 240, 225, 177, 162, 19, 163, 76, 98, 106, 179, 175, 224, 9, 17, 240, 20, 147,
                237, 47, 246, 150, 141, 184, 62, 225, 131, 242, 51, 53, 225, 242
            ],
            master_key.0.key.as_slice()
        );
        assert_eq!(None, master_key.0.mac_key);
    }

    #[test]
    fn test_stretch_master_key() {
        let master_key = MasterKey(SymmetricCryptoKey {
            key: [
                31, 79, 104, 226, 150, 71, 177, 90, 194, 80, 172, 209, 17, 129, 132, 81, 138, 167,
                69, 167, 254, 149, 2, 27, 39, 197, 64, 42, 22, 195, 86, 75,
            ]
            .into(),
            mac_key: None,
        });

        let stretched = stretch_master_key(&master_key).unwrap();

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

    #[test]
    fn test_password_hash_pbkdf2() {
        let password = "asdfasdf".as_bytes();
        let salt = "test_salt".as_bytes();
        let kdf = Kdf::PBKDF2 {
            iterations: NonZeroU32::new(100_000).unwrap(),
        };

        let master_key = MasterKey::derive(password, salt, &kdf).unwrap();

        assert_eq!(
            "ZF6HjxUTSyBHsC+HXSOhZoXN+UuMnygV5YkWXCY4VmM=",
            master_key
                .derive_master_key_hash(password, HashPurpose::ServerAuthorization)
                .unwrap(),
        );
    }

    #[test]
    fn test_password_hash_argon2id() {
        let password = "asdfasdf".as_bytes();
        let salt = "test_salt".as_bytes();
        let kdf = Kdf::Argon2id {
            iterations: NonZeroU32::new(4).unwrap(),
            memory: NonZeroU32::new(32).unwrap(),
            parallelism: NonZeroU32::new(2).unwrap(),
        };

        let master_key = MasterKey::derive(password, salt, &kdf).unwrap();

        assert_eq!(
            "PR6UjYmjmppTYcdyTiNbAhPJuQQOmynKbdEl1oyi/iQ=",
            master_key
                .derive_master_key_hash(password, HashPurpose::ServerAuthorization)
                .unwrap(),
        );
    }

    #[test]
    fn test_make_user_key() {
        let mut rng = rand_chacha::ChaCha8Rng::from_seed([0u8; 32]);

        let master_key = MasterKey(SymmetricCryptoKey {
            key: [
                31, 79, 104, 226, 150, 71, 177, 90, 194, 80, 172, 209, 17, 129, 132, 81, 138, 167,
                69, 167, 254, 149, 2, 27, 39, 197, 64, 42, 22, 195, 86, 75,
            ]
            .into(),
            mac_key: None,
        });

        let (user_key, protected) = make_user_key(&mut rng, &master_key).unwrap();

        assert_eq!(
            user_key.0.key.as_slice(),
            [
                62, 0, 239, 47, 137, 95, 64, 214, 127, 91, 184, 232, 31, 9, 165, 161, 44, 132, 14,
                195, 206, 154, 127, 59, 24, 27, 225, 136, 239, 113, 26, 30
            ]
        );
        assert_eq!(
            user_key.0.mac_key.unwrap().as_slice(),
            [
                152, 76, 225, 114, 185, 33, 111, 65, 159, 68, 83, 103, 69, 109, 86, 25, 49, 74, 66,
                163, 218, 134, 176, 1, 56, 123, 253, 184, 14, 12, 254, 66
            ]
        );

        // Ensure we can decrypt the key and get back the same key
        let decrypted = master_key.decrypt_user_key(protected).unwrap();

        assert_eq!(
            decrypted.key, user_key.0.key,
            "Decrypted key doesn't match user key"
        );
        assert_eq!(
            decrypted.mac_key, user_key.0.mac_key,
            "Decrypted key doesn't match user key"
        );
    }
}
