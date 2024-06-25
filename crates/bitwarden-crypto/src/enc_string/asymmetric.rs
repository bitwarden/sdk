use std::{fmt::Display, str::FromStr};

use base64::{engine::general_purpose::STANDARD, Engine};
pub use internal::AsymmetricEncString;
use rsa::Oaep;
use serde::Deserialize;

use super::{from_b64_vec, split_enc_string};
use crate::{
    error::{CryptoError, EncStringParseError, Result},
    rsa::encrypt_rsa2048_oaep_sha1,
    AsymmetricCryptoKey, AsymmetricEncryptable, KeyDecryptable,
};

// This module is a workaround to avoid deprecated warnings that come from the ZeroizeOnDrop
// macro expansion
#[allow(deprecated)]
mod internal {
    /// # Encrypted string primitive
    ///
    /// [AsymmetricEncString] is a Bitwarden specific primitive that represents an asymmetrically
    /// encrypted string. They are used together with the KeyDecryptable and KeyEncryptable
    /// traits to encrypt and decrypt data using [crate::AsymmetricCryptoKey]s.
    ///
    /// The flexibility of the [AsymmetricEncString] type allows for different encryption algorithms
    /// to be used which is represented by the different variants of the enum.
    ///
    /// ## Note
    ///
    /// For backwards compatibility we will rarely if ever be able to remove support for decrypting
    /// old variants, but we should be opinionated in which variants are used for encrypting.
    ///
    /// ## Variants
    /// - [Rsa2048_OaepSha256_B64](AsymmetricEncString::Rsa2048_OaepSha256_B64)
    /// - [Rsa2048_OaepSha1_B64](AsymmetricEncString::Rsa2048_OaepSha1_B64)
    ///
    /// ## Serialization
    ///
    /// [AsymmetricEncString] implements [std::fmt::Display] and [std::str::FromStr] to allow for
    /// easy serialization and uses a custom scheme to represent the different variants.
    ///
    /// The scheme is one of the following schemes:
    /// - `[type].[data]`
    ///
    /// Where:
    /// - `[type]`: is a digit number representing the variant.
    /// - `[data]`: is the encrypted data.
    #[derive(Clone, zeroize::ZeroizeOnDrop)]
    #[allow(unused, non_camel_case_types)]
    pub enum AsymmetricEncString {
        /// 3
        Rsa2048_OaepSha256_B64 { data: Vec<u8> },
        /// 4
        Rsa2048_OaepSha1_B64 { data: Vec<u8> },
        /// 5
        #[deprecated]
        Rsa2048_OaepSha256_HmacSha256_B64 { data: Vec<u8>, mac: Vec<u8> },
        /// 6
        #[deprecated]
        Rsa2048_OaepSha1_HmacSha256_B64 { data: Vec<u8>, mac: Vec<u8> },
    }
}

/// To avoid printing sensitive information, [AsymmetricEncString] debug prints to
/// `AsymmetricEncString`.
impl std::fmt::Debug for AsymmetricEncString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsymmetricEncString").finish()
    }
}

/// Deserializes an [AsymmetricEncString] from a string.
impl FromStr for AsymmetricEncString {
    type Err = CryptoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (enc_type, parts) = split_enc_string(s);
        match (enc_type, parts.len()) {
            ("3", 1) => {
                let data = from_b64_vec(parts[0])?;
                Ok(AsymmetricEncString::Rsa2048_OaepSha256_B64 { data })
            }
            ("4", 1) => {
                let data = from_b64_vec(parts[0])?;
                Ok(AsymmetricEncString::Rsa2048_OaepSha1_B64 { data })
            }
            #[allow(deprecated)]
            ("5", 2) => {
                let data = from_b64_vec(parts[0])?;
                let mac: Vec<u8> = from_b64_vec(parts[1])?;
                Ok(AsymmetricEncString::Rsa2048_OaepSha256_HmacSha256_B64 { data, mac })
            }
            #[allow(deprecated)]
            ("6", 2) => {
                let data = from_b64_vec(parts[0])?;
                let mac: Vec<u8> = from_b64_vec(parts[1])?;
                Ok(AsymmetricEncString::Rsa2048_OaepSha1_HmacSha256_B64 { data, mac })
            }

            (enc_type, parts) => Err(EncStringParseError::InvalidTypeAsymm {
                enc_type: enc_type.to_string(),
                parts,
            }
            .into()),
        }
    }
}

