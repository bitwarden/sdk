use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
};

use bitwarden_api_api::apis::ciphers_api::ciphers_id_attachment_attachment_id_get;
use bitwarden_crypto::{
    CryptoError, EncString, KeyDecryptable, KeyEncryptable, SymmetricCryptoKey,
};
use log::debug;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Cipher;
use crate::{
    error::{Error, Result},
    vault::api::attachment_get,
    Client,
};

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
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
    pub key: Option<EncString>,
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
        cipher_id,
        attachment_id.to_string().as_str(),
    )
    .await?;

    let attachment: Attachment = response.try_into()?;
    let enc = client.get_encryption_settings()?;
    let k = enc.get_key(&None).ok_or(Error::VaultLocked)?;

    let view = attachment.decrypt_with_key(k)?;
    let mut cipher_key: Vec<u8> = view.key.unwrap().decrypt_with_key(k)?;

    let key = SymmetricCryptoKey::try_from(cipher_key.as_mut_slice())?;

    let response = attachment_get(&view.url.unwrap()).await?;
    let bytes = response.bytes().await?;

    let buf = EncString::from_buffer(&bytes)?;
    let dec: Vec<u8> = buf.decrypt_with_key(&key)?;

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

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct AttachmentEncryptResult {
    pub attachment: Attachment,
    pub contents: Vec<u8>,
}

pub struct AttachmentFile {
    pub cipher: Cipher,
    pub attachment: Attachment,
    pub contents: EncString,
}

pub struct AttachmentFileView<'a> {
    pub cipher: Cipher,
    pub attachment: AttachmentView,
    pub contents: &'a [u8],
}

impl<'a> KeyEncryptable<SymmetricCryptoKey, AttachmentEncryptResult> for AttachmentFileView<'a> {
    fn encrypt_with_key(
        self,
        key: &SymmetricCryptoKey,
    ) -> Result<AttachmentEncryptResult, CryptoError> {
        let ciphers_key = Cipher::get_cipher_key(key, &self.cipher.key)?;
        let ciphers_key = ciphers_key.as_ref().unwrap_or(key);

        let mut attachment = self.attachment;

        // Because this is a new attachment, we have to generate a key for it, encrypt the contents
        // with it, and then encrypt the key with the cipher key
        let attachment_key = SymmetricCryptoKey::generate(rand::thread_rng());
        let encrypted_contents = self.contents.encrypt_with_key(&attachment_key)?;
        attachment.key = Some(attachment_key.to_vec().encrypt_with_key(ciphers_key)?);

        Ok(AttachmentEncryptResult {
            attachment: attachment.encrypt_with_key(ciphers_key)?,
            contents: encrypted_contents.to_buffer()?,
        })
    }
}

impl KeyDecryptable<SymmetricCryptoKey, Vec<u8>> for AttachmentFile {
    fn decrypt_with_key(&self, key: &SymmetricCryptoKey) -> Result<Vec<u8>, CryptoError> {
        let ciphers_key = Cipher::get_cipher_key(key, &self.cipher.key)?;
        let ciphers_key = ciphers_key.as_ref().unwrap_or(key);

        let mut attachment_key: Vec<u8> = self
            .attachment
            .key
            .as_ref()
            .ok_or(CryptoError::MissingKey)?
            .decrypt_with_key(ciphers_key)?;
        let attachment_key = SymmetricCryptoKey::try_from(attachment_key.as_mut_slice())?;

        self.contents.decrypt_with_key(&attachment_key)
    }
}

impl KeyEncryptable<SymmetricCryptoKey, Attachment> for AttachmentView {
    fn encrypt_with_key(self, key: &SymmetricCryptoKey) -> Result<Attachment, CryptoError> {
        Ok(Attachment {
            id: self.id,
            url: self.url,
            size: self.size,
            size_name: self.size_name,
            file_name: self.file_name.encrypt_with_key(key)?,
            key: self.key,
        })
    }
}

