use bitwarden_api_api::models::CipherDetailsResponseModel;
use bitwarden_crypto::{
    CryptoError, EncString, KeyContainer, KeyDecryptable, KeyEncryptable, LocateKey,
    SymmetricCryptoKey,
};
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use uuid::Uuid;

use super::{
    attachment, card, field, identity,
    local_data::{LocalData, LocalDataView},
    login, secure_note,
};
use crate::{
    error::{Error, Result},
    vault::password_history,
};

#[derive(Clone, Copy, Serialize_repr, Deserialize_repr, Debug, JsonSchema)]
#[repr(u8)]
#[cfg_attr(feature = "mobile", derive(uniffi::Enum))]
pub enum CipherType {
    Login = 1,
    SecureNote = 2,
    Card = 3,
    Identity = 4,
}

#[derive(Clone, Copy, Serialize_repr, Deserialize_repr, Debug, JsonSchema)]
#[repr(u8)]
#[cfg_attr(feature = "mobile", derive(uniffi::Enum))]
pub enum CipherRepromptType {
    None = 0,
    Password = 1,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct Cipher {
    pub id: Option<Uuid>,
    pub organization_id: Option<Uuid>,
    pub folder_id: Option<Uuid>,
    pub collection_ids: Vec<Uuid>,

    /// More recent ciphers uses individual encryption keys to encrypt the other fields of the
    /// Cipher.
    pub key: Option<EncString>,

    pub name: EncString,
    pub notes: Option<EncString>,

    pub r#type: CipherType,
    pub login: Option<login::Login>,
    pub identity: Option<identity::Identity>,
    pub card: Option<card::Card>,
    pub secure_note: Option<secure_note::SecureNote>,

    pub favorite: bool,
    pub reprompt: CipherRepromptType,
    pub organization_use_totp: bool,
    pub edit: bool,
    pub view_password: bool,
    pub local_data: Option<LocalData>,

    pub attachments: Option<Vec<attachment::Attachment>>,
    pub fields: Option<Vec<field::Field>>,
    pub password_history: Option<Vec<password_history::PasswordHistory>>,

    pub creation_date: DateTime<Utc>,
    pub deleted_date: Option<DateTime<Utc>>,
    pub revision_date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct CipherView {
    pub id: Option<Uuid>,
    pub organization_id: Option<Uuid>,
    pub folder_id: Option<Uuid>,
    pub collection_ids: Vec<Uuid>,

    pub key: Option<EncString>,

    pub name: String,
    pub notes: Option<String>,

    pub r#type: CipherType,
    pub login: Option<login::LoginView>,
    pub identity: Option<identity::IdentityView>,
    pub card: Option<card::CardView>,
    pub secure_note: Option<secure_note::SecureNoteView>,

    pub favorite: bool,
    pub reprompt: CipherRepromptType,
    pub organization_use_totp: bool,
    pub edit: bool,
    pub view_password: bool,
    pub local_data: Option<LocalDataView>,

    pub attachments: Option<Vec<attachment::AttachmentView>>,
    pub fields: Option<Vec<field::FieldView>>,
    pub password_history: Option<Vec<password_history::PasswordHistoryView>>,

    pub creation_date: DateTime<Utc>,
    pub deleted_date: Option<DateTime<Utc>>,
    pub revision_date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct CipherListView {
    pub id: Option<Uuid>,
    pub organization_id: Option<Uuid>,
    pub folder_id: Option<Uuid>,
    pub collection_ids: Vec<Uuid>,

    pub name: String,
    pub sub_title: String,

    pub r#type: CipherType,

    pub favorite: bool,
    pub reprompt: CipherRepromptType,
    pub edit: bool,
    pub view_password: bool,

    /// The number of attachments
    pub attachments: u32,

    pub creation_date: DateTime<Utc>,
    pub deleted_date: Option<DateTime<Utc>>,
    pub revision_date: DateTime<Utc>,
}

impl KeyEncryptable<SymmetricCryptoKey, Cipher> for CipherView {
    fn encrypt_with_key(self, key: &SymmetricCryptoKey) -> Result<Cipher, CryptoError> {
        let ciphers_key = Cipher::get_cipher_key(key, &self.key)?;
        let key = ciphers_key.as_ref().unwrap_or(key);

        Ok(Cipher {
            id: self.id,
            organization_id: self.organization_id,
            folder_id: self.folder_id,
            collection_ids: self.collection_ids,
            key: self.key,
            name: self.name.encrypt_with_key(key)?,
            notes: self.notes.encrypt_with_key(key)?,
            r#type: self.r#type,
            login: self.login.encrypt_with_key(key)?,
            identity: self.identity.encrypt_with_key(key)?,
            card: self.card.encrypt_with_key(key)?,
            secure_note: self.secure_note.encrypt_with_key(key)?,
            favorite: self.favorite,
            reprompt: self.reprompt,
            organization_use_totp: self.organization_use_totp,
            edit: self.edit,
            view_password: self.view_password,
            local_data: self.local_data.encrypt_with_key(key)?,
            attachments: self.attachments.encrypt_with_key(key)?,
            fields: self.fields.encrypt_with_key(key)?,
            password_history: self.password_history.encrypt_with_key(key)?,
            creation_date: self.creation_date,
            deleted_date: self.deleted_date,
            revision_date: self.revision_date,
        })
    }
}

impl KeyDecryptable<SymmetricCryptoKey, CipherView> for Cipher {
    fn decrypt_with_key(&self, key: &SymmetricCryptoKey) -> Result<CipherView, CryptoError> {
        let ciphers_key = Cipher::get_cipher_key(key, &self.key)?;
        let key = ciphers_key.as_ref().unwrap_or(key);

        Ok(CipherView {
            id: self.id,
            organization_id: self.organization_id,
            folder_id: self.folder_id,
            collection_ids: self.collection_ids.clone(),
            key: self.key.clone(),
            name: self.name.decrypt_with_key(key).ok().unwrap_or_default(),
            notes: self.notes.decrypt_with_key(key).ok().flatten(),
            r#type: self.r#type,
            login: self.login.decrypt_with_key(key).ok().flatten(),
            identity: self.identity.decrypt_with_key(key).ok().flatten(),
            card: self.card.decrypt_with_key(key).ok().flatten(),
            secure_note: self.secure_note.decrypt_with_key(key).ok().flatten(),
            favorite: self.favorite,
            reprompt: self.reprompt,
            organization_use_totp: self.organization_use_totp,
            edit: self.edit,
            view_password: self.view_password,
            local_data: self.local_data.decrypt_with_key(key).ok().flatten(),
            attachments: self.attachments.decrypt_with_key(key).ok().flatten(),
            fields: self.fields.decrypt_with_key(key).ok().flatten(),
            password_history: self.password_history.decrypt_with_key(key).ok().flatten(),
            creation_date: self.creation_date,
            deleted_date: self.deleted_date,
            revision_date: self.revision_date,
        })
    }
}

impl Cipher {
    /// Get the decrypted individual encryption key for this cipher.
    /// Note that some ciphers do not have individual encryption keys,
    /// in which case this will return Ok(None) and the key associated
    /// with this cipher's user or organization must be used instead
    pub(super) fn get_cipher_key(
        key: &SymmetricCryptoKey,
        ciphers_key: &Option<EncString>,
    ) -> Result<Option<SymmetricCryptoKey>, CryptoError> {
        ciphers_key
            .as_ref()
            .map(|k| {
                let mut key: Vec<u8> = k.decrypt_with_key(key)?;
                SymmetricCryptoKey::try_from(key.as_mut_slice())
            })
            .transpose()
    }

    fn get_decrypted_subtitle(&self, key: &SymmetricCryptoKey) -> Result<String, CryptoError> {
        Ok(match self.r#type {
            CipherType::Login => {
                let Some(login) = &self.login else {
                    return Ok(String::new());
                };
                login.username.decrypt_with_key(key)?.unwrap_or_default()
            }
            CipherType::SecureNote => String::new(),
            CipherType::Card => {
                let Some(card) = &self.card else {
                    return Ok(String::new());
                };
                let mut sub_title = String::new();

                if let Some(brand) = &card.brand {
                    let brand: String = brand.decrypt_with_key(key)?;
                    sub_title.push_str(&brand);
                }

                if let Some(number) = &card.number {
                    let number: String = number.decrypt_with_key(key)?;
                    let number_len = number.len();
                    if number_len > 4 {
                        if !sub_title.is_empty() {
                            sub_title.push_str(", ");
                        }

                        // On AMEX cards we show 5 digits instead of 4
                        let digit_count = match &number[0..2] {
                            "34" | "37" => 5,
                            _ => 4,
                        };

                        sub_title.push_str(&number[(number_len - digit_count)..]);
                    }
                }

                sub_title
            }
            CipherType::Identity => {
                let Some(identity) = &self.identity else {
                    return Ok(String::new());
                };
                let mut sub_title = String::new();

                if let Some(first_name) = &identity.first_name {
                    let first_name: String = first_name.decrypt_with_key(key)?;
                    sub_title.push_str(&first_name);
                }

                if let Some(last_name) = &identity.last_name {
                    if !sub_title.is_empty() {
                        sub_title.push(' ');
                    }
                    let last_name: String = last_name.decrypt_with_key(key)?;
                    sub_title.push_str(&last_name);
                }

                sub_title
            }
        })
    }
}

impl KeyDecryptable<SymmetricCryptoKey, CipherListView> for Cipher {
    fn decrypt_with_key(&self, key: &SymmetricCryptoKey) -> Result<CipherListView, CryptoError> {
        let ciphers_key = Cipher::get_cipher_key(key, &self.key)?;
        let key = ciphers_key.as_ref().unwrap_or(key);

        Ok(CipherListView {
            id: self.id,
            organization_id: self.organization_id,
            folder_id: self.folder_id,
            collection_ids: self.collection_ids.clone(),
            name: self.name.decrypt_with_key(key).ok().unwrap_or_default(),
            sub_title: self.get_decrypted_subtitle(key).ok().unwrap_or_default(),
            r#type: self.r#type,
            favorite: self.favorite,
            reprompt: self.reprompt,
            edit: self.edit,
            view_password: self.view_password,
            attachments: self
                .attachments
                .as_ref()
                .map(|a| a.len() as u32)
                .unwrap_or(0),
            creation_date: self.creation_date,
            deleted_date: self.deleted_date,
            revision_date: self.revision_date,
        })
    }
}

impl LocateKey for Cipher {
    fn locate_key<'a>(
        &self,
        enc: &'a dyn KeyContainer,
        _: &Option<Uuid>,
    ) -> Option<&'a SymmetricCryptoKey> {
        enc.get_key(&self.organization_id)
    }
}
impl LocateKey for CipherView {
    fn locate_key<'a>(
        &self,
        enc: &'a dyn KeyContainer,
        _: &Option<Uuid>,
    ) -> Option<&'a SymmetricCryptoKey> {
        enc.get_key(&self.organization_id)
    }
}

