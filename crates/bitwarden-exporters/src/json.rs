use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{Cipher, Field, Folder};

pub(crate) fn export_json(folders: Vec<Folder>, ciphers: Vec<Cipher>) -> Result<String, String> {
    Ok("".to_owned())
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
    name: String,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonCipher {
    id: Uuid,
    folder_id: Option<Uuid>,
    // Organizational IDs which are always empty in personal exports
    organization_id: Option<Uuid>,
    collection_ids: Option<Vec<Uuid>>,

    name: String,
    notes: Option<String>,

    r#type: u8,
    //login: Option<JsonLogin>,
    //identity: Option<JsonIdentity>,
    //card: Option<JsonCard>,
    secure_note: Option<JsonSecureNote>,

    favorite: bool,
    reprompt: u8,

    fields: Vec<JsonField>,
    password_history: Option<Vec<String>>,

    revision_date: DateTime<Utc>,
    creation_date: DateTime<Utc>,
    deleted_date: Option<DateTime<Utc>>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonSecureNote {
    r#type: u8,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonField {
    name: Option<String>,
    value: Option<String>,
    r#type: u8,
    linked_id: Option<u8>,
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
            crate::CipherType::Login(_) => 1,
            crate::CipherType::SecureNote(_) => 2,
            crate::CipherType::Card(_) => 3,
            crate::CipherType::Identity() => 4,
        };

        let secure_note = match cipher.r#type {
            crate::CipherType::SecureNote(s) => Some(JsonSecureNote {
                r#type: s.r#type as u8,
            }),
            _ => None,
        };

        JsonCipher {
            id: cipher.id,
            folder_id: cipher.folder_id,
            organization_id: None,
            collection_ids: None,
            name: cipher.name,
            notes: cipher.notes,
            r#type,
            //login: None,
            //identity: None,
            //card: None,
            secure_note,
            favorite: cipher.favorite,
            reprompt: cipher.reprompt,
            fields: cipher.fields.iter().map(|f| f.clone().into()).collect(),
            password_history: None,
            revision_date: cipher.revision_date,
            creation_date: cipher.creation_date,
            deleted_date: cipher.deleted_date,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Cipher, Field, LoginUri};

    use super::*;

    #[test]
    fn test_convert_login() {
        let cipher = Cipher {
            id: "25c8c414-b446-48e9-a1bd-b10700bbd740".parse().unwrap(),
            folder_id: Some("942e2984-1b9a-453b-b039-b107012713b9".parse().unwrap()),

            name: "Bitwarden".to_string(),
            notes: Some("My note".to_string()),

            r#type: crate::CipherType::Login(crate::Login {
                username: "test@bitwarden.com".to_string(),
                password: "asdfasdfasdf".to_string(),
                login_uris: vec![LoginUri {
                    uri: Some("https://vault.bitwarden.com".to_string()),
                    r#match: None,
                }],
                totp: Some("ABC".to_string()),
            }),

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
        };

        // Convert to JsonCipher
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

            name: "My secure note".to_string(),
            notes: Some("Very secure!".to_string()),

            r#type: crate::CipherType::SecureNote(crate::SecureNote {
                r#type: crate::SecureNoteType::Generic,
            }),

            favorite: false,
            reprompt: 0,

            fields: vec![],

            revision_date: "2024-01-30T11:25:25.466Z".parse().unwrap(),
            creation_date: "2024-01-30T11:25:25.466Z".parse().unwrap(),
            deleted_date: None,
        };

        // Convert to JsonCipher
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
}
