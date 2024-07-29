use std::num::NonZeroU32;

use base64::{engine::general_purpose::STANDARD, Engine};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::utils::{derive_kdf_key, stretch_kdf_key};
use crate::{util, CryptoError, EncString, KeyDecryptable, Result, SymmetricCryptoKey, UserKey};

/// Key Derivation Function for Bitwarden Account
///
/// In Bitwarden accounts can use multiple KDFs to derive their master key from their password. This
/// Enum represents all the possible KDFs.
#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
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

impl Default for Kdf {
    /// Default KDF for new accounts.
    fn default() -> Self {
        Kdf::PBKDF2 {
            iterations: default_pbkdf2_iterations(),
        }
    }
}

/// Default PBKDF2 iterations
pub fn default_pbkdf2_iterations() -> NonZeroU32 {
    NonZeroU32::new(600_000).expect("Non-zero number")
}
/// Default Argon2 iterations
pub fn default_argon2_iterations() -> NonZeroU32 {
    NonZeroU32::new(3).expect("Non-zero number")
}
/// Default Argon2 memory
pub fn default_argon2_memory() -> NonZeroU32 {
    NonZeroU32::new(64).expect("Non-zero number")
}
/// Default Argon2 parallelism
pub fn default_argon2_parallelism() -> NonZeroU32 {
    NonZeroU32::new(4).expect("Non-zero number")
}

#[derive(Copy, Clone, JsonSchema)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
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
    ///
    /// Note: the email is trimmed and converted to lowercase before being used.
    pub fn derive(password: &str, email: &str, kdf: &Kdf) -> Result<Self> {
        derive_kdf_key(
            password.as_bytes(),
            email.trim().to_lowercase().as_bytes(),
            kdf,
        )
        .map(Self)
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

    /// Encrypt the users user key
    pub fn encrypt_user_key(&self, user_key: &SymmetricCryptoKey) -> Result<EncString> {
        encrypt_user_key(&self.0, user_key)
    }

    /// Decrypt the users user key
    pub fn decrypt_user_key(&self, user_key: EncString) -> Result<SymmetricCryptoKey> {
        decrypt_user_key(&self.0, user_key)
    }
}

/// Helper function to encrypt a user key with a master or pin key.
pub(super) fn encrypt_user_key(
    key: &SymmetricCryptoKey,
    user_key: &SymmetricCryptoKey,
) -> Result<EncString> {
    let stretched_key = stretch_kdf_key(key)?;

    EncString::encrypt_aes256_hmac(
        &user_key.to_vec(),
        stretched_key
            .mac_key
            .as_ref()
            .ok_or(CryptoError::InvalidMac)?,
        &stretched_key.key,
    )
}

/// Helper function to decrypt a user key with a master or pin key.
pub(super) fn decrypt_user_key(
    key: &SymmetricCryptoKey,
    user_key: EncString,
) -> Result<SymmetricCryptoKey> {
    let mut dec: Vec<u8> = match user_key {
        // Legacy. user_keys were encrypted using `AesCbc256_B64` a long time ago. We've since
        // moved to using `AesCbc256_HmacSha256_B64`. However, we still need to support
        // decrypting these old keys.
        EncString::AesCbc256_B64 { .. } => user_key.decrypt_with_key(key)?,
        _ => {
            let stretched_key = stretch_kdf_key(key)?;
            user_key.decrypt_with_key(&stretched_key)?
        }
    };

    SymmetricCryptoKey::try_from(dec.as_mut_slice())
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

#[cfg(test)]
mod tests {
    use std::num::NonZeroU32;

    use rand::SeedableRng;

    use super::{make_user_key, HashPurpose, Kdf, MasterKey};
    use crate::{keys::symmetric_crypto_key::derive_symmetric_key, EncString, SymmetricCryptoKey};

    #[test]
    fn test_master_key_derive_pbkdf2() {
        let master_key = MasterKey::derive(
            "67t9b5g67$%Dh89n",
            "test_key",
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
            "67t9b5g67$%Dh89n",
            "test_key",
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
    fn test_password_hash_pbkdf2() {
        let password = "asdfasdf";
        let salts = [
            "test@bitwarden.com",
            "TEST@bitwarden.com",
            " test@bitwarden.com",
        ];
        let kdf = Kdf::PBKDF2 {
            iterations: NonZeroU32::new(100_000).unwrap(),
        };

        for salt in salts.iter() {
            let master_key = MasterKey::derive(password, salt, &kdf).unwrap();

            assert_eq!(
                "wmyadRMyBZOH7P/a/ucTCbSghKgdzDpPqUnu/DAVtSw=",
                master_key
                    .derive_master_key_hash(password.as_bytes(), HashPurpose::ServerAuthorization)
                    .unwrap(),
            );
        }
    }

    #[test]
    fn test_password_hash_argon2id() {
        let password = "asdfasdf";
        let salt = "test_salt";
        let kdf = Kdf::Argon2id {
            iterations: NonZeroU32::new(4).unwrap(),
            memory: NonZeroU32::new(32).unwrap(),
            parallelism: NonZeroU32::new(2).unwrap(),
        };

        let master_key = MasterKey::derive(password, salt, &kdf).unwrap();

        assert_eq!(
            "PR6UjYmjmppTYcdyTiNbAhPJuQQOmynKbdEl1oyi/iQ=",
            master_key
                .derive_master_key_hash(password.as_bytes(), HashPurpose::ServerAuthorization)
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

    #[test]
    fn test_decrypt_user_key_aes_cbc256_b64() {
        let password = "asdfasdfasdf";
        let salt = "legacy@bitwarden.com";
        let kdf = Kdf::PBKDF2 {
            iterations: NonZeroU32::new(600_000).unwrap(),
        };

        let master_key = MasterKey::derive(password, salt, &kdf).unwrap();

        let user_key: EncString = "0.8UClLa8IPE1iZT7chy5wzQ==|6PVfHnVk5S3XqEtQemnM5yb4JodxmPkkWzmDRdfyHtjORmvxqlLX40tBJZ+CKxQWmS8tpEB5w39rbgHg/gqs0haGdZG4cPbywsgGzxZ7uNI=".parse().unwrap();

        let decrypted = master_key.decrypt_user_key(user_key).unwrap();

        assert_eq!(
            decrypted.key.as_slice(),
            [
                12, 95, 151, 203, 37, 4, 236, 67, 137, 97, 90, 58, 6, 127, 242, 28, 209, 168, 125,
                29, 118, 24, 213, 44, 117, 202, 2, 115, 132, 165, 125, 148
            ]
        );
        assert_eq!(
            decrypted.mac_key.as_ref().unwrap().as_slice(),
            [
                186, 215, 234, 137, 24, 169, 227, 29, 218, 57, 180, 237, 73, 91, 189, 51, 253, 26,
                17, 52, 226, 4, 134, 75, 194, 208, 178, 133, 128, 224, 140, 167
            ]
        );
    }
}