impl Display for AsymmetricEncString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parts: Vec<&[u8]> = match self {
            AsymmetricEncString::Rsa2048_OaepSha256_B64 { data } => vec![data],
            AsymmetricEncString::Rsa2048_OaepSha1_B64 { data } => vec![data],
            #[allow(deprecated)]
            AsymmetricEncString::Rsa2048_OaepSha256_HmacSha256_B64 { data, mac } => vec![data, mac],
            #[allow(deprecated)]
            AsymmetricEncString::Rsa2048_OaepSha1_HmacSha256_B64 { data, mac } => vec![data, mac],
        };

        let encoded_parts: Vec<String> = parts.iter().map(|part| STANDARD.encode(part)).collect();

        write!(f, "{}.{}", self.enc_type(), encoded_parts.join("|"))?;

        Ok(())
    }
}

impl<'de> Deserialize<'de> for AsymmetricEncString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(super::FromStrVisitor::new())
    }
}

impl serde::Serialize for AsymmetricEncString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl AsymmetricEncString {
    /// Encrypt and produce a [AsymmetricEncString::Rsa2048_OaepSha1_B64] variant.
    pub fn encrypt_rsa2048_oaep_sha1(
        data_dec: &[u8],
        key: &dyn AsymmetricEncryptable,
    ) -> Result<AsymmetricEncString> {
        let enc = encrypt_rsa2048_oaep_sha1(key.to_public_key(), data_dec)?;
        Ok(AsymmetricEncString::Rsa2048_OaepSha1_B64 { data: enc })
    }

    /// The numerical representation of the encryption type of the [AsymmetricEncString].
    const fn enc_type(&self) -> u8 {
        match self {
            AsymmetricEncString::Rsa2048_OaepSha256_B64 { .. } => 3,
            AsymmetricEncString::Rsa2048_OaepSha1_B64 { .. } => 4,
            #[allow(deprecated)]
            AsymmetricEncString::Rsa2048_OaepSha256_HmacSha256_B64 { .. } => 5,
            #[allow(deprecated)]
            AsymmetricEncString::Rsa2048_OaepSha1_HmacSha256_B64 { .. } => 6,
        }
    }
}

impl KeyDecryptable<AsymmetricCryptoKey, Vec<u8>> for AsymmetricEncString {
    fn decrypt_with_key(&self, key: &AsymmetricCryptoKey) -> Result<Vec<u8>> {
        use AsymmetricEncString::*;
        match self {
            Rsa2048_OaepSha256_B64 { data } => key.key.decrypt(Oaep::new::<sha2::Sha256>(), data),
            Rsa2048_OaepSha1_B64 { data } => key.key.decrypt(Oaep::new::<sha1::Sha1>(), data),
            #[allow(deprecated)]
            Rsa2048_OaepSha256_HmacSha256_B64 { data, .. } => {
                key.key.decrypt(Oaep::new::<sha2::Sha256>(), data)
            }
            #[allow(deprecated)]
            Rsa2048_OaepSha1_HmacSha256_B64 { data, .. } => {
                key.key.decrypt(Oaep::new::<sha1::Sha1>(), data)
            }
        }
        .map_err(|_| CryptoError::KeyDecrypt)
    }
}

impl KeyDecryptable<AsymmetricCryptoKey, String> for AsymmetricEncString {
    fn decrypt_with_key(&self, key: &AsymmetricCryptoKey) -> Result<String> {
        let dec: Vec<u8> = self.decrypt_with_key(key)?;
        String::from_utf8(dec).map_err(|_| CryptoError::InvalidUtf8String)
    }
}

