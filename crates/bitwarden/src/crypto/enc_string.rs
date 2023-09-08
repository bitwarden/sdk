use std::{fmt::Display, marker::PhantomData, str::FromStr};

use base64::Engine;
use serde::{de::Visitor, Deserialize};
use uuid::Uuid;

use crate::{
    client::encryption_settings::EncryptionSettings,
    crypto::{decrypt_aes256_hmac, Decryptable, Encryptable, SymmetricCryptoKey},
    error::{CryptoError, EncStringParseError, Error, Result},
    util::BASE64_ENGINE,
};

pub trait KeyType: Clone + Copy {
    const IS_SYMMETRIC: bool;
}

#[derive(Clone, Copy)]
pub struct SymmetricKey;
#[derive(Clone, Copy)]
pub struct AsymmetricKey;

impl KeyType for SymmetricKey {
    const IS_SYMMETRIC: bool = true;
}
impl KeyType for AsymmetricKey {
    const IS_SYMMETRIC: bool = false;
}

#[derive(Clone)]
pub struct BaseEncString<K: KeyType>(pub(crate) EncStringVariants, PhantomData<K>);

pub type EncString = BaseEncString<SymmetricKey>;
pub type AsymmetricEncString = BaseEncString<AsymmetricKey>;

// We manually implement these to make sure we don't print any sensitive data
impl std::fmt::Debug for EncString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EncString").finish()
    }
}

impl std::fmt::Debug for AsymmetricEncString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsymmetricEncString").finish()
    }
}

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub enum EncStringVariants {
    /// 0
    AesCbc256_B64 { iv: [u8; 16], data: Vec<u8> },
    /// 1
    AesCbc128_HmacSha256_B64 {
        iv: [u8; 16],
        mac: [u8; 32],
        data: Vec<u8>,
    },
    /// 2
    AesCbc256_HmacSha256_B64 {
        iv: [u8; 16],
        mac: [u8; 32],
        data: Vec<u8>,
    },
    /// 3
    Rsa2048_OaepSha256_B64 { data: Vec<u8> },
    /// 4
    Rsa2048_OaepSha1_B64 { data: Vec<u8> },
    /// 5
    #[deprecated]
    Rsa2048_OaepSha256_HmacSha256_B64 { data: Vec<u8> },
    /// 6
    #[deprecated]
    Rsa2048_OaepSha1_HmacSha256_B64 { data: Vec<u8> },
}

impl<K: KeyType> From<EncStringVariants> for BaseEncString<K> {
    fn from(value: EncStringVariants) -> Self {
        Self(value, PhantomData)
    }
}

impl<K: KeyType> FromStr for BaseEncString<K> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (enc_type, parts): (&str, Vec<_>) = {
            let header_parts: Vec<_> = s.split('.').collect();

            if header_parts.len() == 2 {
                (header_parts[0], header_parts[1].split('|').collect())
            } else {
                // Support legacy format with no header
                let parts: Vec<_> = s.split('|').collect();
                if parts.len() == 3 {
                    ("1", parts) // AesCbc128_HmacSha256_B64
                } else {
                    ("0", parts) // AesCbc256_B64
                }
            }
        };

        fn from_b64_vec(s: &str) -> Result<Vec<u8>> {
            Ok(BASE64_ENGINE
                .decode(s)
                .map_err(EncStringParseError::InvalidBase64)?)
        }

        fn from_b64<const N: usize>(s: &str) -> Result<[u8; N]> {
            Ok(from_b64_vec(s)?.try_into().map_err(invalid_len_error(N))?)
        }

        match (K::IS_SYMMETRIC, enc_type, parts.len()) {
            (true, "0", 2) => {
                let iv = from_b64(parts[0])?;
                let data = from_b64_vec(parts[1])?;

                Ok(EncStringVariants::AesCbc256_B64 { iv, data })
            }
            (true, "1" | "2", 3) => {
                let iv = from_b64(parts[0])?;
                let data = from_b64_vec(parts[1])?;
                let mac = from_b64(parts[2])?;

                if enc_type == "1" {
                    Ok(EncStringVariants::AesCbc128_HmacSha256_B64 { iv, mac, data })
                } else {
                    Ok(EncStringVariants::AesCbc256_HmacSha256_B64 { iv, mac, data })
                }
            }
            (false, "3", 1) => {
                let data = from_b64_vec(parts[0])?;
                Ok(EncStringVariants::Rsa2048_OaepSha256_B64 { data })
            }
            (false, "4", 1) => {
                let data = from_b64_vec(parts[0])?;
                Ok(EncStringVariants::Rsa2048_OaepSha1_B64 { data })
            }
            #[allow(deprecated)]
            (false, "5", 1) => {
                let data = from_b64_vec(parts[0])?;
                Ok(EncStringVariants::Rsa2048_OaepSha256_HmacSha256_B64 { data })
            }
            #[allow(deprecated)]
            (false, "6", 1) => {
                let data = from_b64_vec(parts[0])?;
                Ok(EncStringVariants::Rsa2048_OaepSha1_HmacSha256_B64 { data })
            }

            (_, enc_type, parts) => Err(EncStringParseError::InvalidType {
                enc_type: enc_type.to_string(),
                parts,
            }
            .into()),
        }
        .map(Into::into)
    }
}

