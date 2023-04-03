//! Cryptographic primitives used in the SDK

use std::{fmt::Display, num::NonZeroU32, str::FromStr};

use aes::cipher::{
    generic_array::GenericArray,
    typenum::{U32, U64},
    Unsigned,
};
use base64::Engine;
use hmac::digest::OutputSizeUser;
use serde::{de::Visitor, Deserialize, Serialize};

pub use crate::client::encryption_settings::{decrypt, encrypt_aes256, SymmetricCryptoKey};
use crate::{
    error::{CSParseError, Error, Result},
    util::BASE64_ENGINE,
};

#[allow(unused, non_camel_case_types)]
pub enum CipherString {
    // 0
    AesCbc256_B64 {
        iv: [u8; 16],
        data: Vec<u8>,
    },
    // 1
    AesCbc128_HmacSha256_B64 {
        iv: [u8; 16],
        mac: [u8; 32],
        data: Vec<u8>,
    },
    // 2
    AesCbc256_HmacSha256_B64 {
        iv: [u8; 16],
        mac: [u8; 32],
        data: Vec<u8>,
    },
    // 3
    Rsa2048_OaepSha256_B64 {
        data: Vec<u8>,
    },
    // 4
    Rsa2048_OaepSha1_B64 {
        data: Vec<u8>,
    },
    // 5
    Rsa2048_OaepSha256_HmacSha256_B64 {
        mac: [u8; 32],
        data: Vec<u8>,
    },
    // 6
    Rsa2048_OaepSha1_HmacSha256_B64 {
        mac: [u8; 32],
        data: Vec<u8>,
    },
}

// We manually implement these to make sure we don't print any sensitive data
impl std::fmt::Debug for CipherString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CipherString").finish()
    }
}

fn invalid_len_error(expected: usize) -> impl Fn(Vec<u8>) -> CSParseError {
    move |e: Vec<_>| CSParseError::InvalidBase64Length {
        expected,
        got: e.len(),
    }
}

impl FromStr for CipherString {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (enc_type, data) = s.split_once('.').ok_or(CSParseError::NoType)?;

        let parts: Vec<_> = data.split('|').collect();
        match (enc_type, parts.len()) {
            ("0", 2) => unimplemented!(),

            ("1" | "2", 3) => {
                let iv_str = parts[0];
                let data_str = parts[1];
                let mac_str = parts[2];

                let iv = BASE64_ENGINE
                    .decode(iv_str)
                    .map_err(CSParseError::InvalidBase64)?
                    .try_into()
                    .map_err(invalid_len_error(16))?;

                let mac = BASE64_ENGINE
                    .decode(mac_str)
                    .map_err(CSParseError::InvalidBase64)?
                    .try_into()
                    .map_err(invalid_len_error(32))?;

                let data = BASE64_ENGINE
                    .decode(data_str)
                    .map_err(CSParseError::InvalidBase64)?;

                if enc_type == "1" {
                    Ok(CipherString::AesCbc128_HmacSha256_B64 { iv, mac, data })
                } else {
                    Ok(CipherString::AesCbc256_HmacSha256_B64 { iv, mac, data })
                }
            }

            ("3" | "4", 1) => {
                let data = BASE64_ENGINE
                    .decode(data)
                    .map_err(CSParseError::InvalidBase64)?;
                if enc_type == "3" {
                    Ok(CipherString::Rsa2048_OaepSha256_B64 { data })
                } else {
                    Ok(CipherString::Rsa2048_OaepSha1_B64 { data })
                }
            }
            ("5" | "6", 2) => {
                unimplemented!()
            }

            (enc_type, parts) => Err(CSParseError::InvalidType {
                enc_type: enc_type.to_string(),
                parts,
            }
            .into()),
        }
    }
}

impl Display for CipherString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.", self.enc_type())?;

        let mut parts = Vec::<&[u8]>::new();

        match self {
            CipherString::AesCbc256_B64 { iv, data } => {
                parts.push(iv);
                parts.push(data);
            }
            CipherString::AesCbc128_HmacSha256_B64 { iv, mac, data } => {
                parts.push(iv);
                parts.push(data);
                parts.push(mac);
            }
            CipherString::AesCbc256_HmacSha256_B64 { iv, mac, data } => {
                parts.push(iv);
                parts.push(data);
                parts.push(mac);
            }
            CipherString::Rsa2048_OaepSha256_B64 { data } => {
                parts.push(data);
            }
            CipherString::Rsa2048_OaepSha1_B64 { data } => {
                parts.push(data);
            }
            CipherString::Rsa2048_OaepSha256_HmacSha256_B64 { mac, data } => {
                parts.push(data);
                parts.push(mac);
            }
            CipherString::Rsa2048_OaepSha1_HmacSha256_B64 { mac, data } => {
                parts.push(data);
                parts.push(mac);
            }
        }

        for i in 0..parts.len() {
            if i == parts.len() - 1 {
                write!(f, "{}", BASE64_ENGINE.encode(parts[i]))?;
            } else {
                write!(f, "{}|", BASE64_ENGINE.encode(parts[i]))?;
            }
        }

        Ok(())
    }
}

impl<'de> Deserialize<'de> for CipherString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct CSVisitor;
        impl Visitor<'_> for CSVisitor {
            type Value = CipherString;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "A valid string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                CipherString::from_str(v).map_err(|e| E::custom(format!("{:?}", e)))
            }
        }

        deserializer.deserialize_str(CSVisitor)
    }
}

