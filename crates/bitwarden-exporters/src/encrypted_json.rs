use base64::{engine::general_purpose::STANDARD, Engine};
use bitwarden_crypto::{generate_random_bytes, Kdf, KeyEncryptable, PinKey};
use serde::Serialize;
use thiserror::Error;
use uuid::Uuid;

use crate::{
    json::{self, export_json},
    Cipher, Folder,
};

#[derive(Error, Debug)]
pub enum EncryptedJsonError {
    #[error(transparent)]
    JsonExport(#[from] json::JsonError),

    #[error("JSON error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Cryptography error, {0}")]
    Crypto(#[from] bitwarden_crypto::CryptoError),
}

pub(crate) fn export_encrypted_json(
    folders: Vec<Folder>,
    ciphers: Vec<Cipher>,
    password: String,
    kdf: Kdf,
) -> Result<String, EncryptedJsonError> {
    let decrypted_export = export_json(folders, ciphers)?;

    let (kdf_type, kdf_iterations, kdf_memory, kdf_parallelism) = match kdf {
        Kdf::PBKDF2 { iterations } => (0, iterations.get(), None, None),
        Kdf::Argon2id {
            iterations,
            memory,
            parallelism,
        } => (
            1,
            iterations.get(),
            Some(memory.get()),
            Some(parallelism.get()),
        ),
    };

    let salt = generate_random_bytes::<[u8; 16]>();
    let salt = STANDARD.encode(salt);
    let key = PinKey::derive(password.as_bytes(), salt.as_bytes(), &kdf)?;

    let enc_key_validation = Uuid::new_v4().to_string();

    let encrypted_export = EncryptedJsonExport {
        encrypted: true,
        password_protected: true,
        salt,
        kdf_type,
        kdf_iterations,
        kdf_memory,
        kdf_parallelism,
        enc_key_validation: enc_key_validation.encrypt_with_key(&key)?.to_string(),
        data: decrypted_export.encrypt_with_key(&key)?.to_string(),
    };

    Ok(serde_json::to_string_pretty(&encrypted_export)?)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct EncryptedJsonExport {
    encrypted: bool,
    password_protected: bool,
    salt: String,
    kdf_type: u32,
    kdf_iterations: u32,
    kdf_memory: Option<u32>,
    kdf_parallelism: Option<u32>,
    #[serde(rename = "encKeyValidation_DO_NOT_EDIT")]
    enc_key_validation: String,
    data: String,
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU32;

    use super::*;
    use crate::{
        Card, Cipher, CipherType, Field, Identity, Login, LoginUri, SecureNote, SecureNoteType,
    };

    #[test]
    pub fn test_export() {
        let _export = export_encrypted_json(
            vec![Folder {
                id: "942e2984-1b9a-453b-b039-b107012713b9".parse().unwrap(),
                name: "Important".to_string(),
            }],
            vec![
                Cipher {
                    id: "25c8c414-b446-48e9-a1bd-b10700bbd740".parse().unwrap(),
                    folder_id: Some("942e2984-1b9a-453b-b039-b107012713b9".parse().unwrap()),

                    name: "Bitwarden".to_string(),
                    notes: Some("My note".to_string()),

                    r#type: CipherType::Login(Box::new(Login {
                        username: Some("test@bitwarden.com".to_string()),
                        password: Some("asdfasdfasdf".to_string()),
                        login_uris: vec![LoginUri {
                            uri: Some("https://vault.bitwarden.com".to_string()),
                            r#match: None,
                        }],
                        totp: Some("ABC".to_string()),
                    })),

                    favorite: true,
                    reprompt: 0,

                    fields: vec![
                        Field {
                            name: Some("Text".to_string()),
                            value: Some("A".to_string()),
                            r#type: 0,
                            linked_id: None,
                        },
                        Field {
                            name: Some("Hidden".to_string()),
                            value: Some("B".to_string()),
                            r#type: 1,
                            linked_id: None,
                        },
                        Field {
                            name: Some("Boolean (true)".to_string()),
                            value: Some("true".to_string()),
                            r#type: 2,
                            linked_id: None,
                        },
                        Field {
                            name: Some("Boolean (false)".to_string()),
                            value: Some("false".to_string()),
                            r#type: 2,
                            linked_id: None,
                        },
                        Field {
                            name: Some("Linked".to_string()),
                            value: None,
                            r#type: 3,
                            linked_id: Some(101),
                        },
                    ],

                    revision_date: "2024-01-30T14:09:33.753Z".parse().unwrap(),
                    creation_date: "2024-01-30T11:23:54.416Z".parse().unwrap(),
                    deleted_date: None,
                },
                Cipher {
                    id: "23f0f877-42b1-4820-a850-b10700bc41eb".parse().unwrap(),
                    folder_id: None,

                    name: "My secure note".to_string(),
                    notes: Some("Very secure!".to_string()),

                    r#type: CipherType::SecureNote(Box::new(SecureNote {
                        r#type: SecureNoteType::Generic,
                    })),

                    favorite: false,
                    reprompt: 0,

                    fields: vec![],

                    revision_date: "2024-01-30T11:25:25.466Z".parse().unwrap(),
                    creation_date: "2024-01-30T11:25:25.466Z".parse().unwrap(),
                    deleted_date: None,
                },
                Cipher {
                    id: "3ed8de45-48ee-4e26-a2dc-b10701276c53".parse().unwrap(),
                    folder_id: None,

                    name: "My card".to_string(),
                    notes: None,

                    r#type: CipherType::Card(Box::new(Card {
                        cardholder_name: Some("John Doe".to_string()),
                        exp_month: Some("1".to_string()),
                        exp_year: Some("2032".to_string()),
                        code: Some("123".to_string()),
                        brand: Some("Visa".to_string()),
                        number: Some("4111111111111111".to_string()),
                    })),

                    favorite: false,
                    reprompt: 0,

                    fields: vec![],

                    revision_date: "2024-01-30T17:55:36.150Z".parse().unwrap(),
                    creation_date: "2024-01-30T17:55:36.150Z".parse().unwrap(),
                    deleted_date: None,
                },
                Cipher {
                    id: "41cc3bc1-c3d9-4637-876c-b10701273712".parse().unwrap(),
                    folder_id: Some("942e2984-1b9a-453b-b039-b107012713b9".parse().unwrap()),

                    name: "My identity".to_string(),
                    notes: None,

                    r#type: CipherType::Identity(Box::new(Identity {
                        title: Some("Mr".to_string()),
                        first_name: Some("John".to_string()),
                        middle_name: None,
                        last_name: Some("Doe".to_string()),
                        address1: None,
                        address2: None,
                        address3: None,
                        city: None,
                        state: None,
                        postal_code: None,
                        country: None,
                        company: Some("Bitwarden".to_string()),
                        email: None,
                        phone: None,
                        ssn: None,
                        username: Some("JDoe".to_string()),
                        passport_number: None,
                        license_number: None,
                    })),

                    favorite: false,
                    reprompt: 0,

                    fields: vec![],

                    revision_date: "2024-01-30T17:54:50.706Z".parse().unwrap(),
                    creation_date: "2024-01-30T17:54:50.706Z".parse().unwrap(),
                    deleted_date: None,
                },
            ],
            "password".to_string(),
            Kdf::PBKDF2 {
                iterations: NonZeroU32::new(600_000).unwrap(),
            },
        )
        .unwrap();
    }
}
