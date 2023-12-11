use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    crypto::{EncString, KeyDecryptable, KeyEncryptable, SymmetricCryptoKey},
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

impl KeyEncryptable<Attachment> for AttachmentView {
    fn encrypt_with_key(self, key: &SymmetricCryptoKey) -> Result<Attachment> {
        Ok(Attachment {
            id: self.id,
            url: self.url,
            size: self.size,
            size_name: self.size_name,
            file_name: self.file_name.encrypt_with_key(key)?,
            key: self.key.map(|k| k.encrypt_with_key(key)).transpose()?,
        })
    }
}

impl KeyDecryptable<AttachmentView> for Attachment {
    fn decrypt_with_key(&self, key: &SymmetricCryptoKey) -> Result<AttachmentView> {
        Ok(AttachmentView {
            id: self.id.clone(),
            url: self.url.clone(),
            size: self.size.clone(),
            size_name: self.size_name.clone(),
            file_name: self.file_name.decrypt_with_key(key)?,
            key: self.key.decrypt_with_key(key)?,
        })
    }
}

impl TryFrom<bitwarden_api_api::models::AttachmentResponseModel> for Attachment {
    type Error = Error;

    fn try_from(attachment: bitwarden_api_api::models::AttachmentResponseModel) -> Result<Self> {
        Ok(Self {
            id: attachment.id,
            url: attachment.url,
            size: attachment.size,
            size_name: attachment.size_name,
            file_name: EncString::try_from(attachment.file_name)?,
            key: EncString::try_from(attachment.key)?,
        })
    }
}
