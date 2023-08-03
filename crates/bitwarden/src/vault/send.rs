use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use uuid::Uuid;

use crate::{
    client::encryption_settings::EncryptionSettings,
    crypto::{stretch_key, CipherString, Decryptable},
    error::Result,
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct SendFile {
    pub id: String,
    pub file_name: CipherString,
    pub size: String,
    /// Readable size, ex: "4.2 KB" or "1.43 GB"
    pub size_name: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct SendFileView {
    pub id: String,
    pub file_name: String,
    pub size: String,
    /// Readable size, ex: "4.2 KB" or "1.43 GB"
    pub size_name: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct SendText {
    pub text: CipherString,
    pub hidden: bool,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct SendTextView {
    pub text: String,
    pub hidden: bool,
}

#[derive(Clone, Copy, Serialize_repr, Deserialize_repr, Debug, JsonSchema)]
#[repr(u8)]
#[cfg_attr(feature = "mobile", derive(uniffi::Enum))]
pub enum SendType {
    Text = 0,
    File = 1,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct Send {
    pub id: Uuid,
    pub access_id: String,

    pub name: CipherString,
    pub notes: Option<CipherString>,
    pub key: CipherString,
    pub password: Option<String>,

    pub r#type: SendType,
    pub file: Option<SendFile>,
    pub text: Option<SendText>,

    pub max_access_count: Option<u32>,
    pub access_count: u32,
    pub disabled: bool,
    pub hide_email: bool,

    pub revision_date: DateTime<Utc>,
    pub deletion_date: DateTime<Utc>,
    pub expiration_date: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct SendView {
    pub id: Uuid,
    pub access_id: String,

    pub name: String,
    pub notes: Option<String>,
    pub key: CipherString,
    pub password: Option<String>,

    pub r#type: SendType,
    pub file: Option<SendFileView>,
    pub text: Option<SendTextView>,

    pub max_access_count: Option<u32>,
    pub access_count: u32,
    pub disabled: bool,
    pub hide_email: bool,

    pub revision_date: DateTime<Utc>,
    pub deletion_date: DateTime<Utc>,
    pub expiration_date: Option<DateTime<Utc>>,
}

impl Send {
    pub(crate) fn get_send_key(
        &self,
        enc: &EncryptionSettings,
        org_id: &Option<Uuid>,
    ) -> Result<EncryptionSettings> {
        let key: Vec<u8> = enc.decrypt_bytes(&self.key, org_id)?;
        let key = stretch_key(key.try_into().unwrap(), "send", Some("send"));
        Ok(EncryptionSettings::new_single_key(key))
    }
}

impl Decryptable<SendTextView> for SendText {
    fn decrypt(&self, enc: &EncryptionSettings, org_id: &Option<Uuid>) -> Result<SendTextView> {
        Ok(SendTextView {
            text: self.text.decrypt(enc, org_id)?,
            hidden: self.hidden,
        })
    }
}

impl Decryptable<SendFileView> for SendFile {
    fn decrypt(&self, enc: &EncryptionSettings, org_id: &Option<Uuid>) -> Result<SendFileView> {
        Ok(SendFileView {
            id: self.id.clone(),
            file_name: self.file_name.decrypt(enc, org_id)?,
            size: self.size.clone(),
            size_name: self.size_name.clone(),
        })
    }
}

impl Decryptable<SendView> for Send {
    fn decrypt(&self, enc: &EncryptionSettings, org_id: &Option<Uuid>) -> Result<SendView> {
        // For sends, we first decrypt the send key with the user key, and stretch it to it's full size
        let enc_owned = self.get_send_key(enc, org_id)?;

        // For the rest of the fields, we ignore the provided EncryptionSettings and use a new one with the stretched key
        let enc = &enc_owned;

        Ok(SendView {
            id: self.id,
            access_id: self.access_id.clone(),

            name: self.name.decrypt(enc, org_id)?,
            notes: self.notes.decrypt(enc, org_id)?,
            key: self.key.clone(),
            password: self.password.clone(),

            r#type: self.r#type,
            file: self.file.decrypt(enc, org_id)?,
            text: self.text.decrypt(enc, org_id)?,

            max_access_count: self.max_access_count,
            access_count: self.access_count,
            disabled: self.disabled,
            hide_email: self.hide_email,

            revision_date: self.revision_date,
            deletion_date: self.deletion_date,
            expiration_date: self.expiration_date,
        })
    }
}
