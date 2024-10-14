use bitwarden_api_api::models::CipherDetailsResponseModel;
use bitwarden_core::{require, MissingFieldError, VaultLocked};
use bitwarden_crypto::{
    CryptoError, EncString, KeyContainer, KeyDecryptable, KeyEncryptable, LocateKey,
    SymmetricCryptoKey,
};
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use thiserror::Error;
use uuid::Uuid;

use super::{
    attachment, card, field, identity,
    local_data::{LocalData, LocalDataView},
    secure_note,
};
use crate::{
    password_history, Fido2CredentialFullView, Fido2CredentialView, Login, LoginView,
    VaultParseError,
};

#[derive(Debug, Error)]
pub enum CipherError {
    #[error(transparent)]
    MissingFieldError(#[from] MissingFieldError),
    #[error(transparent)]
    VaultLocked(#[from] VaultLocked),
    #[error(transparent)]
    CryptoError(#[from] CryptoError),
    #[error("This cipher contains attachments without keys. Those attachments will need to be reuploaded to complete the operation")]
    AttachmentsWithoutKeys,
}

#[derive(Clone, Copy, Serialize_repr, Deserialize_repr, Debug, JsonSchema)]
#[repr(u8)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum CipherType {
    Login = 1,
    SecureNote = 2,
    Card = 3,
    Identity = 4,
}

#[derive(Clone, Copy, Serialize_repr, Deserialize_repr, Debug, JsonSchema, PartialEq)]
#[repr(u8)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum CipherRepromptType {
    None = 0,
    Password = 1,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
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
    pub login: Option<Login>,
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

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct CipherView {
    pub id: Option<Uuid>,
    pub organization_id: Option<Uuid>,
    pub folder_id: Option<Uuid>,
    pub collection_ids: Vec<Uuid>,

    /// Temporary, required to support re-encrypting existing items.
    pub key: Option<EncString>,

    pub name: String,
    pub notes: Option<String>,

    pub r#type: CipherType,
    pub login: Option<LoginView>,
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

#[derive(Serialize, Deserialize, Debug, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum CipherListViewType {
    Login {
        has_fido2: bool,
        totp: Option<EncString>,
    },
    SecureNote,
    Card,
    Identity,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct CipherListView {
    pub id: Option<Uuid>,
    pub organization_id: Option<Uuid>,
    pub folder_id: Option<Uuid>,
    pub collection_ids: Vec<Uuid>,

    /// Temporary, required to support calculating TOTP from CipherListView.
    pub key: Option<EncString>,

    pub name: String,
    pub sub_title: String,

    pub r#type: CipherListViewType,

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

impl CipherListView {
    pub(crate) fn get_totp_key(
        self,
        enc: &dyn KeyContainer,
    ) -> Result<Option<String>, CryptoError> {
        let key = self.locate_key(enc, &None)?;
        let cipher_key = Cipher::get_cipher_key(key, &self.key)?;
        let key = cipher_key.as_ref().unwrap_or(key);

        let totp = if let CipherListViewType::Login { totp, .. } = self.r#type {
            totp.decrypt_with_key(key)?
        } else {
            None
        };

        Ok(totp)
    }
}

impl KeyEncryptable<SymmetricCryptoKey, Cipher> for CipherView {
    fn encrypt_with_key(mut self, key: &SymmetricCryptoKey) -> Result<Cipher, CryptoError> {
        let ciphers_key = Cipher::get_cipher_key(key, &self.key)?;
        let key = ciphers_key.as_ref().unwrap_or(key);

        // For compatibility reasons, we only create checksums for ciphers that have a key
        if ciphers_key.is_some() {
            self.generate_checksums();
        }

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

        let mut cipher = CipherView {
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
        };

        // For compatibility we only remove URLs with invalid checksums if the cipher has a key
        if ciphers_key.is_some() {
            cipher.remove_invalid_checksums();
        }

        Ok(cipher)
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

                build_subtitle_card(
                    card.brand
                        .as_ref()
                        .map(|b| b.decrypt_with_key(key))
                        .transpose()?,
                    card.number
                        .as_ref()
                        .map(|n| n.decrypt_with_key(key))
                        .transpose()?,
                )
            }
            CipherType::Identity => {
                let Some(identity) = &self.identity else {
                    return Ok(String::new());
                };

                build_subtitle_identity(
                    identity
                        .first_name
                        .as_ref()
                        .map(|f| f.decrypt_with_key(key))
                        .transpose()?,
                    identity
                        .last_name
                        .as_ref()
                        .map(|l| l.decrypt_with_key(key))
                        .transpose()?,
                )
            }
        })
    }
}

