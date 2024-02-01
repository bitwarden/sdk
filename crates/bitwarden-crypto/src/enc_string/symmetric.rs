use std::{fmt::Display, str::FromStr};

use aes::cipher::typenum::U32;
use base64::{engine::general_purpose::STANDARD, Engine};
use generic_array::GenericArray;
use serde::Deserialize;

use super::{check_length, from_b64, from_b64_vec, split_enc_string};
use crate::{
    error::{CryptoError, EncStringParseError, Result},
    KeyDecryptable, KeyEncryptable, LocateKey, SymmetricCryptoKey,
};

/// # Encrypted string primitive
///
/// [EncString] is a Bitwarden specific primitive that represents a symmetrically encrypted string.
/// They are are used together with the [KeyDecryptable] and [KeyEncryptable] traits to encrypt and
/// decrypt data using [SymmetricCryptoKey]s.
///
/// The flexibility of the [EncString] type allows for different encryption algorithms to be used
/// which is represented by the different variants of the enum.
///
/// ## Note
///
/// For backwards compatibility we will rarely if ever be able to remove support for decrypting old
/// variants, but we should be opinionated in which variants are used for encrypting.
///
/// ## Variants
/// - [AesCbc256_B64](EncString::AesCbc256_B64)
/// - [AesCbc128_HmacSha256_B64](EncString::AesCbc128_HmacSha256_B64)
/// - [AesCbc256_HmacSha256_B64](EncString::AesCbc256_HmacSha256_B64)
///
/// ## Serialization
///
/// [EncString] implements [Display] and [FromStr] to allow for easy serialization and uses a
/// custom scheme to represent the different variants.
///
/// The scheme is one of the following schemes:
/// - `[type].[iv]|[data]`
/// - `[type].[iv]|[data]|[mac]`
///
/// Where:
/// - `[type]`: is a digit number representing the variant.
/// - `[iv]`: (optional) is the initialization vector used for encryption.
/// - `[data]`: is the encrypted data.
/// - `[mac]`: (optional) is the MAC used to validate the integrity of the data.
#[derive(Clone, zeroize::ZeroizeOnDrop)]
#[allow(unused, non_camel_case_types)]
pub enum EncString {
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
}

/// To avoid printing sensitive information, [EncString] debug prints to `EncString`.
impl std::fmt::Debug for EncString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EncString").finish()
    }
}

/// Deserializes an [EncString] from a string.
impl FromStr for EncString {
    type Err = CryptoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (enc_type, parts) = split_enc_string(s);
        match (enc_type, parts.len()) {
            ("0", 2) => {
                let iv = from_b64(parts[0])?;
                let data = from_b64_vec(parts[1])?;

                Ok(EncString::AesCbc256_B64 { iv, data })
            }
            ("1" | "2", 3) => {
                let iv = from_b64(parts[0])?;
                let data = from_b64_vec(parts[1])?;
                let mac = from_b64(parts[2])?;

                if enc_type == "1" {
                    Ok(EncString::AesCbc128_HmacSha256_B64 { iv, mac, data })
                } else {
                    Ok(EncString::AesCbc256_HmacSha256_B64 { iv, mac, data })
                }
            }

            (enc_type, parts) => Err(EncStringParseError::InvalidTypeSymm {
                enc_type: enc_type.to_string(),
                parts,
            }
            .into()),
        }
    }
}

impl EncString {
    /// Synthetic sugar for mapping `Option<String>` to `Result<Option<EncString>>`
    pub fn try_from_optional(s: Option<String>) -> Result<Option<EncString>, CryptoError> {
        s.map(|s| s.parse()).transpose()
    }

