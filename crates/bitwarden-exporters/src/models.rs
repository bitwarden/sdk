use bitwarden_core::{require, MissingFieldError};
use bitwarden_vault::{
    CipherType, CipherView, FieldView, FolderView, LoginUriView, SecureNoteType,
};

impl TryFrom<FolderView> for crate::Folder {
    type Error = MissingFieldError;

    fn try_from(value: FolderView) -> Result<Self, Self::Error> {
        Ok(Self {
            id: require!(value.id),
            name: value.name,
        })
    }
}

impl TryFrom<CipherView> for crate::Cipher {
    type Error = MissingFieldError;

    fn try_from(value: CipherView) -> Result<Self, Self::Error> {
        let r = match value.r#type {
            CipherType::Login => {
                let l = require!(value.login);
                crate::CipherType::Login(Box::new(crate::Login {
                    username: l.username,
                    password: l.password,
                    login_uris: l
                        .uris
                        .unwrap_or_default()
                        .into_iter()
                        .map(|u| u.into())
                        .collect(),
                    totp: l.totp,
                }))
            }
            CipherType::SecureNote => crate::CipherType::SecureNote(Box::new(crate::SecureNote {
                r#type: value
                    .secure_note
                    .map(|t| t.r#type)
                    .unwrap_or(SecureNoteType::Generic)
                    .into(),
            })),
            CipherType::Card => {
                let c = require!(value.card);
                crate::CipherType::Card(Box::new(crate::Card {
                    cardholder_name: c.cardholder_name,
                    exp_month: c.exp_month,
                    exp_year: c.exp_year,
                    code: c.code,
                    brand: c.brand,
                    number: c.number,
                }))
            }
            CipherType::Identity => {
                let i = require!(value.identity);
                crate::CipherType::Identity(Box::new(crate::Identity {
                    title: i.title,
                    first_name: i.first_name,
                    middle_name: i.middle_name,
                    last_name: i.last_name,
                    address1: i.address1,
                    address2: i.address2,
                    address3: i.address3,
                    city: i.city,
                    state: i.state,
                    postal_code: i.postal_code,
                    country: i.country,
                    company: i.company,
                    email: i.email,
                    phone: i.phone,
                    ssn: i.ssn,
                    username: i.username,
                    passport_number: i.passport_number,
                    license_number: i.license_number,
                }))
            }
        };

        Ok(Self {
            id: require!(value.id),
            folder_id: value.folder_id,
            name: value.name,
            notes: value.notes,
            r#type: r,
            favorite: value.favorite,
            reprompt: value.reprompt as u8,
            fields: value
                .fields
                .unwrap_or_default()
                .into_iter()
                .map(|f| f.into())
                .collect(),
            revision_date: value.revision_date,
            creation_date: value.creation_date,
            deleted_date: value.deleted_date,
        })
    }
}

impl From<FieldView> for crate::Field {
    fn from(value: FieldView) -> Self {
        Self {
            name: value.name,
            value: value.value,
            r#type: value.r#type as u8,
            linked_id: value.linked_id.map(|id| id.into()),
        }
    }
}

impl From<LoginUriView> for crate::LoginUri {
    fn from(value: LoginUriView) -> Self {
        Self {
            r#match: value.r#match.map(|v| v as u8),
            uri: value.uri,
        }
    }
}

impl From<SecureNoteType> for crate::SecureNoteType {
    fn from(value: SecureNoteType) -> Self {
        match value {
            SecureNoteType::Generic => crate::SecureNoteType::Generic,
        }
    }
}

#[cfg(test)]
mod tests {
    use bitwarden_vault::{CipherRepromptType, LoginView};
    use chrono::{DateTime, Utc};

    use super::*;

    #[test]
    fn test_try_from_folder_view() {
        let view = FolderView {
            id: Some("fd411a1a-fec8-4070-985d-0e6560860e69".parse().unwrap()),
            name: "test_name".to_string(),
            revision_date: "2024-01-30T17:55:36.150Z".parse().unwrap(),
        };

        let f: crate::Folder = view.try_into().unwrap();

        assert_eq!(
            f.id,
            "fd411a1a-fec8-4070-985d-0e6560860e69".parse().unwrap()
        );
        assert_eq!(f.name, "test_name".to_string());
    }

    #[test]
    fn test_try_from_cipher_view_login() {
        let cipher_view = CipherView {
            r#type: CipherType::Login,
            login: Some(LoginView {
                username: Some("test_username".to_string()),
                password: Some("test_password".to_string()),
                password_revision_date: None,
                uris: None,
                totp: None,
                autofill_on_page_load: None,
                fido2_credentials: None,
            }),
            id: "fd411a1a-fec8-4070-985d-0e6560860e69".parse().ok(),
            organization_id: None,
            folder_id: None,
            collection_ids: vec![],
            key: None,
            name: "My login".to_string(),
            notes: None,
            identity: None,
            card: None,
            secure_note: None,
            favorite: false,
            reprompt: CipherRepromptType::None,
            organization_use_totp: true,
            edit: true,
            view_password: true,
            local_data: None,
            attachments: None,
            fields: None,
            password_history: None,
            creation_date: "2024-01-30T17:55:36.150Z".parse().unwrap(),
            deleted_date: None,
            revision_date: "2024-01-30T17:55:36.150Z".parse().unwrap(),
        };

        let cipher: crate::Cipher = cipher_view.try_into().unwrap();

        assert_eq!(
            cipher.id,
            "fd411a1a-fec8-4070-985d-0e6560860e69".parse().unwrap()
        );
        assert_eq!(cipher.folder_id, None);
        assert_eq!(cipher.name, "My login".to_string());
        assert_eq!(cipher.notes, None);
        assert!(!cipher.favorite);
        assert_eq!(cipher.reprompt, 0);
        assert!(cipher.fields.is_empty());
        assert_eq!(
            cipher.revision_date,
            "2024-01-30T17:55:36.150Z".parse::<DateTime<Utc>>().unwrap()
        );
        assert_eq!(
            cipher.creation_date,
            "2024-01-30T17:55:36.150Z".parse::<DateTime<Utc>>().unwrap()
        );
        assert_eq!(cipher.deleted_date, None);

        if let crate::CipherType::Login(l) = cipher.r#type {
            assert_eq!(l.username, Some("test_username".to_string()));
            assert_eq!(l.password, Some("test_password".to_string()));
            assert!(l.login_uris.is_empty());
            assert_eq!(l.totp, None);
        } else {
            panic!("Expected login type");
        }
    }
}