/// Builds the subtitle for a card cipher
fn build_subtitle_card(brand: Option<String>, number: Option<String>) -> String {
    // Attempt to pre-allocate the string with the expected max-size
    let mut sub_title =
        String::with_capacity(brand.as_ref().map(|b| b.len()).unwrap_or_default() + 8);

    if let Some(brand) = brand {
        sub_title.push_str(&brand);
    }

    if let Some(number) = number {
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

            sub_title.push('*');
            sub_title.push_str(&number[(number_len - digit_count)..]);
        }
    }

    sub_title
}

/// Builds the subtitle for a card cipher
fn build_subtitle_identity(first_name: Option<String>, last_name: Option<String>) -> String {
    let len = match (first_name.as_ref(), last_name.as_ref()) {
        (Some(first_name), Some(last_name)) => first_name.len() + last_name.len() + 1,
        (Some(first_name), None) => first_name.len(),
        (None, Some(last_name)) => last_name.len(),
        (None, None) => 0,
    };

    let mut sub_title = String::with_capacity(len);

    if let Some(first_name) = &first_name {
        sub_title.push_str(first_name);
    }

    if let Some(last_name) = &last_name {
        if !sub_title.is_empty() {
            sub_title.push(' ');
        }
        sub_title.push_str(last_name);
    }

    sub_title
}

impl CipherView {
    pub fn generate_cipher_key(&mut self, key: &SymmetricCryptoKey) -> Result<(), CryptoError> {
        let old_ciphers_key = Cipher::get_cipher_key(key, &self.key)?;
        let old_key = old_ciphers_key.as_ref().unwrap_or(key);

        let new_key = SymmetricCryptoKey::generate(rand::thread_rng());

        self.reencrypt_attachment_keys(old_key, &new_key)?;
        self.reencrypt_fido2_credentials(old_key, &new_key)?;

        self.key = Some(new_key.to_vec().encrypt_with_key(key)?);
        Ok(())
    }

    pub fn generate_checksums(&mut self) {
        if let Some(uris) = self.login.as_mut().and_then(|l| l.uris.as_mut()) {
            for uri in uris {
                uri.generate_checksum();
            }
        }
    }

    pub fn remove_invalid_checksums(&mut self) {
        if let Some(uris) = self.login.as_mut().and_then(|l| l.uris.as_mut()) {
            uris.retain(|u| u.is_checksum_valid());
        }
    }

    fn reencrypt_attachment_keys(
        &mut self,
        old_key: &SymmetricCryptoKey,
        new_key: &SymmetricCryptoKey,
    ) -> Result<(), CryptoError> {
        if let Some(attachments) = &mut self.attachments {
            for attachment in attachments {
                if let Some(attachment_key) = &mut attachment.key {
                    let dec_attachment_key: Vec<u8> = attachment_key.decrypt_with_key(old_key)?;
                    *attachment_key = dec_attachment_key.encrypt_with_key(new_key)?;
                }
            }
        }
        Ok(())
    }

    pub fn decrypt_fido2_credentials(
        &self,
        enc: &dyn KeyContainer,
    ) -> Result<Vec<Fido2CredentialView>, CipherError> {
        let key = self.locate_key(enc, &None)?;
        let cipher_key = Cipher::get_cipher_key(key, &self.key)?;

        let key = cipher_key.as_ref().unwrap_or(key);

        Ok(self
            .login
            .as_ref()
            .and_then(|l| l.fido2_credentials.as_ref())
            .map(|f| f.decrypt_with_key(key))
            .transpose()?
            .unwrap_or_default())
    }

    fn reencrypt_fido2_credentials(
        &mut self,
        old_key: &SymmetricCryptoKey,
        new_key: &SymmetricCryptoKey,
    ) -> Result<(), CryptoError> {
        if let Some(login) = self.login.as_mut() {
            if let Some(fido2_credentials) = &mut login.fido2_credentials {
                let dec_fido2_credentials: Vec<Fido2CredentialFullView> =
                    fido2_credentials.decrypt_with_key(old_key)?;
                *fido2_credentials = dec_fido2_credentials.encrypt_with_key(new_key)?;
            }
        }
        Ok(())
    }