/// Usually we wouldn't want to expose AsymmetricEncStrings in the API or the schemas.
/// But during the transition phase we will expose endpoints using the AsymmetricEncString type.
impl schemars::JsonSchema for AsymmetricEncString {
    fn schema_name() -> String {
        "AsymmetricEncString".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        gen.subschema_for::<String>()
    }
}

#[cfg(test)]
mod tests {
    use schemars::schema_for;

    use super::{AsymmetricCryptoKey, AsymmetricEncString, KeyDecryptable};

    const RSA_PRIVATE_KEY: &str = "-----BEGIN PRIVATE KEY-----
MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCXRVrCX+2hfOQS
8HzYUS2oc/jGVTZpv+/Ryuoh9d8ihYX9dd0cYh2tl6KWdFc88lPUH11Oxqy20Rk2
e5r/RF6T9yM0Me3NPnaKt+hlhLtfoc0h86LnhD56A9FDUfuI0dVnPcrwNv0YJIo9
4LwxtbqBULNvXl6wJ7WAbODrCQy5ZgMVg+iH+gGpwiqsZqHt+KuoHWcN53MSPDfa
F4/YMB99U3TziJMOOJask1TEEnakMPln11PczNDazT17DXIxYrbPfutPdh6sLs6A
QOajdZijfEvepgnOe7cQ7aeatiOJFrjTApKPGxOVRzEMX4XS4xbyhH0QxQeB6l16
l8C0uxIBAgMBAAECggEASaWfeVDA3cVzOPFSpvJm20OTE+R6uGOU+7vh36TX/POq
92qBuwbd0h0oMD32FxsXywd2IxtBDUSiFM9699qufTVuM0Q3tZw6lHDTOVG08+tP
dr8qSbMtw7PGFxN79fHLBxejjO4IrM9lapjWpxEF+11x7r+wM+0xRZQ8sNFYG46a
PfIaty4BGbL0I2DQ2y8I57iBCAy69eht59NLMm27fRWGJIWCuBIjlpfzET1j2HLX
UIh5bTBNzqaN039WH49HczGE3mQKVEJZc/efk3HaVd0a1Sjzyn0QY+N1jtZN3jTR
buDWA1AknkX1LX/0tUhuS3/7C3ejHxjw4Dk1ZLo5/QKBgQDIWvqFn0+IKRSu6Ua2
hDsufIHHUNLelbfLUMmFthxabcUn4zlvIscJO00Tq/ezopSRRvbGiqnxjv/mYxuc
vOUBeZtlus0Q9RTACBtw9TGoNTmQbEunJ2FOSlqbQxkBBAjgGEppRPt30iGj/VjA
hCATq2MYOa/X4dVR51BqQAFIEwKBgQDBSIfTFKC/hDk6FKZlgwvupWYJyU9Rkyfs
tPErZFmzoKhPkQ3YORo2oeAYmVUbS9I2iIYpYpYQJHX8jMuCbCz4ONxTCuSIXYQY
UcUq4PglCKp31xBAE6TN8SvhfME9/MvuDssnQinAHuF0GDAhF646T3LLS1not6Vs
zv7brwSoGwKBgQC88v/8cGfi80ssQZeMnVvq1UTXIeQcQnoY5lGHJl3K8mbS3TnX
E6c9j417Fdz+rj8KWzBzwWXQB5pSPflWcdZO886Xu/mVGmy9RWgLuVFhXwCwsVEP
jNX5ramRb0/vY0yzenUCninBsIxFSbIfrPtLUYCc4hpxr+sr2Mg/y6jpvQKBgBez
MRRs3xkcuXepuI2R+BCXL1/b02IJTUf1F+1eLLGd7YV0H+J3fgNc7gGWK51hOrF9
JBZHBGeOUPlaukmPwiPdtQZpu4QNE3l37VlIpKTF30E6mb+BqR+nht3rUjarnMXg
AoEZ18y6/KIjpSMpqC92Nnk/EBM9EYe6Cf4eA9ApAoGAeqEUg46UTlJySkBKURGp
Is3v1kkf5I0X8DnOhwb+HPxNaiEdmO7ckm8+tPVgppLcG0+tMdLjigFQiDUQk2y3
WjyxP5ZvXu7U96jaJRI8PFMoE06WeVYcdIzrID2HvqH+w0UQJFrLJ/0Mn4stFAEz
XKZBokBGnjFnTnKcs7nv/O8=
-----END PRIVATE KEY-----";

