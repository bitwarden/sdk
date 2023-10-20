use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    client::encryption_settings::EncryptionSettings,
    crypto::{Decryptable, EncString, Encryptable},
    error::{Error, Result},
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct Attachment {
    pub id: Option<String>,
    pub url: Option<String>,
    pub size: Option<String>,
    /// Readable size, ex: "4.2 KB" or "1.43 GB"
    pub size_name: Option<String>,
    pub file_name: Option<EncString>,
    pub key: Option<EncString>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct AttachmentView {
    pub id: Option<String>,
    pub url: Option<String>,
    pub size: Option<String>,
    pub size_name: Option<String>,
    pub file_name: Option<String>,
    pub key: Option<Vec<u8>>, // TODO: Should be made into SymmetricCryptoKey
}

impl Encryptable<Attachment> for AttachmentView {
    fn encrypt(self, enc: &EncryptionSettings, org_id: &Option<Uuid>) -> Result<Attachment> {
        Ok(Attachment {
            id: self.id,
            url: self.url,
            size: self.size,
            size_name: self.size_name,
            file_name: self.file_name.encrypt(enc, org_id)?,
            key: self.key.map(|k| k.encrypt(enc, org_id)).transpose()?,
        })
    }
}

impl Decryptable<AttachmentView> for Attachment {
    fn decrypt(&self, enc: &EncryptionSettings, org_id: &Option<Uuid>) -> Result<AttachmentView> {
        Ok(AttachmentView {
            id: self.id.clone(),
            url: self.url.clone(),
            size: self.size.clone(),
            size_name: self.size_name.clone(),
            file_name: self.file_name.decrypt(enc, org_id)?,
            key: self
                .key
                .as_ref()
                .map(|key| enc.decrypt_bytes(key, org_id))
                .transpose()?,
        })
    }
}

impl TryFrom<bitwarden_api_api::models::AttachmentResponseModel> for Attachment {
    type Error = Error;

    fn try_from(attachment: bitwarden_api_api::models::AttachmentResponseModel) -> Result<Self> {
        Ok(Self {
            id: attachment.id,
            url: attachment.url,
            size: attachment.size.map(|s| s.to_string()),
            size_name: attachment.size_name,
            file_name: EncString::try_from(attachment.file_name)?,
            key: EncString::try_from(attachment.key)?,
        })
    }
}