    pub fn move_to_organization(
        &mut self,
        enc: &dyn KeyContainer,
        organization_id: Uuid,
    ) -> Result<(), CipherError> {
        let old_key = enc.get_key(&self.organization_id)?;

        let new_key = enc.get_key(&Some(organization_id))?;

        // If any attachment is missing a key we can't reencrypt the attachment keys
        if self.attachments.iter().flatten().any(|a| a.key.is_none()) {
            return Err(CipherError::AttachmentsWithoutKeys);
        }

        // If the cipher has a key, we need to re-encrypt it with the new organization key
        if let Some(cipher_key) = &mut self.key {
            let dec_cipher_key: Vec<u8> = cipher_key.decrypt_with_key(old_key)?;
            *cipher_key = dec_cipher_key.encrypt_with_key(new_key)?;
        } else {
            // If the cipher does not have a key, we need to reencrypt all attachment keys
            self.reencrypt_attachment_keys(old_key, new_key)?;
            self.reencrypt_fido2_credentials(old_key, new_key)?;
        }

        self.organization_id = Some(organization_id);
        Ok(())
    }

    pub fn set_new_fido2_credentials(
        &mut self,
        enc: &dyn KeyContainer,
        creds: Vec<Fido2CredentialFullView>,
    ) -> Result<(), CipherError> {
        let key = enc.get_key(&self.organization_id)?;

        let ciphers_key = Cipher::get_cipher_key(key, &self.key)?;
        let ciphers_key = ciphers_key.as_ref().unwrap_or(key);

        require!(self.login.as_mut()).fido2_credentials =
            Some(creds.encrypt_with_key(ciphers_key)?);

        Ok(())
    }

    pub fn get_fido2_credentials(
        &self,
        enc: &dyn KeyContainer,
    ) -> Result<Vec<Fido2CredentialFullView>, CipherError> {
        let key = enc.get_key(&self.organization_id)?;

        let ciphers_key = Cipher::get_cipher_key(key, &self.key)?;
        let ciphers_key = ciphers_key.as_ref().unwrap_or(key);

        let login = require!(self.login.as_ref());
        let creds = require!(login.fido2_credentials.as_ref());
        let res = creds.decrypt_with_key(ciphers_key)?;
        Ok(res)
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
            key: self.key.clone(),
            name: self.name.decrypt_with_key(key).ok().unwrap_or_default(),
            sub_title: self.get_decrypted_subtitle(key).ok().unwrap_or_default(),
            r#type: match self.r#type {
                CipherType::Login => {
                    let login = self
                        .login
                        .as_ref()
                        .ok_or(CryptoError::MissingField("login"))?;
                    CipherListViewType::Login {
                        has_fido2: login.fido2_credentials.is_some(),
                        totp: login.totp.clone(),
                    }
                }
                CipherType::SecureNote => CipherListViewType::SecureNote,
                CipherType::Card => CipherListViewType::Card,
                CipherType::Identity => CipherListViewType::Identity,
            },
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
    ) -> Result<&'a SymmetricCryptoKey, CryptoError> {
        enc.get_key(&self.organization_id)
    }
}
impl LocateKey for CipherView {
    fn locate_key<'a>(
        &self,
        enc: &'a dyn KeyContainer,
        _: &Option<Uuid>,
    ) -> Result<&'a SymmetricCryptoKey, CryptoError> {
        enc.get_key(&self.organization_id)
    }
}
impl LocateKey for CipherListView {
    fn locate_key<'a>(
        &self,
        enc: &'a dyn KeyContainer,
        _: &Option<Uuid>,
    ) -> Result<&'a SymmetricCryptoKey, CryptoError> {
        enc.get_key(&self.organization_id)
    }
}

impl TryFrom<CipherDetailsResponseModel> for Cipher {
    type Error = VaultParseError;

    fn try_from(cipher: CipherDetailsResponseModel) -> Result<Self, Self::Error> {
        Ok(Self {
            id: cipher.id,
            organization_id: cipher.organization_id,
            folder_id: cipher.folder_id,
            collection_ids: cipher.collection_ids.unwrap_or_default(),
            name: require!(EncString::try_from_optional(cipher.name)?),
            notes: EncString::try_from_optional(cipher.notes)?,
            r#type: require!(cipher.r#type).into(),
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
            creation_date: require!(cipher.creation_date).parse()?,
            deleted_date: cipher.deleted_date.map(|d| d.parse()).transpose()?,
            revision_date: require!(cipher.revision_date).parse()?,
            key: EncString::try_from_optional(cipher.key)?,
        })
    }
}