    pub fn from_buffer(buf: &[u8]) -> Result<Self> {
        if buf.is_empty() {
            return Err(EncStringParseError::NoType.into());
        }
        let enc_type = buf[0];

        match enc_type {
            0 => {
                check_length(buf, 18)?;
                let iv = buf[1..17].try_into().unwrap();
                let data = buf[17..].to_vec();

                Ok(EncString::AesCbc256_B64 { iv, data })
            }
            1 | 2 => {
                check_length(buf, 50)?;
                let iv = buf[1..17].try_into().unwrap();
                let mac = buf[17..49].try_into().unwrap();
                let data = buf[49..].to_vec();

                if enc_type == 1 {
                    Ok(EncString::AesCbc128_HmacSha256_B64 { iv, mac, data })
                } else {
                    Ok(EncString::AesCbc256_HmacSha256_B64 { iv, mac, data })
                }
            }
            _ => Err(EncStringParseError::InvalidTypeSymm {
                enc_type: enc_type.to_string(),
                parts: 1,
            }
            .into()),
        }
    }

    pub fn to_buffer(&self) -> Result<Vec<u8>> {
        let mut buf;

        match self {
            EncString::AesCbc256_B64 { iv, data } => {
                buf = Vec::with_capacity(1 + 16 + data.len());
                buf.push(self.enc_type());
                buf.extend_from_slice(iv);
                buf.extend_from_slice(data);
            }
            EncString::AesCbc128_HmacSha256_B64 { iv, mac, data }
            | EncString::AesCbc256_HmacSha256_B64 { iv, mac, data } => {
                buf = Vec::with_capacity(1 + 16 + 32 + data.len());
                buf.push(self.enc_type());
                buf.extend_from_slice(iv);
                buf.extend_from_slice(mac);
                buf.extend_from_slice(data);
            }
        }

        Ok(buf)
    }
}

impl Display for EncString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parts: Vec<&[u8]> = match self {
            EncString::AesCbc256_B64 { iv, data } => vec![iv, data],
            EncString::AesCbc128_HmacSha256_B64 { iv, mac, data } => vec![iv, data, mac],
            EncString::AesCbc256_HmacSha256_B64 { iv, mac, data } => vec![iv, data, mac],
        };

        let encoded_parts: Vec<String> = parts.iter().map(|part| STANDARD.encode(part)).collect();

        write!(f, "{}.{}", self.enc_type(), encoded_parts.join("|"))?;

        Ok(())
    }
}

impl<'de> Deserialize<'de> for EncString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(super::FromStrVisitor::new())
    }
}

impl serde::Serialize for EncString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl EncString {
    pub fn encrypt_aes256_hmac(
        data_dec: &[u8],
        mac_key: &GenericArray<u8, U32>,
        key: &GenericArray<u8, U32>,
    ) -> Result<EncString> {
        let (iv, mac, data) = crate::aes::encrypt_aes256_hmac(data_dec, mac_key, key)?;
        Ok(EncString::AesCbc256_HmacSha256_B64 { iv, mac, data })
    }

    /// The numerical representation of the encryption type of the [EncString].
    const fn enc_type(&self) -> u8 {
        match self {
            EncString::AesCbc256_B64 { .. } => 0,
            EncString::AesCbc128_HmacSha256_B64 { .. } => 1,
            EncString::AesCbc256_HmacSha256_B64 { .. } => 2,
        }
    }
}

impl LocateKey for EncString {}
impl KeyEncryptable<SymmetricCryptoKey, EncString> for &[u8] {
    fn encrypt_with_key(self, key: &SymmetricCryptoKey) -> Result<EncString> {
        EncString::encrypt_aes256_hmac(
            self,
            key.mac_key.as_ref().ok_or(CryptoError::InvalidMac)?,
            &key.key,
        )
    }
}

impl KeyDecryptable<SymmetricCryptoKey, Vec<u8>> for EncString {
    fn decrypt_with_key(&self, key: &SymmetricCryptoKey) -> Result<Vec<u8>> {
        match self {
            EncString::AesCbc256_HmacSha256_B64 { iv, mac, data } => {
                let mac_key = key.mac_key.as_ref().ok_or(CryptoError::InvalidMac)?;
                let dec =
                    crate::aes::decrypt_aes256_hmac(iv, mac, data.clone(), mac_key, &key.key)?;
                Ok(dec)
            }
            _ => Err(CryptoError::InvalidKey),
        }
    }
}

