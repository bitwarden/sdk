use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine,
};
use bitwarden_api_api::models::{SendFileModel, SendResponseModel, SendTextModel};
use bitwarden_crypto::{
    derive_shareable_key, generate_random_bytes, CryptoError, EncString, KeyDecryptable,
    KeyEncryptable, LocateKey, SymmetricCryptoKey,
};
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use uuid::Uuid;

use crate::error::{Error, Result};

const SEND_ITERATIONS: u32 = 100_000;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct SendFile {
    pub id: Option<String>,
    pub file_name: EncString,
    pub size: Option<String>,
    /// Readable size, ex: "4.2 KB" or "1.43 GB"
    pub size_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct SendFileView {
    pub id: Option<String>,
    pub file_name: String,
    pub size: Option<String>,
    /// Readable size, ex: "4.2 KB" or "1.43 GB"
    pub size_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct SendText {
    pub text: Option<EncString>,
    pub hidden: bool,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct SendTextView {
    pub text: Option<String>,
    pub hidden: bool,
}

#[derive(Clone, Copy, Serialize_repr, Deserialize_repr, Debug, JsonSchema, PartialEq)]
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
    pub id: Option<Uuid>,
    pub access_id: Option<String>,

    pub name: EncString,
    pub notes: Option<EncString>,
    pub key: EncString,
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

#[derive(Serialize, Deserialize, Debug, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct SendView {
    pub id: Option<Uuid>,
    pub access_id: Option<String>,

    pub name: String,
    pub notes: Option<String>,
    /// Base64 encoded key
    pub key: Option<String>,
    /// Replace or add a password to an existing send. The SDK will always return None when
    /// decrypting a [Send]
    /// TODO: We should revisit this, one variant is to have `[Create, Update]SendView` DTOs.
    pub new_password: Option<String>,
    /// Denote if an existing send has a password. The SDK will ignore this value when creating or
    /// updating sends.
    pub has_password: bool,

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

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct SendListView {
    pub id: Option<Uuid>,
    pub access_id: Option<String>,

    pub name: String,

    pub r#type: SendType,
    pub disabled: bool,

    pub revision_date: DateTime<Utc>,
    pub deletion_date: DateTime<Utc>,
    pub expiration_date: Option<DateTime<Utc>>,
}

impl Send {
    pub(crate) fn get_key(
        send_key: &EncString,
        enc_key: &SymmetricCryptoKey,
    ) -> Result<SymmetricCryptoKey, CryptoError> {
        let key: Vec<u8> = send_key.decrypt_with_key(enc_key)?;
        Self::derive_shareable_key(&key)
    }

    fn derive_shareable_key(key: &[u8]) -> Result<SymmetricCryptoKey, CryptoError> {
        let key = key.try_into().map_err(|_| CryptoError::InvalidKeyLen)?;
        Ok(derive_shareable_key(key, "send", Some("send")))
    }
}

impl KeyDecryptable<SymmetricCryptoKey, SendTextView> for SendText {
    fn decrypt_with_key(&self, key: &SymmetricCryptoKey) -> Result<SendTextView, CryptoError> {
        Ok(SendTextView {
            text: self.text.decrypt_with_key(key)?,
            hidden: self.hidden,
        })
    }
}

impl KeyEncryptable<SymmetricCryptoKey, SendText> for SendTextView {
    fn encrypt_with_key(self, key: &SymmetricCryptoKey) -> Result<SendText, CryptoError> {
        Ok(SendText {
            text: self.text.encrypt_with_key(key)?,
            hidden: self.hidden,
        })
    }
}

impl KeyDecryptable<SymmetricCryptoKey, SendFileView> for SendFile {
    fn decrypt_with_key(&self, key: &SymmetricCryptoKey) -> Result<SendFileView, CryptoError> {
        Ok(SendFileView {
            id: self.id.clone(),
            file_name: self.file_name.decrypt_with_key(key)?,
            size: self.size.clone(),
            size_name: self.size_name.clone(),
        })
    }
}

impl KeyEncryptable<SymmetricCryptoKey, SendFile> for SendFileView {
    fn encrypt_with_key(self, key: &SymmetricCryptoKey) -> Result<SendFile, CryptoError> {
        Ok(SendFile {
            id: self.id.clone(),
            file_name: self.file_name.encrypt_with_key(key)?,
            size: self.size.clone(),
            size_name: self.size_name.clone(),
        })
    }
}

impl LocateKey for Send {}
impl KeyDecryptable<SymmetricCryptoKey, SendView> for Send {
    fn decrypt_with_key(&self, key: &SymmetricCryptoKey) -> Result<SendView, CryptoError> {
        // For sends, we first decrypt the send key with the user key, and stretch it to it's full
        // size For the rest of the fields, we ignore the provided SymmetricCryptoKey and
        // the stretched key
        let k: Vec<u8> = self.key.decrypt_with_key(key)?;
        let key = Send::derive_shareable_key(&k)?;

        Ok(SendView {
            id: self.id,
            access_id: self.access_id.clone(),

            name: self.name.decrypt_with_key(&key).ok().unwrap_or_default(),
            notes: self.notes.decrypt_with_key(&key).ok().flatten(),
            key: Some(URL_SAFE_NO_PAD.encode(k)),
            new_password: None,
            has_password: self.password.is_some(),

            r#type: self.r#type,
            file: self.file.decrypt_with_key(&key).ok().flatten(),
            text: self.text.decrypt_with_key(&key).ok().flatten(),

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

impl KeyDecryptable<SymmetricCryptoKey, SendListView> for Send {
    fn decrypt_with_key(&self, key: &SymmetricCryptoKey) -> Result<SendListView, CryptoError> {
        // For sends, we first decrypt the send key with the user key, and stretch it to it's full
        // size For the rest of the fields, we ignore the provided SymmetricCryptoKey and
        // the stretched key
        let key = Send::get_key(&self.key, key)?;

        Ok(SendListView {
            id: self.id,
            access_id: self.access_id.clone(),

            name: self.name.decrypt_with_key(&key)?,
            r#type: self.r#type,

            disabled: self.disabled,

            revision_date: self.revision_date,
            deletion_date: self.deletion_date,
            expiration_date: self.expiration_date,
        })
    }
}

impl LocateKey for SendView {}
impl KeyEncryptable<SymmetricCryptoKey, Send> for SendView {
    fn encrypt_with_key(self, key: &SymmetricCryptoKey) -> Result<Send, CryptoError> {
        // For sends, we first decrypt the send key with the user key, and stretch it to it's full
        // size For the rest of the fields, we ignore the provided SymmetricCryptoKey and
        // the stretched key
        let k = match (self.key, self.id) {
            // Existing send, decrypt key
            (Some(k), _) => URL_SAFE_NO_PAD
                .decode(k)
                .map_err(|_| CryptoError::InvalidKey)?,
            // New send, generate random key
            (None, None) => {
                let key: [u8; 16] = generate_random_bytes();
                key.to_vec()
            }
            // Existing send without key
            _ => return Err(CryptoError::InvalidKey),
        };
        let send_key = Send::derive_shareable_key(&k)?;

        Ok(Send {
            id: self.id,
            access_id: self.access_id,

            name: self.name.encrypt_with_key(&send_key)?,
            notes: self.notes.encrypt_with_key(&send_key)?,
            key: k.encrypt_with_key(key)?,
            password: self.new_password.map(|password| {
                let password = bitwarden_crypto::pbkdf2(password.as_bytes(), &k, SEND_ITERATIONS);
                STANDARD.encode(password)
            }),

            r#type: self.r#type,
            file: self.file.encrypt_with_key(&send_key)?,
            text: self.text.encrypt_with_key(&send_key)?,

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

impl TryFrom<SendResponseModel> for Send {
    type Error = Error;

    fn try_from(send: SendResponseModel) -> Result<Self> {
        Ok(Send {
            id: send.id,
            access_id: send.access_id,
            name: send.name.ok_or(Error::MissingFields)?.parse()?,
            notes: EncString::try_from_optional(send.notes)?,
            key: send.key.ok_or(Error::MissingFields)?.parse()?,
            password: send.password,
            r#type: send.r#type.ok_or(Error::MissingFields)?.into(),
            file: send.file.map(|f| (*f).try_into()).transpose()?,
            text: send.text.map(|t| (*t).try_into()).transpose()?,
            max_access_count: send.max_access_count.map(|s| s as u32),
            access_count: send.access_count.ok_or(Error::MissingFields)? as u32,
            disabled: send.disabled.unwrap_or(false),
            hide_email: send.hide_email.unwrap_or(false),
            revision_date: send.revision_date.ok_or(Error::MissingFields)?.parse()?,
            deletion_date: send.deletion_date.ok_or(Error::MissingFields)?.parse()?,
            expiration_date: send.expiration_date.map(|s| s.parse()).transpose()?,
        })
    }
}

impl From<bitwarden_api_api::models::SendType> for SendType {
    fn from(t: bitwarden_api_api::models::SendType) -> Self {
        match t {
            bitwarden_api_api::models::SendType::Variant0 => SendType::Text,
            bitwarden_api_api::models::SendType::Variant1 => SendType::File,
        }
    }
}

impl TryFrom<SendFileModel> for SendFile {
    type Error = Error;

    fn try_from(file: SendFileModel) -> Result<Self> {
        Ok(SendFile {
            id: file.id,
            file_name: file.file_name.ok_or(Error::MissingFields)?.parse()?,
            size: file.size.map(|v| v.to_string()),
            size_name: file.size_name,
        })
    }
}

impl TryFrom<SendTextModel> for SendText {
    type Error = Error;

    fn try_from(text: SendTextModel) -> Result<Self> {
        Ok(SendText {
            text: EncString::try_from_optional(text.text)?,
            hidden: text.hidden.unwrap_or(false),
        })
    }
}

#[cfg(test)]
mod tests {
    use bitwarden_crypto::{KeyDecryptable, KeyEncryptable};

    use super::{Send, SendText, SendTextView, SendType};
    use crate::{
        client::{encryption_settings::EncryptionSettings, Kdf, UserLoginMethod},
        vault::SendView,
    };

    #[test]
    fn test_get_send_key() {
        // Initialize user encryption with some test data
        let enc = EncryptionSettings::new(
            &UserLoginMethod::Username {
                client_id: "test".into(),
                email: "test@bitwarden.com".into(),
                kdf: Kdf::PBKDF2 {
                    iterations: 345123.try_into().unwrap(),
                },
            },
            "asdfasdfasdf",
            "2.majkL1/hNz9yptLqNAUSnw==|RiOzMTTJMG948qu8O3Zm1EQUO2E8BuTwFKnO9LWQjMzxMWJM5GbyOq2/A+tumPbTERt4JWur/FKfgHb+gXuYiEYlXPMuVBvT7nv4LPytJuM=|IVqMxHJeR1ZXY0sGngTC0x+WqbG8p6V+BTrdgBbQXjM=".parse().unwrap(),
            "2.kmLY8NJVuiKBFJtNd/ZFpA==|qOodlRXER+9ogCe3yOibRHmUcSNvjSKhdDuztLlucs10jLiNoVVVAc+9KfNErLSpx5wmUF1hBOJM8zwVPjgQTrmnNf/wuDpwiaCxNYb/0v4FygPy7ccAHK94xP1lfqq7U9+tv+/yiZSwgcT+xF0wFpoxQeNdNRFzPTuD9o4134n8bzacD9DV/WjcrXfRjbBCzzuUGj1e78+A7BWN7/5IWLz87KWk8G7O/W4+8PtEzlwkru6Wd1xO19GYU18oArCWCNoegSmcGn7w7NDEXlwD403oY8Oa7ylnbqGE28PVJx+HLPNIdSC6YKXeIOMnVs7Mctd/wXC93zGxAWD6ooTCzHSPVV50zKJmWIG2cVVUS7j35H3rGDtUHLI+ASXMEux9REZB8CdVOZMzp2wYeiOpggebJy6MKOZqPT1R3X0fqF2dHtRFPXrNsVr1Qt6bS9qTyO4ag1/BCvXF3P1uJEsI812BFAne3cYHy5bIOxuozPfipJrTb5WH35bxhElqwT3y/o/6JWOGg3HLDun31YmiZ2HScAsUAcEkA4hhoTNnqy4O2s3yVbCcR7jF7NLsbQc0MDTbnjxTdI4VnqUIn8s2c9hIJy/j80pmO9Bjxp+LQ9a2hUkfHgFhgHxZUVaeGVth8zG2kkgGdrp5VHhxMVFfvB26Ka6q6qE/UcS2lONSv+4T8niVRJz57qwctj8MNOkA3PTEfe/DP/LKMefke31YfT0xogHsLhDkx+mS8FCc01HReTjKLktk/Jh9mXwC5oKwueWWwlxI935ecn+3I2kAuOfMsgPLkoEBlwgiREC1pM7VVX1x8WmzIQVQTHd4iwnX96QewYckGRfNYWz/zwvWnjWlfcg8kRSe+68EHOGeRtC5r27fWLqRc0HNcjwpgHkI/b6czerCe8+07TWql4keJxJxhBYj3iOH7r9ZS8ck51XnOb8tGL1isimAJXodYGzakwktqHAD7MZhS+P02O+6jrg7d+yPC2ZCuS/3TOplYOCHQIhnZtR87PXTUwr83zfOwAwCyv6KP84JUQ45+DItrXLap7nOVZKQ5QxYIlbThAO6eima6Zu5XHfqGPMNWv0bLf5+vAjIa5np5DJrSwz9no/hj6CUh0iyI+SJq4RGI60lKtypMvF6MR3nHLEHOycRUQbZIyTHWl4QQLdHzuwN9lv10ouTEvNr6sFflAX2yb6w3hlCo7oBytH3rJekjb3IIOzBpeTPIejxzVlh0N9OT5MZdh4sNKYHUoWJ8mnfjdM+L4j5Q2Kgk/XiGDgEebkUxiEOQUdVpePF5uSCE+TPav/9FIRGXGiFn6NJMaU7aBsDTFBLloffFLYDpd8/bTwoSvifkj7buwLYM+h/qcnfdy5FWau1cKav+Blq/ZC0qBpo658RTC8ZtseAFDgXoQZuksM10hpP9bzD04Bx30xTGX81QbaSTNwSEEVrOtIhbDrj9OI43KH4O6zLzK+t30QxAv5zjk10RZ4+5SAdYndIlld9Y62opCfPDzRy3ubdve4ZEchpIKWTQvIxq3T5ogOhGaWBVYnkMtM2GVqvWV//46gET5SH/MdcwhACUcZ9kCpMnWH9CyyUwYvTT3UlNyV+DlS27LMPvaw7tx7qa+GfNCoCBd8S4esZpQYK/WReiS8=|pc7qpD42wxyXemdNPuwxbh8iIaryrBPu8f/DGwYdHTw=".parse().unwrap(),
        ).unwrap();

        let k = enc.get_key(&None).unwrap();

        let send_key = "2.+1KUfOX8A83Xkwk1bumo/w==|Nczvv+DTkeP466cP/wMDnGK6W9zEIg5iHLhcuQG6s+M=|SZGsfuIAIaGZ7/kzygaVUau3LeOvJUlolENBOU+LX7g="
            .parse()
            .unwrap();

        // Get the send key
        let send_key = Send::get_key(&send_key, k).unwrap();
        let send_key_b64 = send_key.to_base64();
        assert_eq!(send_key_b64, "IR9ImHGm6rRuIjiN7csj94bcZR5WYTJj5GtNfx33zm6tJCHUl+QZlpNPba8g2yn70KnOHsAODLcR0um6E3MAlg==");
    }

    fn build_encryption_settings() -> EncryptionSettings {
        EncryptionSettings::new(
            &UserLoginMethod::Username {
                client_id: "test".into(),
                email: "test@bitwarden.com".into(),
                kdf: Kdf::PBKDF2 {
                    iterations: 600_000.try_into().unwrap(),
                },
            },
            "asdfasdfasdf",
            "2.Q/2PhzcC7GdeiMHhWguYAQ==|GpqzVdr0go0ug5cZh1n+uixeBC3oC90CIe0hd/HWA/pTRDZ8ane4fmsEIcuc8eMKUt55Y2q/fbNzsYu41YTZzzsJUSeqVjT8/iTQtgnNdpo=|dwI+uyvZ1h/iZ03VQ+/wrGEFYVewBUUl/syYgjsNMbE=".parse().unwrap(),
            "2.yN7l00BOlUE0Sb0M//Q53w==|EwKG/BduQRQ33Izqc/ogoBROIoI5dmgrxSo82sgzgAMIBt3A2FZ9vPRMY+GWT85JiqytDitGR3TqwnFUBhKUpRRAq4x7rA6A1arHrFp5Tp1p21O3SfjtvB3quiOKbqWk6ZaU1Np9HwqwAecddFcB0YyBEiRX3VwF2pgpAdiPbSMuvo2qIgyob0CUoC/h4Bz1be7Qa7B0Xw9/fMKkB1LpOm925lzqosyMQM62YpMGkjMsbZz0uPopu32fxzDWSPr+kekNNyLt9InGhTpxLmq1go/pXR2uw5dfpXc5yuta7DB0EGBwnQ8Vl5HPdDooqOTD9I1jE0mRyuBpWTTI3FRnu3JUh3rIyGBJhUmHqGZvw2CKdqHCIrQeQkkEYqOeJRJVdBjhv5KGJifqT3BFRwX/YFJIChAQpebNQKXe/0kPivWokHWwXlDB7S7mBZzhaAPidZvnuIhalE2qmTypDwHy22FyqV58T8MGGMchcASDi/QXI6kcdpJzPXSeU9o+NC68QDlOIrMVxKFeE7w7PvVmAaxEo0YwmuAzzKy9QpdlK0aab/xEi8V4iXj4hGepqAvHkXIQd+r3FNeiLfllkb61p6WTjr5urcmDQMR94/wYoilpG5OlybHdbhsYHvIzYoLrC7fzl630gcO6t4nM24vdB6Ymg9BVpEgKRAxSbE62Tqacxqnz9AcmgItb48NiR/He3n3ydGjPYuKk/ihZMgEwAEZvSlNxYONSbYrIGDtOY+8Nbt6KiH3l06wjZW8tcmFeVlWv+tWotnTY9IqlAfvNVTjtsobqtQnvsiDjdEVtNy/s2ci5TH+NdZluca2OVEr91Wayxh70kpM6ib4UGbfdmGgCo74gtKvKSJU0rTHakQ5L9JlaSDD5FamBRyI0qfL43Ad9qOUZ8DaffDCyuaVyuqk7cz9HwmEmvWU3VQ+5t06n/5kRDXttcw8w+3qClEEdGo1KeENcnXCB32dQe3tDTFpuAIMLqwXs6FhpawfZ5kPYvLPczGWaqftIs/RXJ/EltGc0ugw2dmTLpoQhCqrcKEBDoYVk0LDZKsnzitOGdi9mOWse7Se8798ib1UsHFUjGzISEt6upestxOeupSTOh0v4+AjXbDzRUyogHww3V+Bqg71bkcMxtB+WM+pn1XNbVTyl9NR040nhP7KEf6e9ruXAtmrBC2ah5cFEpLIot77VFZ9ilLuitSz+7T8n1yAh1IEG6xxXxninAZIzi2qGbH69O5RSpOJuJTv17zTLJQIIc781JwQ2TTwTGnx5wZLbffhCasowJKd2EVcyMJyhz6ru0PvXWJ4hUdkARJs3Xu8dus9a86N8Xk6aAPzBDqzYb1vyFIfBxP0oO8xFHgd30Cgmz8UrSE3qeWRrF8ftrI6xQnFjHBGWD/JWSvd6YMcQED0aVuQkuNW9ST/DzQThPzRfPUoiL10yAmV7Ytu4fR3x2sF0Yfi87YhHFuCMpV/DsqxmUizyiJuD938eRcH8hzR/VO53Qo3UIsqOLcyXtTv6THjSlTopQ+JOLOnHm1w8dzYbLN44OG44rRsbihMUQp+wUZ6bsI8rrOnm9WErzkbQFbrfAINdoCiNa6cimYIjvvnMTaFWNymqY1vZxGztQiMiHiHYwTfwHTXrb9j0uPM=|09J28iXv9oWzYtzK2LBT6Yht4IT4MijEkk0fwFdrVQ4=".parse().unwrap(),
        ).unwrap()
    }

    #[test]
    pub fn test_decrypt() {
        let enc = build_encryption_settings();
        let key = enc.get_key(&None).unwrap();

        let send = Send {
            id: "3d80dd72-2d14-4f26-812c-b0f0018aa144".parse().ok(),
            access_id: Some("ct2APRQtJk-BLLDwAYqhRA".to_owned()),
            r#type: SendType::Text,
            name: "2.STIyTrfDZN/JXNDN9zNEMw==|NDLum8BHZpPNYhJo9ggSkg==|UCsCLlBO3QzdPwvMAWs2VVwuE6xwOx/vxOooPObqnEw=".parse()
                .unwrap(),
            notes: None,
            file: None,
            text: Some(SendText {
                text: "2.2VPyLzk1tMLug0X3x7RkaQ==|mrMt9vbZsCJhJIj4eebKyg==|aZ7JeyndytEMR1+uEBupEvaZuUE69D/ejhfdJL8oKq0=".parse().ok(),
                hidden: false,
            }),
            key: "2.KLv/j0V4Ebs0dwyPdtt4vw==|jcrFuNYN1Qb3onBlwvtxUV/KpdnR1LPRL4EsCoXNAt4=|gHSywGy4Rj/RsCIZFwze4s2AACYKBtqDXTrQXjkgtIE=".parse().unwrap(),
            max_access_count: None,
            access_count: 0,
            password: None,
            disabled: false,
            revision_date: "2024-01-07T23:56:48.207363Z".parse().unwrap(),
            expiration_date: None,
            deletion_date: "2024-01-14T23:56:48Z".parse().unwrap(),
            hide_email: false,
        };

        let view: SendView = send.decrypt_with_key(key).unwrap();

        let expected = SendView {
            id: "3d80dd72-2d14-4f26-812c-b0f0018aa144".parse().ok(),
            access_id: Some("ct2APRQtJk-BLLDwAYqhRA".to_owned()),
            name: "Test".to_string(),
            notes: None,
            key: Some("Pgui0FK85cNhBGWHAlBHBw".to_owned()),
            new_password: None,
            has_password: false,
            r#type: SendType::Text,
            file: None,
            text: Some(SendTextView {
                text: Some("This is a test".to_owned()),
                hidden: false,
            }),
            max_access_count: None,
            access_count: 0,
            disabled: false,
            hide_email: false,
            revision_date: "2024-01-07T23:56:48.207363Z".parse().unwrap(),
            deletion_date: "2024-01-14T23:56:48Z".parse().unwrap(),
            expiration_date: None,
        };

        assert_eq!(view, expected);
    }

    #[test]
    pub fn test_encrypt() {
        let enc = build_encryption_settings();
        let key = enc.get_key(&None).unwrap();

        let view = SendView {
            id: "3d80dd72-2d14-4f26-812c-b0f0018aa144".parse().ok(),
            access_id: Some("ct2APRQtJk-BLLDwAYqhRA".to_owned()),
            name: "Test".to_string(),
            notes: None,
            key: Some("Pgui0FK85cNhBGWHAlBHBw".to_owned()),
            new_password: None,
            has_password: false,
            r#type: SendType::Text,
            file: None,
            text: Some(SendTextView {
                text: Some("This is a test".to_owned()),
                hidden: false,
            }),
            max_access_count: None,
            access_count: 0,
            disabled: false,
            hide_email: false,
            revision_date: "2024-01-07T23:56:48.207363Z".parse().unwrap(),
            deletion_date: "2024-01-14T23:56:48Z".parse().unwrap(),
            expiration_date: None,
        };

        // Re-encrypt and decrypt again to ensure encrypt works
        let v: SendView = view
            .clone()
            .encrypt_with_key(key)
            .unwrap()
            .decrypt_with_key(key)
            .unwrap();
        assert_eq!(v, view);
    }

    #[test]
    pub fn test_create() {
        let enc = build_encryption_settings();
        let key = enc.get_key(&None).unwrap();

        let view = SendView {
            id: None,
            access_id: Some("ct2APRQtJk-BLLDwAYqhRA".to_owned()),
            name: "Test".to_string(),
            notes: None,
            key: None,
            new_password: None,
            has_password: false,
            r#type: SendType::Text,
            file: None,
            text: Some(SendTextView {
                text: Some("This is a test".to_owned()),
                hidden: false,
            }),
            max_access_count: None,
            access_count: 0,
            disabled: false,
            hide_email: false,
            revision_date: "2024-01-07T23:56:48.207363Z".parse().unwrap(),
            deletion_date: "2024-01-14T23:56:48Z".parse().unwrap(),
            expiration_date: None,
        };

        // Re-encrypt and decrypt again to ensure encrypt works
        let v: SendView = view
            .clone()
            .encrypt_with_key(key)
            .unwrap()
            .decrypt_with_key(key)
            .unwrap();

        // Ignore key when comparing
        let t = SendView { key: None, ..v };
        assert_eq!(t, view);
    }

    #[test]
    pub fn test_create_password() {
        let enc = build_encryption_settings();
        let key = enc.get_key(&None).unwrap();

        let view = SendView {
            id: None,
            access_id: Some("ct2APRQtJk-BLLDwAYqhRA".to_owned()),
            name: "Test".to_owned(),
            notes: None,
            key: Some("Pgui0FK85cNhBGWHAlBHBw".to_owned()),
            new_password: Some("abc123".to_owned()),
            has_password: false,
            r#type: SendType::Text,
            file: None,
            text: Some(SendTextView {
                text: Some("This is a test".to_owned()),
                hidden: false,
            }),
            max_access_count: None,
            access_count: 0,
            disabled: false,
            hide_email: false,
            revision_date: "2024-01-07T23:56:48.207363Z".parse().unwrap(),
            deletion_date: "2024-01-14T23:56:48Z".parse().unwrap(),
            expiration_date: None,
        };

        let send: Send = view.encrypt_with_key(key).unwrap();

        assert_eq!(
            send.password,
            Some("vTIDfdj3FTDbejmMf+mJWpYdMXsxfeSd1Sma3sjCtiQ=".to_owned())
        );

        let v: SendView = send.decrypt_with_key(key).unwrap();
        assert_eq!(v.new_password, None);
        assert!(v.has_password);
    }
}
