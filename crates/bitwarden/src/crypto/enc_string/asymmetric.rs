use std::{fmt::Display, str::FromStr};

use base64::Engine;
#[cfg(feature = "internal")]
use rsa::{Oaep, RsaPrivateKey};
use serde::{de::Visitor, Deserialize};

use crate::{
    error::{EncStringParseError, Error, Result},
    util::BASE64_ENGINE,
};

#[cfg(feature = "internal")]
use crate::error::CryptoError;

use super::{from_b64_vec, split_enc_string};

/// # Encrypted string primitive
///
/// [AsymmEncString] is a Bitwarden specific primitive that represents an asymmetrically encrypted string. They are
/// are used together with the KeyDecryptable and KeyEncryptable traits to encrypt and decrypt
/// data using AsymmetricCryptoKeys.
///
/// The flexibility of the [AsymmEncString] type allows for different encryption algorithms to be used
/// which is represented by the different variants of the enum.
///
/// ## Note
///
/// For backwards compatibility we will rarely if ever be able to remove support for decrypting old
/// variants, but we should be opinionated in which variants are used for encrypting.
///
/// ## Variants
/// - [Rsa2048_OaepSha256_B64](AsymmEncString::Rsa2048_OaepSha256_B64)
/// - [Rsa2048_OaepSha1_B64](AsymmEncString::Rsa2048_OaepSha1_B64)
///
/// ## Serialization
///
/// [AsymmEncString] implements [Display] and [FromStr] to allow for easy serialization and uses a
/// custom scheme to represent the different variants.
///
/// The scheme is one of the following schemes:
/// - `[type].[data]`
///
/// Where:
/// - `[type]`: is a digit number representing the variant.
/// - `[data]`: is the encrypted data.
#[derive(Clone)]
#[allow(unused, non_camel_case_types)]
pub enum AsymmEncString {
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

/// To avoid printing sensitive information, [AsymmEncString] debug prints to `AsymmEncString`.
impl std::fmt::Debug for AsymmEncString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsymmEncString").finish()
    }
}

/// Deserializes an [AsymmEncString] from a string.
impl FromStr for AsymmEncString {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (enc_type, parts) = split_enc_string(s);
        match (enc_type, parts.len()) {
            ("3", 1) => {
                let data = from_b64_vec(parts[0])?;
                Ok(AsymmEncString::Rsa2048_OaepSha256_B64 { data })
            }
            ("4", 1) => {
                let data = from_b64_vec(parts[0])?;
                Ok(AsymmEncString::Rsa2048_OaepSha1_B64 { data })
            }
            #[allow(deprecated)]
            ("5", 2) => {
                let data = from_b64_vec(parts[0])?;
                let mac: Vec<u8> = from_b64_vec(parts[1])?;
                Ok(AsymmEncString::Rsa2048_OaepSha256_HmacSha256_B64 { data, mac })
            }
            #[allow(deprecated)]
            ("6", 2) => {
                let data = from_b64_vec(parts[0])?;
                let mac: Vec<u8> = from_b64_vec(parts[1])?;
                Ok(AsymmEncString::Rsa2048_OaepSha1_HmacSha256_B64 { data, mac })
            }

            (enc_type, parts) => Err(EncStringParseError::InvalidType {
                enc_type: enc_type.to_string(),
                parts,
            }
            .into()),
        }
    }
}

#[allow(unused)]
impl AsymmEncString {
    /// TODO: Convert this to a trait method
    #[cfg(feature = "internal")]
    pub(crate) fn decrypt_with_private_key(&self, key: &RsaPrivateKey) -> Result<Vec<u8>> {
        Ok(match self {
            Self::Rsa2048_OaepSha256_B64 { data } => key.decrypt(Oaep::new::<sha2::Sha256>(), data),
            Self::Rsa2048_OaepSha1_B64 { data } => key.decrypt(Oaep::new::<sha1::Sha1>(), data),
            #[allow(deprecated)]
            Self::Rsa2048_OaepSha256_HmacSha256_B64 { data, mac: _ } => {
                key.decrypt(Oaep::new::<sha2::Sha256>(), data)
            }
            #[allow(deprecated)]
            Self::Rsa2048_OaepSha1_HmacSha256_B64 { data, mac: _ } => {
                key.decrypt(Oaep::new::<sha1::Sha1>(), data)
            }
        }
        .map_err(|_| CryptoError::KeyDecrypt)?)
    }
}

impl Display for AsymmEncString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parts: Vec<&[u8]> = match self {
            AsymmEncString::Rsa2048_OaepSha256_B64 { data } => vec![data],
            AsymmEncString::Rsa2048_OaepSha1_B64 { data } => vec![data],
            #[allow(deprecated)]
            AsymmEncString::Rsa2048_OaepSha256_HmacSha256_B64 { data, mac } => vec![data, mac],
            #[allow(deprecated)]
            AsymmEncString::Rsa2048_OaepSha1_HmacSha256_B64 { data, mac } => vec![data, mac],
        };

        let encoded_parts: Vec<String> = parts
            .iter()
            .map(|part| BASE64_ENGINE.encode(part))
            .collect();

        write!(f, "{}.{}", self.enc_type(), encoded_parts.join("|"))?;

        Ok(())
    }
}