impl KeyEncryptable<SymmetricCryptoKey, EncString> for String {
    fn encrypt_with_key(self, key: &SymmetricCryptoKey) -> Result<EncString> {
        self.as_bytes().encrypt_with_key(key)
    }
}

impl KeyDecryptable<SymmetricCryptoKey, String> for EncString {
    fn decrypt_with_key(&self, key: &SymmetricCryptoKey) -> Result<String> {
        let dec: Vec<u8> = self.decrypt_with_key(key)?;
        String::from_utf8(dec).map_err(|_| CryptoError::InvalidUtf8String)
    }
}

/// Usually we wouldn't want to expose EncStrings in the API or the schemas.
/// But during the transition phase we will expose endpoints using the EncString type.
impl schemars::JsonSchema for EncString {
    fn schema_name() -> String {
        "EncString".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        gen.subschema_for::<String>()
    }
}

#[cfg(test)]
mod tests {
    use schemars::schema_for;

    use super::EncString;
    use crate::{derive_symmetric_key, KeyDecryptable, KeyEncryptable};

    #[test]
    fn test_enc_string_roundtrip() {
        let key = derive_symmetric_key("test");

        let test_string = "encrypted_test_string".to_string();
        let cipher = test_string.clone().encrypt_with_key(&key).unwrap();

        let decrypted_str: String = cipher.decrypt_with_key(&key).unwrap();
        assert_eq!(decrypted_str, test_string);
    }

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

    #[test]
    fn test_from_str_cbc256() {
        let enc_str = "0.pMS6/icTQABtulw52pq2lg==|XXbxKxDTh+mWiN1HjH2N1w==";
        let enc_string: EncString = enc_str.parse().unwrap();

        assert_eq!(enc_string.enc_type(), 0);
        if let EncString::AesCbc256_B64 { iv, data } = &enc_string {
            assert_eq!(
                iv,
                &[164, 196, 186, 254, 39, 19, 64, 0, 109, 186, 92, 57, 218, 154, 182, 150]
            );
            assert_eq!(
                data,
                &[93, 118, 241, 43, 16, 211, 135, 233, 150, 136, 221, 71, 140, 125, 141, 215]
            );
        }
    }

    #[test]
    fn test_from_str_cbc128_hmac() {
        let enc_str = "1.Hh8gISIjJCUmJygpKissLQ==|MjM0NTY3ODk6Ozw9Pj9AQUJDREU=|KCkqKywtLi8wMTIzNDU2Nzg5Ojs8PT4/QEFCQ0RFRkc=";
        let enc_string: EncString = enc_str.parse().unwrap();

        assert_eq!(enc_string.enc_type(), 1);
        if let EncString::AesCbc128_HmacSha256_B64 { iv, mac, data } = &enc_string {
            assert_eq!(
                iv,
                &[30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45]
            );
            assert_eq!(
                mac,
                &[
                    40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59,
                    60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71
                ]
            );
            assert_eq!(
                data,
                &[50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69]
            );
        }
    }

    #[test]
    fn test_from_str_invalid() {
        let enc_str = "7.ABC";
        let enc_string: Result<EncString, _> = enc_str.parse();

        let err = enc_string.unwrap_err();
        assert_eq!(
            err.to_string(),
            "EncString error, Invalid symmetric type, got type 7 with 1 parts"
        );
    }

    #[test]
    fn test_debug_format() {
        let enc_str  = "2.pMS6/icTQABtulw52pq2lg==|XXbxKxDTh+mWiN1HjH2N1w==|Q6PkuT+KX/axrgN9ubD5Ajk2YNwxQkgs3WJM0S0wtG8=";
        let enc_string: EncString = enc_str.parse().unwrap();

        let debug_string = format!("{:?}", enc_string);
        assert_eq!(debug_string, "EncString");
    }

    #[test]
    fn test_json_schema() {
        let schema = schema_for!(EncString);

        assert_eq!(
            serde_json::to_string(&schema).unwrap(),
            r#"{"$schema":"http://json-schema.org/draft-07/schema#","title":"EncString","type":"string"}"#
        );
    }
}