impl Serialize for CipherString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl CipherString {
    fn enc_type(&self) -> u8 {
        match self {
            CipherString::AesCbc256_B64 { .. } => 0,
            CipherString::AesCbc128_HmacSha256_B64 { .. } => 1,
            CipherString::AesCbc256_HmacSha256_B64 { .. } => 2,
            CipherString::Rsa2048_OaepSha256_B64 { .. } => 3,
            CipherString::Rsa2048_OaepSha1_B64 { .. } => 4,
            CipherString::Rsa2048_OaepSha256_HmacSha256_B64 { .. } => 5,
            CipherString::Rsa2048_OaepSha1_HmacSha256_B64 { .. } => 6,
        }
    }
}

pub(crate) type PbkdfSha256Hmac = hmac::Hmac<sha2::Sha256>;
pub(crate) const PBKDF_SHA256_HMAC_OUT_SIZE: usize =
    <<PbkdfSha256Hmac as OutputSizeUser>::OutputSize as Unsigned>::USIZE;

pub(crate) fn stretch_key_password(
    secret: &[u8],
    salt: &[u8],
    iterations: NonZeroU32,
) -> Result<(GenericArray<u8, U32>, GenericArray<u8, U32>), hkdf::InvalidLength> {
    let master_key = pbkdf2::pbkdf2_array::<PbkdfSha256Hmac, PBKDF_SHA256_HMAC_OUT_SIZE>(
        secret,
        salt,
        iterations.get(),
    )
    .unwrap();

    let hkdf =
        hkdf::Hkdf::<sha2::Sha256>::from_prk(&master_key).map_err(|_| hkdf::InvalidLength)?;

    let mut key = GenericArray::default();
    hkdf.expand("enc".as_bytes(), &mut key)?;
    let mut mac_key = GenericArray::default();
    hkdf.expand("mac".as_bytes(), &mut mac_key)?;

    Ok((key, mac_key))
}

pub(crate) fn stretch_key(secret: [u8; 16], name: &str, info: Option<&str>) -> SymmetricCryptoKey {
    use hmac::{Hmac, Mac};
    // Because all inputs are fixed size, we can unwrap all errors here without issue

    // TODO: Are these the final `key` and `info` parameters or should we change them? I followed the pattern used for sends
    let res = Hmac::<sha2::Sha256>::new_from_slice(format!("bitwarden-{}", name).as_bytes())
        .unwrap()
        .chain_update(&secret)
        .finalize()
        .into_bytes();

    let hkdf = hkdf::Hkdf::<sha2::Sha256>::from_prk(&res).unwrap();

    let mut key = GenericArray::<u8, U64>::default();

    // TODO: Should we have a default value for info?
    //  Should it be required?
    let i = info.map(|i| i.as_bytes()).unwrap_or(&[]);
    hkdf.expand(&i, &mut key).unwrap();

    SymmetricCryptoKey::try_from(key.as_slice()).unwrap()
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU32;

    use crate::crypto::{stretch_key_password, CipherString};

    use super::stretch_key;

    #[test]
    fn test_cipher_string_serialization() {
        #[derive(serde::Serialize, serde::Deserialize)]
        struct Test {
            key: CipherString,
        }

        let cipher = "2.pMS6/icTQABtulw52pq2lg==|XXbxKxDTh+mWiN1HjH2N1w==|Q6PkuT+KX/axrgN9ubD5Ajk2YNwxQkgs3WJM0S0wtG8=";
        let serialized = format!("{{\"key\":\"{cipher}\"}}");

        let t = serde_json::from_str::<Test>(&serialized).unwrap();
        assert_eq!(t.key.enc_type(), 2);
        assert_eq!(t.key.to_string(), cipher);
        assert_eq!(serde_json::to_string(&t).unwrap(), serialized);
    }

    #[test]
    fn test_key_stretch() {
        let key = stretch_key(*b"&/$%F1a895g67HlX", "test_key", None);
        assert_eq!(key.to_base64(), "4PV6+PcmF2w7YHRatvyMcVQtI7zvCyssv/wFWmzjiH6Iv9altjmDkuBD1aagLVaLezbthbSe+ktR+U6qswxNnQ==");

        let key = stretch_key(*b"67t9b5g67$%Dh89n", "test_key", Some("test"));
        assert_eq!(key.to_base64(), "F9jVQmrACGx9VUPjuzfMYDjr726JtL300Y3Yg+VYUnVQtQ1s8oImJ5xtp1KALC9h2nav04++1LDW4iFD+infng==");

        let (key, mac) = stretch_key_password(
            &b"67t9b5g67$%Dh89n"[..],
            "test_key".as_bytes(),
            NonZeroU32::new(10000).unwrap(),
        )
        .unwrap();

        assert_eq!(
            key.as_slice(),
            [
                111, 31, 178, 45, 238, 152, 37, 114, 143, 215, 124, 83, 135, 173, 195, 23, 142,
                134, 120, 249, 61, 132, 163, 182, 113, 197, 189, 204, 188, 21, 237, 96
            ]
        );
        assert_eq!(
            mac.as_slice(),
            [
                221, 127, 206, 234, 101, 27, 202, 38, 86, 52, 34, 28, 78, 28, 185, 16, 48, 61, 127,
                166, 209, 247, 194, 87, 232, 26, 48, 85, 193, 249, 179, 155
            ]
        );
    }
}