    #[test]
    fn test_enc_string_rsa2048_oaep_sha256_b64() {
        let private_key = AsymmetricCryptoKey::from_pem(RSA_PRIVATE_KEY).unwrap();
        let enc_str: &str = "3.YFqzW9LL/uLjCnl0RRLtndzGJ1FV27mcwQwGjfJPOVrgCX9nJSUYCCDd0iTIyOZ/zRxG47b6L1Z3qgkEfcxjmrSBq60gijc3E2TBMAg7OCLVcjORZ+i1sOVOudmOPWro6uA8refMrg4lqbieDlbLMzjVEwxfi5WpcL876cD0vYyRwvLO3bzFrsE7x33HHHtZeOPW79RqMn5efsB5Dj9wVheC9Ix9AYDjbo+rjg9qR6guwKmS7k2MSaIQlrDR7yu8LP+ePtiSjx+gszJV5jQGfcx60dtiLQzLS/mUD+RmU7B950Bpx0H7x56lT5yXZbWK5YkoP6qd8B8D2aKbP68Ywg==";
        let enc_string: AsymmetricEncString = enc_str.parse().unwrap();

        assert_eq!(enc_string.enc_type(), 3);

        let res: String = enc_string.decrypt_with_key(&private_key).unwrap();
        assert_eq!(res, "EncryptMe!");
    }

    #[test]
    fn test_enc_string_rsa2048_oaep_sha1_b64() {
        let private_key = AsymmetricCryptoKey::from_pem(RSA_PRIVATE_KEY).unwrap();
        let enc_str: &str = "4.ZheRb3PCfAunyFdQYPfyrFqpuvmln9H9w5nDjt88i5A7ug1XE0LJdQHCIYJl0YOZ1gCOGkhFu/CRY2StiLmT3iRKrrVBbC1+qRMjNNyDvRcFi91LWsmRXhONVSPjywzrJJXglsztDqGkLO93dKXNhuKpcmtBLsvgkphk/aFvxbaOvJ/FHdK/iV0dMGNhc/9tbys8laTdwBlI5xIChpRcrfH+XpSFM88+Bu03uK67N9G6eU1UmET+pISJwJvMuIDMqH+qkT7OOzgL3t6I0H2LDj+CnsumnQmDsvQzDiNfTR0IgjpoE9YH2LvPXVP2wVUkiTwXD9cG/E7XeoiduHyHjw==";
        let enc_string: AsymmetricEncString = enc_str.parse().unwrap();

        assert_eq!(enc_string.enc_type(), 4);

        let res: String = enc_string.decrypt_with_key(&private_key).unwrap();
        assert_eq!(res, "EncryptMe!");
    }

    #[test]
    fn test_enc_string_rsa2048_oaep_sha1_hmac_sha256_b64() {
        let private_key = AsymmetricCryptoKey::from_pem(RSA_PRIVATE_KEY).unwrap();
        let enc_str: &str = "6.ThnNc67nNr7GELyuhGGfsXNP2zJnNqhrIsjntEQ27r2qmn8vwdHbTbfO0cwt6YgSibDN0PjiCZ1O3Wb/IFq+vwvyRwFqF9145wBF8CQCbkhV+M0XvO99kh0daovtt120Nve/5ETI5PbPag9VdalKRQWZypJaqQHm5TAQVf4F5wtLlCLMBkzqTk+wkFe7BPMTGn07T+O3eJbTxXvyMZewQ7icJF0MZVA7VyWX9qElmZ89FCKowbf1BMr5pbcQ+0KdXcSVW3to43VkTp7k7COwsuH3M/i1AuVP5YN8ixjyRpvaeGqX/ap2nCHK2Wj5VxgCGT7XEls6ZknnAp9nB9qVjQ==|s3ntw5H/KKD/qsS0lUghTHl5Sm9j6m7YEdNHf0OeAFQ=";
        let enc_string: AsymmetricEncString = enc_str.parse().unwrap();

        assert_eq!(enc_string.enc_type(), 6);

        let res: String = enc_string.decrypt_with_key(&private_key).unwrap();
        assert_eq!(res, "EncryptMe!");
    }

