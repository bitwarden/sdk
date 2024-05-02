use std::collections::HashMap;

use bitwarden_crypto::{DecryptedString, Sensitive};
use csv::Writer;
use serde::Serializer;
use thiserror::Error;

use crate::{Cipher, CipherType, Field, Folder};

#[derive(Debug, Error)]
pub enum CsvError {
    #[error("CSV error")]
    Csv,
}

pub(crate) fn export_csv(folders: Vec<Folder>, ciphers: Vec<Cipher>) -> Result<String, CsvError> {
    let folders: HashMap<_, _> = folders.into_iter().map(|f| (f.id, f.name)).collect();

    let rows = ciphers
        .into_iter()
        .filter(|c| matches!(c.r#type, CipherType::Login(_) | CipherType::SecureNote(_)))
        .map(|c| {
            let login = if let CipherType::Login(l) = &c.r#type {
                Some(l)
            } else {
                None
            };

            CsvRow {
                folder: c
                    .folder_id
                    .and_then(|f| folders.get(&f))
                    .map(|f| f.to_owned()),
                favorite: c.favorite,
                r#type: c.r#type.to_string(),
                name: c.name.to_owned(),
                notes: c.notes.to_owned(),
                fields: c.fields,
                reprompt: c.reprompt,
                login_uri: login
                    .map(|l| l.login_uris.iter().flat_map(|l| l.uri.clone()).collect())
                    .unwrap_or_default(),
                login_username: login.and_then(|l| l.username.clone()),
                login_password: login.and_then(|l| l.password.clone()),
                login_totp: login.and_then(|l| l.totp.clone()),
            }
        });

    let mut wtr = Writer::from_writer(vec![]);
    for row in rows {
        wtr.serialize(row).expect("Serialize should be infallible");
    }

    String::from_utf8(wtr.into_inner().map_err(|_| CsvError::Csv)?).map_err(|_| CsvError::Csv)
}

/// CSV export format. See <https://bitwarden.com/help/condition-bitwarden-import/#condition-a-csv>
///
/// Be careful when changing this struct to maintain compatibility with old exports.
#[derive(serde::Serialize)]
struct CsvRow {
    folder: Option<DecryptedString>,
    #[serde(serialize_with = "bool_serialize")]
    favorite: bool,
    r#type: String,
    name: DecryptedString,
    notes: Option<DecryptedString>,
    #[serde(serialize_with = "fields_serialize")]
    fields: Vec<Field>,
    reprompt: u8,
    #[serde(serialize_with = "vec_serialize")]
    login_uri: Vec<DecryptedString>,
    login_username: Option<DecryptedString>,
    login_password: Option<DecryptedString>,
    login_totp: Option<DecryptedString>,
}

fn vec_serialize<S>(x: &[DecryptedString], s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let iter = itertools::Itertools::intersperse(x.iter().map(|s| s.as_str()), ",");
    let result: Sensitive<String> = Sensitive::new(Box::new(iter.collect()));

    s.serialize_str(result.expose())
}

fn bool_serialize<S>(x: &bool, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(if *x { "1" } else { "" })
}

fn fields_serialize<S>(x: &[Field], s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(
        x.iter()
            .map(|f| {
                format!(
                    "{}: {}",
                    f.name.as_ref().map(|n| n.as_str()).unwrap_or_default(),
                    f.value.as_ref().map(|n| n.as_str()).unwrap_or_default(),
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
            .as_str(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Card, Identity, Login, LoginUri};

    #[test]
    fn test_export_csv() {
        let folders = vec![
            Folder {
                id: "d55d65d7-c161-40a4-94ca-b0d20184d91a".parse().unwrap(),
                name: DecryptedString::test("Test Folder A"),
            },
            Folder {
                id: "583e7665-0126-4d37-9139-b0d20184dd86".parse().unwrap(),
                name: DecryptedString::test("Test Folder B"),
            },
        ];
        let ciphers = vec![
            Cipher {
                id: "d55d65d7-c161-40a4-94ca-b0d20184d91a".parse().unwrap(),
                folder_id: None,
                name: DecryptedString::test("test@bitwarden.com"),
                notes: None,
                r#type: CipherType::Login(Box::new(Login {
                    username: Some(DecryptedString::test("test@bitwarden.com")),
                    password: Some(DecryptedString::test("Abc123")),
                    login_uris: vec![LoginUri {
                        uri: Some(DecryptedString::test("https://google.com")),
                        r#match: None,
                    }],
                    totp: None,
                })),
                favorite: false,
                reprompt: 0,
                fields: vec![],
                revision_date: "2024-01-30T11:28:20.036Z".parse().unwrap(),
                creation_date: "2024-01-30T11:28:20.036Z".parse().unwrap(),
                deleted_date: None,
            },
            Cipher {
                id: "7dd81bd0-cc72-4f42-96e7-b0fc014e71a3".parse().unwrap(),
                folder_id: Some("583e7665-0126-4d37-9139-b0d20184dd86".parse().unwrap()),
                name: DecryptedString::test("Steam Account"),
                notes: None,
                r#type: CipherType::Login(Box::new(Login {
                    username: Some(DecryptedString::test("steam")),
                    password: Some(DecryptedString::test("3Pvb8u7EfbV*nJ")),
                    login_uris: vec![LoginUri {
                        uri: Some(DecryptedString::test("https://steampowered.com")),
                        r#match: None,
                    }],
                    totp: Some(DecryptedString::test("steam://ABCD123")),
                })),
                favorite: true,
                reprompt: 0,
                fields: vec![
                    Field {
                        name: Some(DecryptedString::test("Test")),
                        value: Some(DecryptedString::test("v")),
                        r#type: 0,
                        linked_id: None,
                    },
                    Field {
                        name: Some(DecryptedString::test("Hidden")),
                        value: Some(DecryptedString::test("asdfer")),
                        r#type: 1,
                        linked_id: None,
                    },
                ],
                revision_date: "2024-01-30T11:28:20.036Z".parse().unwrap(),
                creation_date: "2024-01-30T11:28:20.036Z".parse().unwrap(),
                deleted_date: None,
            },
        ];

        let csv = export_csv(folders, ciphers).unwrap();
        let expected = [
            "folder,favorite,type,name,notes,fields,reprompt,login_uri,login_username,login_password,login_totp",
            ",,login,test@bitwarden.com,,,0,https://google.com,test@bitwarden.com,Abc123,",
            "Test Folder B,1,login,Steam Account,,\"Test: v\nHidden: asdfer\",0,https://steampowered.com,steam,3Pvb8u7EfbV*nJ,steam://ABCD123",
            "",
        ].join("\n");

        assert_eq!(csv, expected);
    }

    #[test]
    fn test_export_ignore_card() {
        let folders = vec![];
        let ciphers = vec![Cipher {
            id: "d55d65d7-c161-40a4-94ca-b0d20184d91a".parse().unwrap(),
            folder_id: None,
            name: DecryptedString::test("My Card"),
            notes: None,
            r#type: CipherType::Card(Box::new(Card {
                cardholder_name: None,
                exp_month: None,
                exp_year: None,
                code: None,
                brand: None,
                number: None,
            })),
            favorite: false,
            reprompt: 0,
            fields: vec![],
            revision_date: "2024-01-30T11:28:20.036Z".parse().unwrap(),
            creation_date: "2024-01-30T11:28:20.036Z".parse().unwrap(),
            deleted_date: None,
        }];

        let csv = export_csv(folders, ciphers).unwrap();

        assert_eq!(csv, "");
    }

    #[test]
    fn test_export_ignore_identity() {
        let folders = vec![];
        let ciphers = vec![Cipher {
            id: "d55d65d7-c161-40a4-94ca-b0d20184d91a".parse().unwrap(),
            folder_id: None,
            name: DecryptedString::test("My Identity"),
            notes: None,
            r#type: CipherType::Identity(Box::new(Identity {
                title: None,
                first_name: None,
                middle_name: None,
                last_name: None,
                address1: None,
                address2: None,
                address3: None,
                city: None,
                state: None,
                postal_code: None,
                country: None,
                company: None,
                email: None,
                phone: None,
                ssn: None,
                username: None,
                passport_number: None,
                license_number: None,
            })),
            favorite: false,
            reprompt: 0,
            fields: vec![],
            revision_date: "2024-01-30T11:28:20.036Z".parse().unwrap(),
            creation_date: "2024-01-30T11:28:20.036Z".parse().unwrap(),
            deleted_date: None,
        }];

        let csv = export_csv(folders, ciphers).unwrap();

        assert_eq!(csv, "");
    }
}
