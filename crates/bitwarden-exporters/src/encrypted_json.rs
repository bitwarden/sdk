use base64::engine::general_purpose::STANDARD;
use bitwarden_crypto::{
    generate_random_bytes, Kdf, KeyEncryptable, PinKey, Sensitive, SensitiveVec,
};
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

    let salt: Sensitive<[u8; 16]> = generate_random_bytes();
    let salt = SensitiveVec::from(salt).encode_base64(STANDARD);
    let key = PinKey::derive(password.as_bytes(), salt.expose().as_bytes(), &kdf)?;

    let enc_key_validation = Uuid::new_v4().to_string();

    let encrypted_export = EncryptedJsonExport {
        encrypted: true,
        password_protected: true,
        salt: salt.expose().to_string(),
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

    use bitwarden_crypto::DecryptedString;

    use super::*;
    use crate::{
        Card, Cipher, CipherType, Field, Identity, Login, LoginUri, SecureNote, SecureNoteType,
    };

    #[test]
    pub fn test_export() {
        let _export = export_encrypted_json(
            vec![Folder {
                id: "942e2984-1b9a-453b-b039-b107012713b9".parse().unwrap(),
                name: DecryptedString::test("Important"),
            }],
            vec![
                Cipher {
                    id: "25c8c414-b446-48e9-a1bd-b10700bbd740".parse().unwrap(),
                    folder_id: Some("942e2984-1b9a-453b-b039-b107012713b9".parse().unwrap()),

                    name: DecryptedString::test("Bitwarden"),
                    notes: Some(DecryptedString::test("My note")),

                    r#type: CipherType::Login(Box::new(Login {
                        username: Some(DecryptedString::test("test@bitwarden.com")),
                        password: Some(DecryptedString::test("asdfasdfasdf")),
                        login_uris: vec![LoginUri {
                            uri: Some(DecryptedString::test("https://vault.bitwarden.com")),
                            r#match: None,
                        }],
                        totp: Some(DecryptedString::test("ABC")),
                    })),

                    favorite: true,
                    reprompt: 0,

                    fields: vec![
                        Field {
                            name: Some(DecryptedString::test("Text")),
                            value: Some(DecryptedString::test("A")),
                            r#type: 0,
                            linked_id: None,
                        },
                        Field {
                            name: Some(DecryptedString::test("Hidden")),
                            value: Some(DecryptedString::test("B")),
                            r#type: 1,
                            linked_id: None,
                        },
                        Field {
                            name: Some(DecryptedString::test("Boolean (true)")),
                            value: Some(DecryptedString::test("true")),
                            r#type: 2,
                            linked_id: None,
                        },
                        Field {
                            name: Some(DecryptedString::test("Boolean (false)")),
                            value: Some(DecryptedString::test("false")),
                            r#type: 2,
                            linked_id: None,
                        },
                        Field {
                            name: Some(DecryptedString::test("Linked")),
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

                    name: DecryptedString::test("My secure note"),
                    notes: Some(DecryptedString::test("Very secure!")),

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

                    name: DecryptedString::test("My card"),
                    notes: None,

                    r#type: CipherType::Card(Box::new(Card {
                        cardholder_name: Some(DecryptedString::test("John Doe")),
                        exp_month: Some(DecryptedString::test("1")),
                        exp_year: Some(DecryptedString::test("2032")),
                        code: Some(DecryptedString::test("123")),
                        brand: Some(DecryptedString::test("Visa")),
                        number: Some(DecryptedString::test("4111111111111111")),
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

                    name: DecryptedString::test("My identity"),
                    notes: None,

                    r#type: CipherType::Identity(Box::new(Identity {
                        title: Some(DecryptedString::test("Mr")),
                        first_name: Some(DecryptedString::test("John")),
                        middle_name: None,
                        last_name: Some(DecryptedString::test("Doe")),
                        address1: None,
                        address2: None,
                        address3: None,
                        city: None,
                        state: None,
                        postal_code: None,
                        country: None,
                        company: Some(DecryptedString::test("Bitwarden")),
                        email: None,
                        phone: None,
                        ssn: None,
                        username: Some(DecryptedString::test("JDoe")),
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
