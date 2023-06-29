//! Cryptographic primitives used in the SDK

use std::{collections::HashMap, fmt::Display, hash::Hash, str::FromStr};

use aes::cipher::{
    generic_array::GenericArray,
    typenum::{U32, U64},
    Unsigned,
};
use base64::Engine;
use hmac::digest::OutputSizeUser;
use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;
use serde::{de::Visitor, Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

pub use crate::client::encryption_settings::{decrypt, encrypt_aes256, SymmetricCryptoKey};
use crate::{
    client::{auth_settings::Kdf, encryption_settings::EncryptionSettings},
    error::{CSParseError, Error, Result},
    util::BASE64_ENGINE,
    wordlist::EFF_LONG_WORD_LIST,
};

#[allow(unused, non_camel_case_types)]
#[derive(Clone)]
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

impl schemars::JsonSchema for CipherString {
    fn schema_name() -> String {
        "CipherString".to_owned()
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        gen.subschema_for::<String>()
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
    kdf: &Kdf,
) -> Result<(GenericArray<u8, U32>, GenericArray<u8, U32>), hkdf::InvalidLength> {
    let master_key = match kdf {
        Kdf::PBKDF2 { iterations } => pbkdf2::pbkdf2_array::<
            PbkdfSha256Hmac,
            PBKDF_SHA256_HMAC_OUT_SIZE,
        >(secret, salt, iterations.get())
        .unwrap(),
        Kdf::Argon2id {
            iterations,
            memory,
            parallelism,
        } => todo!("Argon2id not implemented"),
    };

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

pub(crate) fn fingerprint(fingerprint_material: &str, public_key: &[u8]) -> Result<String> {
    let mut h = Sha256::new();
    h.update(public_key);
    h.finalize();

    let hkdf = hkdf::Hkdf::<sha2::Sha256>::from_prk(public_key)
        .map_err(|_| Error::Internal("hkdf::InvalidLength"))?;

    let mut user_fingerprint = [0u8; 32];
    hkdf.expand(fingerprint_material.as_bytes(), &mut user_fingerprint)
        .map_err(|_| Error::Internal("hkdf::expand"))?;

    Ok(hash_word(user_fingerprint).unwrap())
}

fn hash_word(hash: [u8; 32]) -> Result<String> {
    let minimum_entropy = 64;

    let entropy_per_word = (EFF_LONG_WORD_LIST.len() as f64).log2();
    let num_words = ((minimum_entropy as f64 / entropy_per_word).ceil() as f64).to_owned() as i64;

    let hash_arr: Vec<u8> = hash.to_vec();
    let entropy_available = hash_arr.len() * 4;
    if num_words as f64 * entropy_per_word > entropy_available as f64 {
        return Err(Error::Internal(
            "Output entropy of hash function is too small",
        ));
    }

    let mut phrase = Vec::new();

    let mut hash_number = BigUint::from_bytes_be(&hash_arr);
    for _ in 0..num_words {
        let remainder = hash_number.clone() % EFF_LONG_WORD_LIST.len();
        hash_number = hash_number / EFF_LONG_WORD_LIST.len();

        phrase.push(EFF_LONG_WORD_LIST[remainder.to_usize().unwrap()].to_string());
    }

    Ok(phrase.join("-"))
}

pub trait Encryptable<Output> {
    fn encrypt(self, enc: &EncryptionSettings, org_id: &Option<Uuid>) -> Result<Output>;
}

pub trait Decryptable<Output> {
    fn decrypt(&self, enc: &EncryptionSettings, org_id: &Option<Uuid>) -> Result<Output>;
}

impl Encryptable<CipherString> for String {
    fn encrypt(self, enc: &EncryptionSettings, org_id: &Option<Uuid>) -> Result<CipherString> {
        enc.encrypt(self.as_bytes(), org_id)
    }
}

impl Decryptable<String> for CipherString {
    fn decrypt(&self, enc: &EncryptionSettings, org_id: &Option<Uuid>) -> Result<String> {
        enc.decrypt(self, org_id)
    }
}

impl<T: Encryptable<Output>, Output> Encryptable<Option<Output>> for Option<T> {
    fn encrypt(self, enc: &EncryptionSettings, org_id: &Option<Uuid>) -> Result<Option<Output>> {
        self.map(|e| e.encrypt(enc, org_id)).transpose()
    }
}

impl<T: Decryptable<Output>, Output> Decryptable<Option<Output>> for Option<T> {
    fn decrypt(&self, enc: &EncryptionSettings, org_id: &Option<Uuid>) -> Result<Option<Output>> {
        self.as_ref().map(|e| e.decrypt(enc, org_id)).transpose()
    }
}

impl<T: Encryptable<Output>, Output> Encryptable<Vec<Output>> for Vec<T> {
    fn encrypt(self, enc: &EncryptionSettings, org_id: &Option<Uuid>) -> Result<Vec<Output>> {
        self.into_iter().map(|e| e.encrypt(enc, org_id)).collect()
    }
}

impl<T: Decryptable<Output>, Output> Decryptable<Vec<Output>> for Vec<T> {
    fn decrypt(&self, enc: &EncryptionSettings, org_id: &Option<Uuid>) -> Result<Vec<Output>> {
        self.into_iter().map(|e| e.decrypt(enc, org_id)).collect()
    }
}

impl<T: Encryptable<Output>, Output, Id: Hash + Eq> Encryptable<HashMap<Id, Output>>
    for HashMap<Id, T>
{
    fn encrypt(
        self,
        enc: &EncryptionSettings,
        org_id: &Option<Uuid>,
    ) -> Result<HashMap<Id, Output>> {
        self.into_iter()
            .map(|(id, e)| Ok((id, e.encrypt(enc, org_id)?)))
            .collect::<Result<HashMap<_, _>>>()
    }
}

impl<T: Decryptable<Output>, Output, Id: Hash + Eq + Clone> Decryptable<HashMap<Id, Output>>
    for HashMap<Id, T>
{
    fn decrypt(
        &self,
        enc: &EncryptionSettings,
        org_id: &Option<Uuid>,
    ) -> Result<HashMap<Id, Output>> {
        self.into_iter().map(|(id, e)| Ok(((id.to_owned()), e.decrypt(enc, org_id)?)))
            .collect::<Result<HashMap<_, _>>>()
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU32;

    use super::{fingerprint, stretch_key};
    use crate::{
        client::auth_settings::Kdf,
        crypto::{stretch_key_password, CipherString},
    };

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
            &Kdf::PBKDF2 {
                iterations: NonZeroU32::new(10000).unwrap(),
            },
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

    #[test]
    fn test_fingerprint() {
        let user_id = "a09726a0-9590-49d1-a5f5-afe300b6a515";
        let key: &[u8] = &[
            48, 130, 1, 34, 48, 13, 6, 9, 42, 134, 72, 134, 247, 13, 1, 1, 1, 5, 0, 3, 130, 1, 15,
            0, 48, 130, 1, 10, 2, 130, 1, 1, 0, 187, 38, 44, 241, 110, 205, 89, 253, 25, 191, 126,
            84, 121, 202, 61, 223, 189, 244, 118, 212, 74, 139, 130, 97, 115, 164, 167, 106, 191,
            188, 233, 218, 196, 250, 187, 146, 125, 160, 150, 49, 198, 224, 176, 10, 0, 143, 99,
            230, 232, 160, 51, 104, 154, 211, 33, 80, 170, 4, 68, 80, 219, 115, 167, 114, 156, 227,
            125, 193, 128, 123, 39, 254, 191, 124, 63, 129, 44, 63, 18, 56, 161, 48, 158, 0, 27,
            146, 2, 99, 136, 75, 21, 135, 6, 118, 12, 26, 251, 184, 172, 249, 53, 78, 210, 46, 143,
            17, 104, 202, 65, 173, 229, 219, 233, 144, 163, 101, 216, 238, 152, 54, 158, 1, 195,
            50, 203, 21, 226, 12, 82, 170, 175, 170, 160, 21, 247, 248, 80, 97, 123, 0, 152, 116,
            229, 126, 221, 199, 155, 194, 192, 51, 207, 177, 240, 160, 84, 241, 41, 88, 176, 53,
            111, 28, 173, 177, 232, 158, 22, 79, 133, 152, 31, 32, 12, 196, 147, 58, 57, 50, 252,
            208, 131, 150, 179, 132, 178, 150, 234, 251, 143, 125, 163, 144, 20, 46, 71, 168, 252,
            164, 86, 120, 124, 56, 252, 206, 210, 236, 212, 139, 127, 189, 236, 40, 46, 2, 238, 13,
            216, 40, 48, 85, 133, 229, 181, 155, 176, 217, 241, 154, 153, 213, 112, 222, 72, 219,
            197, 3, 219, 56, 77, 109, 47, 72, 251, 131, 36, 240, 96, 169, 31, 82, 93, 166, 242, 3,
            33, 213, 2, 3, 1, 0, 1,
        ];

        assert_eq!(
            "turban-deftly-anime-chatroom-unselfish",
            fingerprint(user_id, key).unwrap()
        );
    }
}