impl<K: KeyType> BaseEncString<K> {
    #[cfg(feature = "mobile")]
    pub(crate) fn from_buffer(buf: &[u8]) -> Result<Self> {
        if buf.is_empty() {
            return Err(EncStringParseError::NoType.into());
        }
        let enc_type = buf[0];

        fn check_length(buf: &[u8], expected: usize) -> Result<()> {
            if buf.len() < expected {
                return Err(EncStringParseError::InvalidLength {
                    expected,
                    got: buf.len(),
                }
                .into());
            }
            Ok(())
        }

        match (K::IS_SYMMETRIC, enc_type) {
            (true, 0) => {
                check_length(buf, 18)?;
                let iv = buf[1..17].try_into().unwrap();
                let data = buf[17..].to_vec();

                Ok(EncStringVariants::AesCbc256_B64 { iv, data })
            }
            (true, 1 | 2) => {
                check_length(buf, 50)?;
                let iv = buf[1..17].try_into().unwrap();
                let mac = buf[17..49].try_into().unwrap();
                let data = buf[49..].to_vec();

                if enc_type == 1 {
                    Ok(EncStringVariants::AesCbc128_HmacSha256_B64 { iv, mac, data })
                } else {
                    Ok(EncStringVariants::AesCbc256_HmacSha256_B64 { iv, mac, data })
                }
            }
            (false, 3) => {
                check_length(buf, 2)?;
                let data = buf[1..].to_vec();
                Ok(EncStringVariants::Rsa2048_OaepSha256_B64 { data })
            }
            (false, 4) => {
                check_length(buf, 2)?;
                let data = buf[1..].to_vec();
                Ok(EncStringVariants::Rsa2048_OaepSha1_B64 { data })
            }
            #[allow(deprecated)]
            (false, 5) => {
                check_length(buf, 2)?;
                let data = buf[1..].to_vec();
                Ok(EncStringVariants::Rsa2048_OaepSha256_HmacSha256_B64 { data })
            }
            #[allow(deprecated)]
            (false, 6) => {
                check_length(buf, 2)?;
                let data = buf[1..].to_vec();
                Ok(EncStringVariants::Rsa2048_OaepSha1_HmacSha256_B64 { data })
            }
            (_, _) => Err(EncStringParseError::InvalidType {
                enc_type: enc_type.to_string(),
                parts: 1,
            }
            .into()),
        }
        .map(Into::into)
    }

