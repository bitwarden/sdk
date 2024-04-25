use bitwarden_crypto::DecryptedString;
use chrono::{DateTime, Utc};
use thiserror::Error;
use uuid::Uuid;

use crate::{Card, Cipher, CipherType, Field, Folder, Identity, Login, LoginUri, SecureNote};

#[derive(Error, Debug)]
pub enum JsonError {
    #[error("JSON error: {0}")]
    Serde(#[from] serde_json::Error),
}

pub(crate) fn export_json(folders: Vec<Folder>, ciphers: Vec<Cipher>) -> Result<String, JsonError> {
    let export = JsonExport {
        encrypted: false,
        folders: folders.into_iter().map(|f| f.into()).collect(),
        items: ciphers.into_iter().map(|c| c.into()).collect(),
    };

    Ok(serde_json::to_string_pretty(&export)?)
}

/// JSON export format. These are intentionally decoupled from the internal data structures to
/// ensure internal changes are not reflected in the public exports.
///
/// Be careful about changing these structs to maintain compatibility with old exporters/importers.
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonExport {
    encrypted: bool,
    folders: Vec<JsonFolder>,
    items: Vec<JsonCipher>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonFolder {
    id: Uuid,
    name: DecryptedString,
}

impl From<Folder> for JsonFolder {
    fn from(folder: Folder) -> Self {
        JsonFolder {
            id: folder.id,
            name: folder.name,
        }
    }
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonCipher {
    id: Uuid,
    folder_id: Option<Uuid>,
    // Organizational IDs which are always empty in personal exports
    organization_id: Option<Uuid>,
    collection_ids: Option<Vec<Uuid>>,

    name: DecryptedString,
    notes: Option<DecryptedString>,

    r#type: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    login: Option<JsonLogin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity: Option<JsonIdentity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    card: Option<JsonCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secure_note: Option<JsonSecureNote>,

    favorite: bool,
    reprompt: u8,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    fields: Vec<JsonField>,
    password_history: Option<Vec<DecryptedString>>,

    revision_date: DateTime<Utc>,
    creation_date: DateTime<Utc>,
    deleted_date: Option<DateTime<Utc>>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonLogin {
    username: Option<DecryptedString>,
    password: Option<DecryptedString>,
    uris: Vec<JsonLoginUri>,
    totp: Option<DecryptedString>,
    fido2_credentials: Vec<DecryptedString>,
}

impl From<Login> for JsonLogin {
    fn from(login: Login) -> Self {
        JsonLogin {
            username: login.username,
            password: login.password,
            uris: login.login_uris.into_iter().map(|u| u.into()).collect(),
            totp: login.totp,
            fido2_credentials: vec![],
        }
    }
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonLoginUri {
    uri: Option<DecryptedString>,
    r#match: Option<u8>,
}

impl From<LoginUri> for JsonLoginUri {
    fn from(login_uri: LoginUri) -> Self {
        JsonLoginUri {
            uri: login_uri.uri,
            r#match: login_uri.r#match,
        }
    }
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonSecureNote {
    r#type: u8,
}

impl From<SecureNote> for JsonSecureNote {
    fn from(note: SecureNote) -> Self {
        JsonSecureNote {
            r#type: note.r#type as u8,
        }
    }
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonCard {
    cardholder_name: Option<DecryptedString>,
    exp_month: Option<DecryptedString>,
    exp_year: Option<DecryptedString>,
    code: Option<DecryptedString>,
    brand: Option<DecryptedString>,
    number: Option<DecryptedString>,
}

impl From<Card> for JsonCard {
    fn from(card: Card) -> Self {
        JsonCard {
            cardholder_name: card.cardholder_name,
            exp_month: card.exp_month,
            exp_year: card.exp_year,
            code: card.code,
            brand: card.brand,
            number: card.number,
        }
    }
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonIdentity {
    title: Option<DecryptedString>,
    first_name: Option<DecryptedString>,
    middle_name: Option<DecryptedString>,
    last_name: Option<DecryptedString>,
    address1: Option<DecryptedString>,
    address2: Option<DecryptedString>,
    address3: Option<DecryptedString>,
    city: Option<DecryptedString>,
    state: Option<DecryptedString>,
    postal_code: Option<DecryptedString>,
    country: Option<DecryptedString>,
    company: Option<DecryptedString>,
    email: Option<DecryptedString>,
    phone: Option<DecryptedString>,
    ssn: Option<DecryptedString>,
    username: Option<DecryptedString>,
    passport_number: Option<DecryptedString>,
    license_number: Option<DecryptedString>,
}

impl From<Identity> for JsonIdentity {
    fn from(identity: Identity) -> Self {
        JsonIdentity {
            title: identity.title,
            first_name: identity.first_name,
            middle_name: identity.middle_name,
            last_name: identity.last_name,
            address1: identity.address1,
            address2: identity.address2,
            address3: identity.address3,
            city: identity.city,
            state: identity.state,
            postal_code: identity.postal_code,
            country: identity.country,
            company: identity.company,
            email: identity.email,
            phone: identity.phone,
            ssn: identity.ssn,
            username: identity.username,
            passport_number: identity.passport_number,
            license_number: identity.license_number,
        }
    }
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonField {
    name: Option<DecryptedString>,
    value: Option<DecryptedString>,
    r#type: u8,
    linked_id: Option<u32>,
}

impl From<Field> for JsonField {
    fn from(field: Field) -> Self {
        JsonField {
            name: field.name,
            value: field.value,
            r#type: field.r#type,
            linked_id: field.linked_id,
        }
    }
}

impl From<Cipher> for JsonCipher {
    fn from(cipher: Cipher) -> Self {
        let r#type = match cipher.r#type {
            CipherType::Login(_) => 1,
            CipherType::SecureNote(_) => 2,
            CipherType::Card(_) => 3,
            CipherType::Identity(_) => 4,
        };

        let (login, secure_note, card, identity) = match cipher.r#type {
            CipherType::Login(l) => (Some((*l).into()), None, None, None),
            CipherType::SecureNote(s) => (None, Some((*s).into()), None, None),
            CipherType::Card(c) => (None, None, Some((*c).into()), None),
            CipherType::Identity(i) => (None, None, None, Some((*i).into())),
        };

        JsonCipher {
            id: cipher.id,
            folder_id: cipher.folder_id,
            organization_id: None,
            collection_ids: None,
            name: cipher.name,
            notes: cipher.notes,
            r#type,
            login,
            identity,
            card,
            secure_note,
            favorite: cipher.favorite,
            reprompt: cipher.reprompt,
            fields: cipher.fields.into_iter().map(|f| f.into()).collect(),
            password_history: None,
            revision_date: cipher.revision_date,
            creation_date: cipher.creation_date,
            deleted_date: cipher.deleted_date,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, io::Read, path::PathBuf};

    use super::*;
    use crate::{Cipher, Field, LoginUri, SecureNoteType};

    #[test]
    fn test_convert_login() {
        let cipher = Cipher {
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
        };

        let json = serde_json::to_string(&JsonCipher::from(cipher)).unwrap();

        let expected = r#"{
            "passwordHistory": null,
            "revisionDate": "2024-01-30T14:09:33.753Z",
            "creationDate": "2024-01-30T11:23:54.416Z",
            "deletedDate": null,
            "id": "25c8c414-b446-48e9-a1bd-b10700bbd740",
            "organizationId": null,
            "folderId": "942e2984-1b9a-453b-b039-b107012713b9",
            "type": 1,
            "reprompt": 0,
            "name": "Bitwarden",
            "notes": "My note",
            "favorite": true,
            "fields": [
              {
                "name": "Text",
                "value": "A",
                "type": 0,
                "linkedId": null
              },
              {
                "name": "Hidden",
                "value": "B",
                "type": 1,
                "linkedId": null
              },
              {
                "name": "Boolean (true)",
                "value": "true",
                "type": 2,
                "linkedId": null
              },
              {
                "name": "Boolean (false)",
                "value": "false",
                "type": 2,
                "linkedId": null
              },
              {
                "name": "Linked",
                "value": null,
                "type": 3,
                "linkedId": 101
              }
            ],
            "login": {
              "fido2Credentials": [],
              "uris": [
                {
                  "match": null,
                  "uri": "https://vault.bitwarden.com"
                }
              ],
              "username": "test@bitwarden.com",
              "password": "asdfasdfasdf",
              "totp": "ABC"
            },
            "collectionIds": null
          }"#;

        assert_eq!(
            json.parse::<serde_json::Value>().unwrap(),
            expected.parse::<serde_json::Value>().unwrap()
        )
    }

    #[test]
    fn test_convert_secure_note() {
        let cipher = Cipher {
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
        };

        let json = serde_json::to_string(&JsonCipher::from(cipher)).unwrap();

        let expected = r#"{
            "passwordHistory": null,
            "revisionDate": "2024-01-30T11:25:25.466Z",
            "creationDate": "2024-01-30T11:25:25.466Z",
            "deletedDate": null,
            "id": "23f0f877-42b1-4820-a850-b10700bc41eb",
            "organizationId": null,
            "folderId": null,
            "type": 2,
            "reprompt": 0,
            "name": "My secure note",
            "notes": "Very secure!",
            "favorite": false,
            "secureNote": {
              "type": 0
            },
            "collectionIds": null
        }"#;

        assert_eq!(
            json.parse::<serde_json::Value>().unwrap(),
            expected.parse::<serde_json::Value>().unwrap()
        )
    }

    #[test]
    fn test_convert_card() {
        let cipher = Cipher {
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
        };

        let json = serde_json::to_string(&JsonCipher::from(cipher)).unwrap();

        let expected = r#"{
            "passwordHistory": null,
            "revisionDate": "2024-01-30T17:55:36.150Z",
            "creationDate": "2024-01-30T17:55:36.150Z",
            "deletedDate": null,
            "id": "3ed8de45-48ee-4e26-a2dc-b10701276c53",
            "organizationId": null,
            "folderId": null,
            "type": 3,
            "reprompt": 0,
            "name": "My card",
            "notes": null,
            "favorite": false,
            "card": {
                "cardholderName": "John Doe",
                "brand": "Visa",
                "number": "4111111111111111",
                "expMonth": "1",
                "expYear": "2032",
                "code": "123"
            },
            "collectionIds": null
        }"#;

        assert_eq!(
            json.parse::<serde_json::Value>().unwrap(),
            expected.parse::<serde_json::Value>().unwrap()
        )
    }

    #[test]
    fn test_convert_identity() {
        let cipher = Cipher {
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
        };

        let json = serde_json::to_string(&JsonCipher::from(cipher)).unwrap();

        let expected = r#"{
            "passwordHistory": null,
            "revisionDate": "2024-01-30T17:54:50.706Z",
            "creationDate": "2024-01-30T17:54:50.706Z",
            "deletedDate": null,
            "id": "41cc3bc1-c3d9-4637-876c-b10701273712",
            "organizationId": null,
            "folderId": "942e2984-1b9a-453b-b039-b107012713b9",
            "type": 4,
            "reprompt": 0,
            "name": "My identity",
            "notes": null,
            "favorite": false,
            "identity": {
                "title": "Mr",
                "firstName": "John",
                "middleName": null,
                "lastName": "Doe",
                "address1": null,
                "address2": null,
                "address3": null,
                "city": null,
                "state": null,
                "postalCode": null,
                "country": null,
                "company": "Bitwarden",
                "email": null,
                "phone": null,
                "ssn": null,
                "username": "JDoe",
                "passportNumber": null,
                "licenseNumber": null
            },
            "collectionIds": null
        }"#;

        assert_eq!(
            json.parse::<serde_json::Value>().unwrap(),
            expected.parse::<serde_json::Value>().unwrap()
        )
    }

    #[test]
    pub fn test_export() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources");
        d.push("json_export.json");

        let mut file = fs::File::open(d).unwrap();

        let mut expected = String::new();
        file.read_to_string(&mut expected).unwrap();

        let export = export_json(
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
        )
        .unwrap();

        assert_eq!(
            export.parse::<serde_json::Value>().unwrap(),
            expected.parse::<serde_json::Value>().unwrap()
        )
    }
}
