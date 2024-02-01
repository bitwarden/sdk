use std::{num::NonZeroU32, pin::Pin};

use aes::cipher::typenum::U32;
use base64::{engine::general_purpose::STANDARD, Engine};
use generic_array::GenericArray;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sha2::Digest;

use crate::{
    util::{self, hkdf_expand},
    EncString, KeyDecryptable, Result, SymmetricCryptoKey, UserKey,
};

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Enum))]
pub enum Kdf {
    PBKDF2 {
        iterations: NonZeroU32,
    },
    Argon2id {
        iterations: NonZeroU32,
        memory: NonZeroU32,
        parallelism: NonZeroU32,
    },
}

#[derive(Copy, Clone, JsonSchema)]
#[cfg_attr(feature = "mobile", derive(uniffi::Enum))]
pub enum HashPurpose {
    ServerAuthorization = 1,
    LocalAuthorization = 2,
}

/// Master Key.
///
/// Derived from the users master password, used to protect the [UserKey].
pub struct MasterKey(SymmetricCryptoKey);

impl MasterKey {
    pub fn new(key: SymmetricCryptoKey) -> MasterKey {
        Self(key)
    }

    /// Derives a users master key from their password, email and KDF.
    pub fn derive(password: &[u8], email: &[u8], kdf: &Kdf) -> Result<Self> {
        derive_key(password, email, kdf).map(Self)
    }

    /// Derive the master key hash, used for local and remote password validation.
    pub fn derive_master_key_hash(&self, password: &[u8], purpose: HashPurpose) -> Result<String> {
        let hash = util::pbkdf2(&self.0.key, password, purpose as u32);

        Ok(STANDARD.encode(hash))
    }

    /// Generate a new random user key and encrypt it with the master key.
    pub fn make_user_key(&self) -> Result<(UserKey, EncString)> {
        make_user_key(rand::thread_rng(), self)
    }

    /// Decrypt the users user key
    pub fn decrypt_user_key(&self, user_key: EncString) -> Result<SymmetricCryptoKey> {
        let stretched_key = stretch_master_key(self)?;

        let mut dec: Vec<u8> = user_key.decrypt_with_key(&stretched_key)?;
        SymmetricCryptoKey::try_from(dec.as_mut_slice())
    }

    pub fn encrypt_user_key(&self, user_key: &SymmetricCryptoKey) -> Result<EncString> {
        let stretched_key = stretch_master_key(self)?;

        EncString::encrypt_aes256_hmac(
            user_key.to_vec().as_slice(),
            stretched_key.mac_key.as_ref().unwrap(),
            &stretched_key.key,
        )
    }
}

/// Generate a new random user key and encrypt it with the master key.
fn make_user_key(
    mut rng: impl rand::RngCore,
    master_key: &MasterKey,
) -> Result<(UserKey, EncString)> {
    let user_key = SymmetricCryptoKey::generate(&mut rng);
    let protected = master_key.encrypt_user_key(&user_key)?;
    Ok((UserKey::new(user_key), protected))
}

/// Derive a generic key from a secret and salt using the provided KDF.
fn derive_key(secret: &[u8], salt: &[u8], kdf: &Kdf) -> Result<SymmetricCryptoKey> {
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
    SymmetricCryptoKey::try_from(hash.as_mut_slice())
}

fn stretch_master_key(master_key: &MasterKey) -> Result<SymmetricCryptoKey> {
    let key: Pin<Box<GenericArray<u8, U32>>> = hkdf_expand(&master_key.0.key, Some("enc"))?;
    let mac_key: Pin<Box<GenericArray<u8, U32>>> = hkdf_expand(&master_key.0.key, Some("mac"))?;
    Ok(SymmetricCryptoKey::new(key, Some(mac_key)))
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU32;

    use rand::SeedableRng;

    use super::{make_user_key, stretch_master_key, HashPurpose, Kdf, MasterKey};
    use crate::{keys::symmetric_crypto_key::derive_symmetric_key, SymmetricCryptoKey};

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
        let master_key = MasterKey(SymmetricCryptoKey::new(
            Box::pin(
                [
                    31, 79, 104, 226, 150, 71, 177, 90, 194, 80, 172, 209, 17, 129, 132, 81, 138,
                    167, 69, 167, 254, 149, 2, 27, 39, 197, 64, 42, 22, 195, 86, 75,
                ]
                .into(),
            ),
            None,
        ));

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
            stretched.mac_key.as_ref().unwrap().as_slice()
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

        let master_key = MasterKey(SymmetricCryptoKey::new(
            Box::pin(
                [
                    31, 79, 104, 226, 150, 71, 177, 90, 194, 80, 172, 209, 17, 129, 132, 81, 138,
                    167, 69, 167, 254, 149, 2, 27, 39, 197, 64, 42, 22, 195, 86, 75,
                ]
                .into(),
            ),
            None,
        ));

        let (user_key, protected) = make_user_key(&mut rng, &master_key).unwrap();

        assert_eq!(
            user_key.0.key.as_slice(),
            [
                62, 0, 239, 47, 137, 95, 64, 214, 127, 91, 184, 232, 31, 9, 165, 161, 44, 132, 14,
                195, 206, 154, 127, 59, 24, 27, 225, 136, 239, 113, 26, 30
            ]
        );
        assert_eq!(
            user_key.0.mac_key.as_ref().unwrap().as_slice(),
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

    #[test]
    fn test_make_user_key2() {
        let master_key = MasterKey(derive_symmetric_key("test1"));

        let user_key = derive_symmetric_key("test2");

        let encrypted = master_key.encrypt_user_key(&user_key).unwrap();
        let decrypted = master_key.decrypt_user_key(encrypted).unwrap();

        assert_eq!(
            decrypted.key, user_key.key,
            "Decrypted key doesn't match user key"
        );
        assert_eq!(
            decrypted.mac_key, user_key.mac_key,
            "Decrypted key doesn't match user key"
        );
    }
}