    #[cfg(feature = "mobile")]
    pub(crate) fn to_buffer(&self) -> Result<Vec<u8>> {
        let mut buf;

        match &self.0 {
            EncStringVariants::AesCbc256_B64 { iv, data } => {
                buf = Vec::with_capacity(1 + 16 + data.len());
                buf.push(self.enc_type());
                buf.extend_from_slice(iv);
                buf.extend_from_slice(data);
            }
            EncStringVariants::AesCbc128_HmacSha256_B64 { iv, mac, data }
            | EncStringVariants::AesCbc256_HmacSha256_B64 { iv, mac, data } => {
                buf = Vec::with_capacity(1 + 16 + 32 + data.len());
                buf.push(self.enc_type());
                buf.extend_from_slice(iv);
                buf.extend_from_slice(mac);
                buf.extend_from_slice(data);
            }

            EncStringVariants::Rsa2048_OaepSha256_B64 { data }
            | EncStringVariants::Rsa2048_OaepSha1_B64 { data } => {
                buf = Vec::with_capacity(1 + data.len());
                buf.push(self.enc_type());
                buf.extend_from_slice(data);
            }
            #[allow(deprecated)]
            EncStringVariants::Rsa2048_OaepSha256_HmacSha256_B64 { data }
            | EncStringVariants::Rsa2048_OaepSha1_HmacSha256_B64 { data } => {
                buf = Vec::with_capacity(1 + data.len());
                buf.push(self.enc_type());
                buf.extend_from_slice(data);
            }
        }

        Ok(buf)
    }
}

impl<K: KeyType> Display for BaseEncString<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parts: Vec<&[u8]> = match &self.0 {
            EncStringVariants::AesCbc256_B64 { iv, data } => vec![iv, data],
            EncStringVariants::AesCbc128_HmacSha256_B64 { iv, mac, data } => vec![iv, data, mac],
            EncStringVariants::AesCbc256_HmacSha256_B64 { iv, mac, data } => vec![iv, data, mac],
            EncStringVariants::Rsa2048_OaepSha256_B64 { data } => vec![data],
            EncStringVariants::Rsa2048_OaepSha1_B64 { data } => vec![data],
            #[allow(deprecated)]
            EncStringVariants::Rsa2048_OaepSha256_HmacSha256_B64 { data } => vec![data],
            #[allow(deprecated)]
            EncStringVariants::Rsa2048_OaepSha1_HmacSha256_B64 { data } => vec![data],
        };

        let encoded_parts: Vec<String> = parts
            .iter()
            .map(|part| BASE64_ENGINE.encode(part))
            .collect();

        write!(f, "{}.{}", self.enc_type(), encoded_parts.join("|"))?;

        Ok(())
    }
}

impl<'de, K: KeyType> Deserialize<'de> for BaseEncString<K> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct EncStringVisitor<K: KeyType>(PhantomData<K>);
        impl<K: KeyType> Visitor<'_> for EncStringVisitor<K> {
            type Value = BaseEncString<K>;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "a valid string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                BaseEncString::from_str(v).map_err(|e| E::custom(format!("{:?}", e)))
            }
        }

        deserializer.deserialize_str(EncStringVisitor::<K>(PhantomData))
    }
}

impl<K: KeyType> serde::Serialize for BaseEncString<K> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<K: KeyType> BaseEncString<K> {
    const fn enc_type(&self) -> u8 {
        match &self.0 {
            EncStringVariants::AesCbc256_B64 { .. } => 0,
            EncStringVariants::AesCbc128_HmacSha256_B64 { .. } => 1,
            EncStringVariants::AesCbc256_HmacSha256_B64 { .. } => 2,
            EncStringVariants::Rsa2048_OaepSha256_B64 { .. } => 3,
            EncStringVariants::Rsa2048_OaepSha1_B64 { .. } => 4,
            #[allow(deprecated)]
            EncStringVariants::Rsa2048_OaepSha256_HmacSha256_B64 { .. } => 5,
            #[allow(deprecated)]
            EncStringVariants::Rsa2048_OaepSha1_HmacSha256_B64 { .. } => 6,
        }
    }
}

impl BaseEncString<SymmetricKey> {
    pub fn decrypt_with_key(&self, key: &SymmetricCryptoKey) -> Result<Vec<u8>> {
        match &self.0 {
            EncStringVariants::AesCbc256_HmacSha256_B64 { iv, mac, data } => {
                let mac_key = key.mac_key.ok_or(CryptoError::InvalidMac)?;
                let dec = decrypt_aes256_hmac(iv, mac, data.clone(), mac_key, key.key)?;
                Ok(dec)
            }
            _ => Err(CryptoError::InvalidKey.into()),
        }
    }
}