impl From<bitwarden_api_api::models::CipherType> for CipherType {
    fn from(t: bitwarden_api_api::models::CipherType) -> Self {
        match t {
            bitwarden_api_api::models::CipherType::Login => CipherType::Login,
            bitwarden_api_api::models::CipherType::SecureNote => CipherType::SecureNote,
            bitwarden_api_api::models::CipherType::Card => CipherType::Card,
            bitwarden_api_api::models::CipherType::Identity => CipherType::Identity,
        }
    }
}

impl From<bitwarden_api_api::models::CipherRepromptType> for CipherRepromptType {
    fn from(t: bitwarden_api_api::models::CipherRepromptType) -> Self {
        match t {
            bitwarden_api_api::models::CipherRepromptType::None => CipherRepromptType::None,
            bitwarden_api_api::models::CipherRepromptType::Password => CipherRepromptType::Password,
        }
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use attachment::AttachmentView;

    use super::*;
    use crate::Fido2Credential;

    fn generate_cipher() -> CipherView {
        CipherView {
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
            name: "My test login".to_string(),
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
        }
    }

    fn generate_fido2(key: &SymmetricCryptoKey) -> Fido2Credential {
        Fido2Credential {
            credential_id: "123".to_string().encrypt_with_key(key).unwrap(),
            key_type: "public-key".to_string().encrypt_with_key(key).unwrap(),
            key_algorithm: "ECDSA".to_string().encrypt_with_key(key).unwrap(),
            key_curve: "P-256".to_string().encrypt_with_key(key).unwrap(),
            key_value: "123".to_string().encrypt_with_key(key).unwrap(),
            rp_id: "123".to_string().encrypt_with_key(key).unwrap(),
            user_handle: None,
            user_name: None,
            counter: "123".to_string().encrypt_with_key(key).unwrap(),
            rp_name: None,
            user_display_name: None,
            discoverable: "true".to_string().encrypt_with_key(key).unwrap(),
            creation_date: "2024-06-07T14:12:36.150Z".parse().unwrap(),
        }
    }

    #[test]
    fn test_decrypt_cipher_list_view() {
        let key: SymmetricCryptoKey = "w2LO+nwV4oxwswVYCxlOfRUseXfvU03VzvKQHrqeklPgiMZrspUe6sOBToCnDn9Ay0tuCBn8ykVVRb7PWhub2Q==".to_string().try_into().unwrap();

        let cipher = Cipher {
            id: Some("090c19ea-a61a-4df6-8963-262b97bc6266".parse().unwrap()),
            organization_id: None,
            folder_id: None,
            collection_ids: vec![],
            key: None,
            name: "2.d3rzo0P8rxV9Hs1m1BmAjw==|JOwna6i0zs+K7ZghwrZRuw==|SJqKreLag1ID+g6H1OdmQr0T5zTrVWKzD6hGy3fDqB0=".parse().unwrap(),
            notes: None,
            r#type: CipherType::Login,
            login: Some(Login {
                username: Some("2.EBNGgnaMHeO/kYnI3A0jiA==|9YXlrgABP71ebZ5umurCJQ==|GDk5jxiqTYaU7e2AStCFGX+a1kgCIk8j0NEli7Jn0L4=".parse().unwrap()),
                password: Some("2.M7ZJ7EuFDXCq66gDTIyRIg==|B1V+jroo6+m/dpHx6g8DxA==|PIXPBCwyJ1ady36a7jbcLg346pm/7N/06W4UZxc1TUo=".parse().unwrap()),
                password_revision_date: None,
                uris: None,
                totp: Some("2.hqdioUAc81FsKQmO1XuLQg==|oDRdsJrQjoFu9NrFVy8tcJBAFKBx95gHaXZnWdXbKpsxWnOr2sKipIG43pKKUFuq|3gKZMiboceIB5SLVOULKg2iuyu6xzos22dfJbvx0EHk=".parse().unwrap()),
                autofill_on_page_load: None,
                fido2_credentials: Some(vec![generate_fido2(&key)]),
            }),
            identity: None,
            card: None,
            secure_note: None,
            favorite: false,
            reprompt: CipherRepromptType::None,
            organization_use_totp: false,
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

        let view: CipherListView = cipher.decrypt_with_key(&key).unwrap();

        assert_eq!(
            view,
            CipherListView {
                id: cipher.id,
                organization_id: cipher.organization_id,
                folder_id: cipher.folder_id,
                collection_ids: cipher.collection_ids,
                key: cipher.key,
                name: "My test login".to_string(),
                sub_title: "test_username".to_string(),
                r#type: CipherListViewType::Login {
                    has_fido2: true,
                    totp: cipher.login.as_ref().unwrap().totp.clone()
                },
                favorite: cipher.favorite,
                reprompt: cipher.reprompt,
                edit: cipher.edit,
                view_password: cipher.view_password,
                attachments: 0,
                creation_date: cipher.creation_date,
                deleted_date: cipher.deleted_date,
                revision_date: cipher.revision_date
            }
        )
    }

    #[test]
    fn test_generate_cipher_key() {
        let key = SymmetricCryptoKey::generate(rand::thread_rng());

        let original_cipher = generate_cipher();

        // Check that the cipher gets encrypted correctly without it's own key
        let cipher = generate_cipher();
        let no_key_cipher_enc = cipher.encrypt_with_key(&key).unwrap();
        let no_key_cipher_dec: CipherView = no_key_cipher_enc.decrypt_with_key(&key).unwrap();
        assert!(no_key_cipher_dec.key.is_none());
        assert_eq!(no_key_cipher_dec.name, original_cipher.name);

        let mut cipher = generate_cipher();
        cipher.generate_cipher_key(&key).unwrap();

        // Check that the cipher gets encrypted correctly when it's assigned it's own key
        let key_cipher_enc = cipher.encrypt_with_key(&key).unwrap();
        let key_cipher_dec: CipherView = key_cipher_enc.decrypt_with_key(&key).unwrap();
        assert!(key_cipher_dec.key.is_some());
        assert_eq!(key_cipher_dec.name, original_cipher.name);
    }

    #[test]
    fn test_generate_cipher_key_when_a_cipher_key_already_exists() {
        let key = SymmetricCryptoKey::generate(rand::thread_rng());

        let cipher_key = SymmetricCryptoKey::generate(rand::thread_rng());
        let cipher_key = cipher_key.to_vec().encrypt_with_key(&key).unwrap();

        let mut original_cipher = generate_cipher();
        original_cipher.key = Some(cipher_key.clone());

        original_cipher.generate_cipher_key(&key).unwrap();

        // Make sure that the cipher key is decryptable
        let _: Vec<u8> = original_cipher.key.unwrap().decrypt_with_key(&key).unwrap();
    }

    #[test]
    fn test_generate_cipher_key_ignores_attachments_without_key() {
        let key = SymmetricCryptoKey::generate(rand::thread_rng());

        let mut cipher = generate_cipher();
        let attachment = AttachmentView {
            id: None,
            url: None,
            size: None,
            size_name: None,
            file_name: Some("Attachment test name".into()),
            key: None,
        };
        cipher.attachments = Some(vec![attachment]);

        cipher.generate_cipher_key(&key).unwrap();
        assert!(cipher.attachments.unwrap()[0].key.is_none());
    }

    struct MockKeyContainer(HashMap<Option<Uuid>, SymmetricCryptoKey>);
    impl KeyContainer for MockKeyContainer {
        fn get_key<'a>(
            &'a self,
            org_id: &Option<Uuid>,
        ) -> Result<&'a SymmetricCryptoKey, CryptoError> {
            self.0
                .get(org_id)
                .ok_or(CryptoError::MissingKey(org_id.unwrap_or_default()))
        }
    }

    #[test]
    fn test_move_user_cipher_to_org() {
        let org = uuid::Uuid::new_v4();

        let enc = MockKeyContainer(HashMap::from([
            (None, SymmetricCryptoKey::generate(rand::thread_rng())),
            (Some(org), SymmetricCryptoKey::generate(rand::thread_rng())),
        ]));

        // Create a cipher with a user key
        let mut cipher = generate_cipher();
        cipher
            .generate_cipher_key(enc.get_key(&None).unwrap())
            .unwrap();

        cipher.move_to_organization(&enc, org).unwrap();
        assert_eq!(cipher.organization_id, Some(org));

        // Check that the cipher can be encrypted/decrypted with the new org key
        let org_key = enc.get_key(&Some(org)).unwrap();
        let cipher_enc = cipher.encrypt_with_key(org_key).unwrap();
        let cipher_dec: CipherView = cipher_enc.decrypt_with_key(org_key).unwrap();

        assert_eq!(cipher_dec.name, "My test login");
    }

    #[test]
    fn test_move_user_cipher_to_org_manually() {
        let org = uuid::Uuid::new_v4();

        let enc = MockKeyContainer(HashMap::from([
            (None, SymmetricCryptoKey::generate(rand::thread_rng())),
            (Some(org), SymmetricCryptoKey::generate(rand::thread_rng())),
        ]));

        // Create a cipher with a user key
        let mut cipher = generate_cipher();
        cipher
            .generate_cipher_key(enc.get_key(&None).unwrap())
            .unwrap();

        cipher.organization_id = Some(org);

        // Check that the cipher can not be encrypted, as the
        // cipher key is tied to the user key and not the org key
        let org_key = enc.get_key(&Some(org)).unwrap();
        assert!(cipher.encrypt_with_key(org_key).is_err());
    }

    #[test]
    fn test_move_user_cipher_with_attachment_without_key_to_org() {
        let org = uuid::Uuid::new_v4();

        let enc = MockKeyContainer(HashMap::from([
            (None, SymmetricCryptoKey::generate(rand::thread_rng())),
            (Some(org), SymmetricCryptoKey::generate(rand::thread_rng())),
        ]));

        let mut cipher = generate_cipher();
        let attachment = AttachmentView {
            id: None,
            url: None,
            size: None,
            size_name: None,
            file_name: Some("Attachment test name".into()),
            key: None,
        };
        cipher.attachments = Some(vec![attachment]);

        // Neither cipher nor attachment have keys, so the cipher can't be moved
        assert!(cipher.move_to_organization(&enc, org).is_err());
    }

    #[test]
    fn test_move_user_cipher_with_attachment_with_key_to_org() {
        let org = uuid::Uuid::new_v4();

        let enc = MockKeyContainer(HashMap::from([
            (None, SymmetricCryptoKey::generate(rand::thread_rng())),
            (Some(org), SymmetricCryptoKey::generate(rand::thread_rng())),
        ]));

        // Attachment has a key that is encrypted with the user key, as the cipher has no key itself
        let attachment_key = SymmetricCryptoKey::generate(rand::thread_rng());
        let attachment_key_enc = attachment_key
            .to_vec()
            .encrypt_with_key(enc.get_key(&None).unwrap())
            .unwrap();

        let mut cipher = generate_cipher();
        let attachment = AttachmentView {
            id: None,
            url: None,
            size: None,
            size_name: None,
            file_name: Some("Attachment test name".into()),
            key: Some(attachment_key_enc),
        };
        cipher.attachments = Some(vec![attachment]);
        let cred = generate_fido2(enc.get_key(&None).unwrap());
        cipher.login.as_mut().unwrap().fido2_credentials = Some(vec![cred]);

        cipher.move_to_organization(&enc, org).unwrap();

        assert!(cipher.key.is_none());

        // Check that the attachment key has been re-encrypted with the org key,
        // and the value matches with the original attachment key
        let new_attachment_key = cipher.attachments.unwrap()[0].key.clone().unwrap();
        let new_attachment_key_dec: Vec<_> = new_attachment_key
            .decrypt_with_key(enc.get_key(&Some(org)).unwrap())
            .unwrap();
        let new_attachment_key_dec: SymmetricCryptoKey = new_attachment_key_dec.try_into().unwrap();
        assert_eq!(new_attachment_key_dec.to_vec(), attachment_key.to_vec());

        let cred2: Fido2CredentialFullView = cipher
            .login
            .unwrap()
            .fido2_credentials
            .unwrap()
            .first()
            .unwrap()
            .decrypt_with_key(enc.get_key(&Some(org)).unwrap())
            .unwrap();

        assert_eq!(cred2.credential_id, "123");
    }

    #[test]
    fn test_move_user_cipher_with_key_with_attachment_with_key_to_org() {
        let org = uuid::Uuid::new_v4();

        let enc = MockKeyContainer(HashMap::from([
            (None, SymmetricCryptoKey::generate(rand::thread_rng())),
            (Some(org), SymmetricCryptoKey::generate(rand::thread_rng())),
        ]));

        let cipher_key = SymmetricCryptoKey::generate(rand::thread_rng());
        let cipher_key_enc = cipher_key
            .to_vec()
            .encrypt_with_key(enc.get_key(&None).unwrap())
            .unwrap();

        // Attachment has a key that is encrypted with the cipher key
        let attachment_key = SymmetricCryptoKey::generate(rand::thread_rng());
        let attachment_key_enc = attachment_key
            .to_vec()
            .encrypt_with_key(&cipher_key)
            .unwrap();

        let mut cipher = generate_cipher();
        cipher.key = Some(cipher_key_enc);

        let attachment = AttachmentView {
            id: None,
            url: None,
            size: None,
            size_name: None,
            file_name: Some("Attachment test name".into()),
            key: Some(attachment_key_enc.clone()),
        };
        cipher.attachments = Some(vec![attachment]);

        let cred = generate_fido2(&cipher_key);
        cipher.login.as_mut().unwrap().fido2_credentials = Some(vec![cred.clone()]);

        cipher.move_to_organization(&enc, org).unwrap();

        // Check that the cipher key has been re-encrypted with the org key,
        let new_cipher_key_dec: Vec<_> = cipher
            .key
            .clone()
            .unwrap()
            .decrypt_with_key(enc.get_key(&Some(org)).unwrap())
            .unwrap();

        let new_cipher_key_dec: SymmetricCryptoKey = new_cipher_key_dec.try_into().unwrap();

        assert_eq!(new_cipher_key_dec.to_vec(), cipher_key.to_vec());

        // Check that the attachment key hasn't changed
        assert_eq!(
            cipher.attachments.unwrap()[0]
                .key
                .as_ref()
                .unwrap()
                .to_string(),
            attachment_key_enc.to_string()
        );

        let cred2: Fido2Credential = cipher
            .login
            .unwrap()
            .fido2_credentials
            .unwrap()
            .first()
            .unwrap()
            .clone();

        assert_eq!(
            cred2.credential_id.to_string(),
            cred.credential_id.to_string()
        );
    }

    #[test]
    fn test_build_subtitle_card_visa() {
        let brand = Some("Visa".to_owned());
        let number = Some("4111111111111111".to_owned());

        let subtitle = build_subtitle_card(brand, number);
        assert_eq!(subtitle, "Visa, *1111");
    }

    #[test]
    fn test_build_subtitle_card_mastercard() {
        let brand = Some("Mastercard".to_owned());
        let number = Some("5555555555554444".to_owned());

        let subtitle = build_subtitle_card(brand, number);
        assert_eq!(subtitle, "Mastercard, *4444");
    }

    #[test]
    fn test_build_subtitle_card_amex() {
        let brand = Some("Amex".to_owned());
        let number = Some("378282246310005".to_owned());

        let subtitle = build_subtitle_card(brand, number);
        assert_eq!(subtitle, "Amex, *10005");
    }

    #[test]
    fn test_build_subtitle_card_underflow() {
        let brand = Some("Mastercard".to_owned());
        let number = Some("4".to_owned());

        let subtitle = build_subtitle_card(brand, number);
        assert_eq!(subtitle, "Mastercard");
    }

    #[test]
    fn test_build_subtitle_card_only_brand() {
        let brand = Some("Mastercard".to_owned());
        let number = None;

        let subtitle = build_subtitle_card(brand, number);
        assert_eq!(subtitle, "Mastercard");
    }

    #[test]
    fn test_build_subtitle_card_only_card() {
        let brand = None;
        let number = Some("5555555555554444".to_owned());

        let subtitle = build_subtitle_card(brand, number);
        assert_eq!(subtitle, "*4444");
    }

    #[test]
    fn test_build_subtitle_identity() {
        let first_name = Some("John".to_owned());
        let last_name = Some("Doe".to_owned());

        let subtitle = build_subtitle_identity(first_name, last_name);
        assert_eq!(subtitle, "John Doe");
    }

    #[test]
    fn test_build_subtitle_identity_only_first() {
        let first_name = Some("John".to_owned());
        let last_name = None;

        let subtitle = build_subtitle_identity(first_name, last_name);
        assert_eq!(subtitle, "John");
    }

    #[test]
    fn test_build_subtitle_identity_only_last() {
        let first_name = None;
        let last_name = Some("Doe".to_owned());

        let subtitle = build_subtitle_identity(first_name, last_name);
        assert_eq!(subtitle, "Doe");
    }

    #[test]
    fn test_build_subtitle_identity_none() {
        let first_name = None;
        let last_name = None;

        let subtitle = build_subtitle_identity(first_name, last_name);
        assert_eq!(subtitle, "");
    }
}