impl KeyDecryptable<SymmetricCryptoKey, AttachmentView> for Attachment {
    fn decrypt_with_key(&self, key: &SymmetricCryptoKey) -> Result<AttachmentView, CryptoError> {
        Ok(AttachmentView {
            id: self.id.clone(),
            url: self.url.clone(),
            size: self.size.clone(),
            size_name: self.size_name.clone(),
            file_name: self.file_name.decrypt_with_key(key)?,
            key: self.key.clone(),
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
            file_name: EncString::try_from_optional(attachment.file_name)?,
            key: EncString::try_from_optional(attachment.key)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use base64::{engine::general_purpose::STANDARD, Engine};
    use bitwarden_crypto::{EncString, KeyDecryptable, SymmetricCryptoKey};

    use crate::vault::{
        cipher::cipher::{CipherRepromptType, CipherType},
        Attachment, AttachmentFile, Cipher,
    };

    #[test]
    fn test_attachment_key() {
        let user_key : SymmetricCryptoKey = "w2LO+nwV4oxwswVYCxlOfRUseXfvU03VzvKQHrqeklPgiMZrspUe6sOBToCnDn9Ay0tuCBn8ykVVRb7PWhub2Q==".parse().unwrap();

        let attachment = Attachment {
            id: None,
            url: None,
            size: Some("161".into()),
            size_name: Some("161 Bytes".into()),
            file_name: Some("2.M3z1MOO9eBG9BWRTEUbPog==|jPw0By1AakHDfoaY8UOwOQ==|eP9/J1583OJpHsSM4ZnXZzdBHfqVTXnOXGlkkmAKSfA=".parse().unwrap()),
            key: Some("2.r288/AOSPiaLFkW07EBGBw==|SAmnnCbOLFjX5lnURvoualOetQwuyPc54PAmHDTRrhT0gwO9ailna9U09q9bmBfI5XrjNNEsuXssgzNygRkezoVQvZQggZddOwHB6KQW5EQ=|erIMUJp8j+aTcmhdE50zEX+ipv/eR1sZ7EwULJm/6DY=".parse().unwrap())
        };

        let cipher  = Cipher {
            id: None,
            organization_id: None,
            folder_id: None,
            collection_ids: Vec::new(),
            key: Some("2.Gg8yCM4IIgykCZyq0O4+cA==|GJLBtfvSJTDJh/F7X4cJPkzI6ccnzJm5DYl3yxOW2iUn7DgkkmzoOe61sUhC5dgVdV0kFqsZPcQ0yehlN1DDsFIFtrb4x7LwzJNIkMgxNyg=|1rGkGJ8zcM5o5D0aIIwAyLsjMLrPsP3EWm3CctBO3Fw=".parse().unwrap()),
            name: "2.d24xECyEdMZ3MG9s6SrGNw==|XvJlTeu5KJ22M3jKosy6iw==|8xGiQty4X61cDMx6PVqkJfSQ0ZTdA/5L9TpG7QfovoM=".parse().unwrap(),
            notes: None,
            r#type: CipherType::Login,
            login: None,
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
            creation_date: "2023-07-24T12:05:09.466666700Z".parse().unwrap(),
            deleted_date: None,
            revision_date: "2023-07-27T19:28:05.240Z".parse().unwrap(),
        };

        let enc_file = STANDARD.decode(b"Ao00qr1xLsV+ZNQpYZ/UwEwOWo3hheKwCYcOGIbsorZ6JIG2vLWfWEXCVqP0hDuzRvmx8otApNZr8pJYLNwCe1aQ+ySHQYGkdubFjoMojulMbQ959Y4SJ6Its/EnVvpbDnxpXTDpbutDxyhxfq1P3lstL2G9rObJRrxiwdGlRGu1h94UA1fCCkIUQux5LcqUee6W4MyQmRnsUziH8gGzmtI=").unwrap();
        let original = STANDARD.decode(b"rMweTemxOL9D0iWWfRxiY3enxiZ5IrwWD6ef2apGO6MvgdGhy2fpwmATmn7BpSj9lRumddLLXm7u8zSp6hnXt1hS71YDNh78LjGKGhGL4sbg8uNnpa/I6GK/83jzqGYN7+ESbg==").unwrap();

        let dec = AttachmentFile {
            cipher,
            attachment,
            contents: EncString::from_buffer(&enc_file).unwrap(),
        }
        .decrypt_with_key(&user_key)
        .unwrap();

        assert_eq!(dec, original);
    }
}