    #[test]
    fn test_enc_string_serialization() {
        #[derive(serde::Serialize, serde::Deserialize)]
        struct Test {
            key: AsymmetricEncString,
        }

        let cipher = "6.ThnNc67nNr7GELyuhGGfsXNP2zJnNqhrIsjntEQ27r2qmn8vwdHbTbfO0cwt6YgSibDN0PjiCZ1O3Wb/IFq+vwvyRwFqF9145wBF8CQCbkhV+M0XvO99kh0daovtt120Nve/5ETI5PbPag9VdalKRQWZypJaqQHm5TAQVf4F5wtLlCLMBkzqTk+wkFe7BPMTGn07T+O3eJbTxXvyMZewQ7icJF0MZVA7VyWX9qElmZ89FCKowbf1BMr5pbcQ+0KdXcSVW3to43VkTp7k7COwsuH3M/i1AuVP5YN8ixjyRpvaeGqX/ap2nCHK2Wj5VxgCGT7XEls6ZknnAp9nB9qVjQ==|s3ntw5H/KKD/qsS0lUghTHl5Sm9j6m7YEdNHf0OeAFQ=";
        let serialized = format!("{{\"key\":\"{cipher}\"}}");

        let t = serde_json::from_str::<Test>(&serialized).unwrap();
        assert_eq!(t.key.enc_type(), 6);
        assert_eq!(t.key.to_string(), cipher);
        assert_eq!(serde_json::to_string(&t).unwrap(), serialized);
    }

    #[test]
    fn test_from_str_invalid() {
        let enc_str = "7.ABC";
        let enc_string: Result<AsymmetricEncString, _> = enc_str.parse();

        let err = enc_string.unwrap_err();
        assert_eq!(
            err.to_string(),
            "EncString error, Invalid asymmetric type, got type 7 with 1 parts"
        );
    }

    #[test]
    fn test_debug_format() {
        let enc_str: &str = "4.ZheRb3PCfAunyFdQYPfyrFqpuvmln9H9w5nDjt88i5A7ug1XE0LJdQHCIYJl0YOZ1gCOGkhFu/CRY2StiLmT3iRKrrVBbC1+qRMjNNyDvRcFi91LWsmRXhONVSPjywzrJJXglsztDqGkLO93dKXNhuKpcmtBLsvgkphk/aFvxbaOvJ/FHdK/iV0dMGNhc/9tbys8laTdwBlI5xIChpRcrfH+XpSFM88+Bu03uK67N9G6eU1UmET+pISJwJvMuIDMqH+qkT7OOzgL3t6I0H2LDj+CnsumnQmDsvQzDiNfTR0IgjpoE9YH2LvPXVP2wVUkiTwXD9cG/E7XeoiduHyHjw==";
        let enc_string: AsymmetricEncString = enc_str.parse().unwrap();

        let debug_string = format!("{:?}", enc_string);
        assert_eq!(debug_string, "AsymmetricEncString");
    }

    #[test]
    fn test_json_schema() {
        let schema = schema_for!(AsymmetricEncString);

        assert_eq!(
            serde_json::to_string(&schema).unwrap(),
            r#"{"$schema":"http://json-schema.org/draft-07/schema#","title":"AsymmetricEncString","type":"string"}"#
        );
    }
}
