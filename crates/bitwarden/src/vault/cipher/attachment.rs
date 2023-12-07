use bitwarden_crypto::{EncString, KeyDecryptable, KeyEncryptable, SymmetricCryptoKey};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
    pub key: Option<String>,
}

impl KeyEncryptable<Attachment> for AttachmentView {
    fn encrypt_with_key(self, key: &SymmetricCryptoKey) -> bitwarden_crypto::Result<Attachment> {
        Ok(Attachment {
            id: self.id,
            url: self.url,
            size: self.size,
            size_name: self.size_name,
            file_name: self.file_name.encrypt_with_key(key)?,
            key: self.key.encrypt_with_key(key)?,
        })
    }
}

impl KeyDecryptable<AttachmentView> for Attachment {
    fn decrypt_with_key(
        &self,
        key: &SymmetricCryptoKey,
    ) -> bitwarden_crypto::Result<AttachmentView> {
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