impl<'de> Deserialize<'de> for AsymmEncString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct CSVisitor;
        impl Visitor<'_> for CSVisitor {
            type Value = AsymmEncString;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "a valid string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                AsymmEncString::from_str(v).map_err(|e| E::custom(format!("{:?}", e)))
            }
        }

        deserializer.deserialize_str(CSVisitor)
    }
}

impl serde::Serialize for AsymmEncString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl AsymmEncString {
    /// The numerical representation of the encryption type of the [AsymmEncString].
    const fn enc_type(&self) -> u8 {
        match self {
            AsymmEncString::Rsa2048_OaepSha256_B64 { .. } => 3,
            AsymmEncString::Rsa2048_OaepSha1_B64 { .. } => 4,
            #[allow(deprecated)]
            AsymmEncString::Rsa2048_OaepSha256_HmacSha256_B64 { .. } => 5,
            #[allow(deprecated)]
            AsymmEncString::Rsa2048_OaepSha1_HmacSha256_B64 { .. } => 6,
        }
    }
}

/// Usually we wouldn't want to expose AsymmEncStrings in the API or the schemas.
/// But during the transition phase we will expose endpoints using the AsymmEncString type.
impl schemars::JsonSchema for AsymmEncString {
    fn schema_name() -> String {
        "AsymmEncString".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        gen.subschema_for::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::AsymmEncString;

    #[cfg(feature = "internal")]
    #[test]
    fn test_enc_string_rsa2048_oaep_sha1_hmac_sha256_b64() {
        use rsa::{pkcs8::DecodePrivateKey, RsaPrivateKey};

        let rsa_private_key: &str = "-----BEGIN PRIVATE KEY-----
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
        let private_key = RsaPrivateKey::from_pkcs8_pem(rsa_private_key).unwrap();
        let enc_str: &str = "6.ThnNc67nNr7GELyuhGGfsXNP2zJnNqhrIsjntEQ27r2qmn8vwdHbTbfO0cwt6YgSibDN0PjiCZ1O3Wb/IFq+vwvyRwFqF9145wBF8CQCbkhV+M0XvO99kh0daovtt120Nve/5ETI5PbPag9VdalKRQWZypJaqQHm5TAQVf4F5wtLlCLMBkzqTk+wkFe7BPMTGn07T+O3eJbTxXvyMZewQ7icJF0MZVA7VyWX9qElmZ89FCKowbf1BMr5pbcQ+0KdXcSVW3to43VkTp7k7COwsuH3M/i1AuVP5YN8ixjyRpvaeGqX/ap2nCHK2Wj5VxgCGT7XEls6ZknnAp9nB9qVjQ==|s3ntw5H/KKD/qsS0lUghTHl5Sm9j6m7YEdNHf0OeAFQ=";
        let enc_string: AsymmEncString = enc_str.parse().unwrap();

        assert_eq!(enc_string.enc_type(), 6);

        let res = enc_string.decrypt_with_private_key(&private_key).unwrap();

        assert_eq!(std::str::from_utf8(&res).unwrap(), "EncryptMe!");
    }

    #[test]
    fn test_enc_string_serialization() {
        #[derive(serde::Serialize, serde::Deserialize)]
        struct Test {
            key: AsymmEncString,
        }

        let cipher = "6.ThnNc67nNr7GELyuhGGfsXNP2zJnNqhrIsjntEQ27r2qmn8vwdHbTbfO0cwt6YgSibDN0PjiCZ1O3Wb/IFq+vwvyRwFqF9145wBF8CQCbkhV+M0XvO99kh0daovtt120Nve/5ETI5PbPag9VdalKRQWZypJaqQHm5TAQVf4F5wtLlCLMBkzqTk+wkFe7BPMTGn07T+O3eJbTxXvyMZewQ7icJF0MZVA7VyWX9qElmZ89FCKowbf1BMr5pbcQ+0KdXcSVW3to43VkTp7k7COwsuH3M/i1AuVP5YN8ixjyRpvaeGqX/ap2nCHK2Wj5VxgCGT7XEls6ZknnAp9nB9qVjQ==|s3ntw5H/KKD/qsS0lUghTHl5Sm9j6m7YEdNHf0OeAFQ=";
        let serialized = format!("{{\"key\":\"{cipher}\"}}");

        let t = serde_json::from_str::<Test>(&serialized).unwrap();
        assert_eq!(t.key.enc_type(), 6);
        assert_eq!(t.key.to_string(), cipher);
        assert_eq!(serde_json::to_string(&t).unwrap(), serialized);
    }
}