impl BaseEncString<AsymmetricKey> {
    pub fn decrypt_with_key(&self, key: &rsa::RsaPrivateKey) -> Result<Vec<u8>> {
        match &self.0 {
            EncStringVariants::Rsa2048_OaepSha1_B64 { data } => {
                let dec = key
                    .decrypt(rsa::Oaep::new::<sha1::Sha1>(), data)
                    .map_err(|_| CryptoError::KeyDecrypt)?;
                Ok(dec)
            }
            _ => Err(CryptoError::InvalidKey.into()),
        }
    }
}

fn invalid_len_error(expected: usize) -> impl Fn(Vec<u8>) -> EncStringParseError {
    move |e: Vec<_>| EncStringParseError::InvalidLength {
        expected,
        got: e.len(),
    }
}

impl Encryptable<BaseEncString<SymmetricKey>> for String {
    fn encrypt(
        self,
        enc: &EncryptionSettings,
        org_id: &Option<Uuid>,
    ) -> Result<BaseEncString<SymmetricKey>> {
        enc.encrypt(self.as_bytes(), org_id)
    }
}

impl Decryptable<String> for BaseEncString<SymmetricKey> {
    fn decrypt(&self, enc: &EncryptionSettings, org_id: &Option<Uuid>) -> Result<String> {
        enc.decrypt(self, org_id)
    }
}

impl Encryptable<BaseEncString<AsymmetricKey>> for String {
    fn encrypt(
        self,
        enc: &EncryptionSettings,
        org_id: &Option<Uuid>,
    ) -> Result<BaseEncString<AsymmetricKey>> {
        enc.encrypt_asymmetric(self.as_bytes(), org_id)
    }
}

impl Decryptable<String> for BaseEncString<AsymmetricKey> {
    fn decrypt(&self, enc: &EncryptionSettings, org_id: &Option<Uuid>) -> Result<String> {
        enc.decrypt_asymmetric(self, org_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enc_string_serialization() {
        #[derive(serde::Serialize, serde::Deserialize)]
        struct Test {
            key: EncString,
        }

        let cipher = "2.pMS6/icTQABtulw52pq2lg==|XXbxKxDTh+mWiN1HjH2N1w==|Q6PkuT+KX/axrgN9ubD5Ajk2YNwxQkgs3WJM0S0wtG8=";
        let serialized = format!("{{\"key\":\"{cipher}\"}}");

        let t = serde_json::from_str::<Test>(&serialized).unwrap();
        assert_eq!(t.key.enc_type(), 2);
        assert_eq!(t.key.to_string(), cipher);
        assert_eq!(serde_json::to_string(&t).unwrap(), serialized);
    }

    #[cfg(feature = "mobile")]
    #[test]
    fn test_enc_from_to_buffer() {
        let enc_str: &str = "2.pMS6/icTQABtulw52pq2lg==|XXbxKxDTh+mWiN1HjH2N1w==|Q6PkuT+KX/axrgN9ubD5Ajk2YNwxQkgs3WJM0S0wtG8=";
        let enc_string: EncString = enc_str.parse().unwrap();

        let enc_buf = enc_string.to_buffer().unwrap();

        assert_eq!(
            enc_buf,
            vec![
                2, 164, 196, 186, 254, 39, 19, 64, 0, 109, 186, 92, 57, 218, 154, 182, 150, 67,
                163, 228, 185, 63, 138, 95, 246, 177, 174, 3, 125, 185, 176, 249, 2, 57, 54, 96,
                220, 49, 66, 72, 44, 221, 98, 76, 209, 45, 48, 180, 111, 93, 118, 241, 43, 16, 211,
                135, 233, 150, 136, 221, 71, 140, 125, 141, 215
            ]
        );

        let enc_string_new = EncString::from_buffer(&enc_buf).unwrap();

        assert_eq!(enc_string_new.to_string(), enc_str)
    }
}
