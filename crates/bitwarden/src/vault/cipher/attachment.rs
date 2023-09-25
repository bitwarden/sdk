use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
    str::FromStr,
};

use bitwarden_api_api::apis::ciphers_api::ciphers_id_attachment_attachment_id_get;
use log::debug;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    client::encryption_settings::EncryptionSettings,
    crypto::{Decryptable, EncString, Encryptable, SymmetricCryptoKey},
    error::Result,
    vault::api::attachment_get,
    Client,
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

pub async fn download_attachment(
    client: &mut Client,
    cipher_id: Uuid,
    attachment_id: &str,
) -> Result<Vec<u8>> {
    // The attachments from sync doesn't contain the correct url
    let configuration = &client.get_api_configurations().await.api;
    let response = ciphers_id_attachment_attachment_id_get(
        configuration,
        cipher_id.to_string().as_str(),
        attachment_id.to_string().as_str(),
    )
    .await?;

    let attachment: Attachment = response.into();
    let enc = client.get_encryption_settings()?;
    let view = attachment.decrypt(enc, &None)?;

    let key = SymmetricCryptoKey::try_from(view.key.unwrap().as_slice())?;
    let enc = EncryptionSettings::new_single_key(key);

    let response = attachment_get(&view.url.unwrap()).await?;
    let bytes = response.bytes().await?;

    let buf = EncString::from_buffer(&bytes)?;
    let dec = enc.decrypt_bytes(&buf, &None)?;

    let path = Path::new("attachments")
        .join(cipher_id.to_string())
        .join(attachment_id)
        .join(view.file_name.unwrap());

    create_dir_all(path.parent().unwrap())?;
    let mut file = File::create(path)?;
    file.write_all(&dec)?;

    debug!("{:?}", bytes.len());

    todo!()
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
            file_name: self.file_name.decrypt(enc, org_id).unwrap(),
            key: self
                .key
                .as_ref()
                .map(|key| enc.decrypt_bytes(key, org_id).unwrap()),
        })
    }
}

impl From<bitwarden_api_api::models::AttachmentResponseModel> for Attachment {
    fn from(attachment: bitwarden_api_api::models::AttachmentResponseModel) -> Self {
        debug!("{:?}", attachment);
        Self {
            id: attachment.id,
            url: attachment.url,
            size: attachment.size.map(|s| s.to_string()),
            size_name: attachment.size_name,
            file_name: attachment
                .file_name
                .map(|s| EncString::from_str(&s).unwrap()),
            key: attachment.key.map(|s| EncString::from_str(&s).unwrap()),
        }
    }
}