impl TryFrom<CipherDetailsResponseModel> for Cipher {
    type Error = Error;

    fn try_from(cipher: CipherDetailsResponseModel) -> Result<Self> {
        Ok(Self {
            id: cipher.id,
            organization_id: cipher.organization_id,
            folder_id: cipher.folder_id,
            collection_ids: cipher.collection_ids.unwrap_or_default(),
            name: EncString::try_from_optional(cipher.name)?.ok_or(Error::MissingFields)?,
            notes: EncString::try_from_optional(cipher.notes)?,
            r#type: cipher.r#type.ok_or(Error::MissingFields)?.into(),
            login: cipher.login.map(|l| (*l).try_into()).transpose()?,
            identity: cipher.identity.map(|i| (*i).try_into()).transpose()?,
            card: cipher.card.map(|c| (*c).try_into()).transpose()?,
            secure_note: cipher.secure_note.map(|s| (*s).try_into()).transpose()?,
            favorite: cipher.favorite.unwrap_or(false),
            reprompt: cipher
                .reprompt
                .map(|r| r.into())
                .unwrap_or(CipherRepromptType::None),
            organization_use_totp: cipher.organization_use_totp.unwrap_or(true),
            edit: cipher.edit.unwrap_or(true),
            view_password: cipher.view_password.unwrap_or(true),
            local_data: None, // Not sent from server
            attachments: cipher
                .attachments
                .map(|a| a.into_iter().map(|a| a.try_into()).collect())
                .transpose()?,
            fields: cipher
                .fields
                .map(|f| f.into_iter().map(|f| f.try_into()).collect())
                .transpose()?,
            password_history: cipher
                .password_history
                .map(|p| p.into_iter().map(|p| p.try_into()).collect())
                .transpose()?,
            creation_date: cipher.creation_date.ok_or(Error::MissingFields)?.parse()?,
            deleted_date: cipher.deleted_date.map(|d| d.parse()).transpose()?,
            revision_date: cipher.revision_date.ok_or(Error::MissingFields)?.parse()?,
            key: EncString::try_from_optional(cipher.key)?,
        })
    }
}

impl From<bitwarden_api_api::models::CipherType> for CipherType {
    fn from(t: bitwarden_api_api::models::CipherType) -> Self {
        match t {
            bitwarden_api_api::models::CipherType::Variant1 => CipherType::Login,
            bitwarden_api_api::models::CipherType::Variant2 => CipherType::SecureNote,
            bitwarden_api_api::models::CipherType::Variant3 => CipherType::Card,
            bitwarden_api_api::models::CipherType::Variant4 => CipherType::Identity,
        }
    }
}

impl From<bitwarden_api_api::models::CipherRepromptType> for CipherRepromptType {
    fn from(t: bitwarden_api_api::models::CipherRepromptType) -> Self {
        match t {
            bitwarden_api_api::models::CipherRepromptType::Variant0 => CipherRepromptType::None,
            bitwarden_api_api::models::CipherRepromptType::Variant1 => CipherRepromptType::Password,
        }
    }
}
