use std::{collections::HashMap, str::FromStr};

use bitwarden_core::VaultLocked;
use bitwarden_crypto::{CryptoError, KeyContainer};
use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use reqwest::Url;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::CipherListView;

type HmacSha1 = Hmac<sha1::Sha1>;
type HmacSha256 = Hmac<sha2::Sha256>;
type HmacSha512 = Hmac<sha2::Sha512>;

const BASE32_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
const STEAM_CHARS: &str = "23456789BCDFGHJKMNPQRTVWXY";

const DEFAULT_ALGORITHM: Algorithm = Algorithm::Sha1;
const DEFAULT_DIGITS: u32 = 6;
const DEFAULT_PERIOD: u32 = 30;

#[derive(Debug, Error)]
pub enum TotpError {
    #[error("Invalid otpauth")]
    InvalidOtpauth,
    #[error("Missing secret")]
    MissingSecret,

    #[error(transparent)]
    CryptoError(#[from] CryptoError),
    #[error(transparent)]
    VaultLocked(#[from] VaultLocked),
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct TotpResponse {
    /// Generated TOTP code
    pub code: String,
    /// Time period
    pub period: u32,
}

/// Generate a OATH or RFC 6238 TOTP code from a provided key.
///
/// <https://datatracker.ietf.org/doc/html/rfc6238>
///
/// Key can be either:
/// - A base32 encoded string
/// - OTP Auth URI
/// - Steam URI
///
/// Supports providing an optional time, and defaults to current system time if none is provided.
///
/// Arguments:
/// - `key` - The key to generate the TOTP code from
/// - `time` - The time in UTC to generate the TOTP code for, defaults to current system time
pub fn generate_totp(key: String, time: Option<DateTime<Utc>>) -> Result<TotpResponse, TotpError> {
    let params: Totp = key.parse()?;

    let time = time.unwrap_or_else(Utc::now);

    let otp = params.derive_otp(time.timestamp());

    Ok(TotpResponse {
        code: otp,
        period: params.period,
    })
}

/// Generate a OATH or RFC 6238 TOTP code from a provided CipherListView.
///
/// See [generate_totp] for more information.
pub fn generate_totp_cipher_view(
    enc: &dyn KeyContainer,
    view: CipherListView,
    time: Option<DateTime<Utc>>,
) -> Result<TotpResponse, TotpError> {
    let key = view.get_totp_key(enc)?.ok_or(TotpError::MissingSecret)?;

    generate_totp(key, time)
}

#[derive(Clone, Copy, Debug)]
enum Algorithm {
    Sha1,
    Sha256,
    Sha512,
    Steam,
}

impl Algorithm {
    // Derive the HMAC hash for the given algorithm
    fn derive_hash(&self, key: &[u8], time: &[u8]) -> Vec<u8> {
        fn compute_digest<D: Mac>(digest: D, time: &[u8]) -> Vec<u8> {
            digest.chain_update(time).finalize().into_bytes().to_vec()
        }

        match self {
            Algorithm::Sha1 => compute_digest(
                HmacSha1::new_from_slice(key).expect("hmac new_from_slice should not fail"),
                time,
            ),
            Algorithm::Sha256 => compute_digest(
                HmacSha256::new_from_slice(key).expect("hmac new_from_slice should not fail"),
                time,
            ),
            Algorithm::Sha512 => compute_digest(
                HmacSha512::new_from_slice(key).expect("hmac new_from_slice should not fail"),
                time,
            ),
            Algorithm::Steam => compute_digest(
                HmacSha1::new_from_slice(key).expect("hmac new_from_slice should not fail"),
                time,
            ),
        }
    }
}

#[derive(Debug)]
struct Totp {
    algorithm: Algorithm,
    digits: u32,
    period: u32,
    secret: Vec<u8>,
}

impl Totp {
    fn derive_otp(&self, time: i64) -> String {
        let time = time / self.period as i64;

        let hash = self
            .algorithm
            .derive_hash(&self.secret, time.to_be_bytes().as_ref());
        let binary = derive_binary(hash);

        if let Algorithm::Steam = self.algorithm {
            derive_steam_otp(binary, self.digits)
        } else {
            let otp = binary % 10_u32.pow(self.digits);
            format!("{1:00$}", self.digits as usize, otp)
        }
    }
}

impl FromStr for Totp {
    type Err = TotpError;

    /// Parses the provided key and returns the corresponding `Totp`.
    ///
    /// Key can be either:
    /// - A base32 encoded string
    /// - OTP Auth URI
    /// - Steam URI
    fn from_str(key: &str) -> Result<Self, Self::Err> {
        let key = key.to_lowercase();

        let params = if key.starts_with("otpauth://") {
            let url = Url::parse(&key).map_err(|_| TotpError::InvalidOtpauth)?;
            let parts: HashMap<_, _> = url.query_pairs().collect();

            Totp {
                algorithm: parts
                    .get("algorithm")
                    .and_then(|v| match v.as_ref() {
                        "sha1" => Some(Algorithm::Sha1),
                        "sha256" => Some(Algorithm::Sha256),
                        "sha512" => Some(Algorithm::Sha512),
                        _ => None,
                    })
                    .unwrap_or(DEFAULT_ALGORITHM),
                digits: parts
                    .get("digits")
                    .and_then(|v| v.parse().ok())
                    .map(|v: u32| v.clamp(0, 10))
                    .unwrap_or(DEFAULT_DIGITS),
                period: parts
                    .get("period")
                    .and_then(|v| v.parse().ok())
                    .map(|v: u32| v.max(1))
                    .unwrap_or(DEFAULT_PERIOD),
                secret: decode_b32(
                    &parts
                        .get("secret")
                        .map(|v| v.to_string())
                        .ok_or(TotpError::MissingSecret)?,
                ),
            }
        } else if let Some(secret) = key.strip_prefix("steam://") {
            Totp {
                algorithm: Algorithm::Steam,
                digits: 5,
                period: DEFAULT_PERIOD,
                secret: decode_b32(secret),
            }
        } else {
            Totp {
                algorithm: DEFAULT_ALGORITHM,
                digits: DEFAULT_DIGITS,
                period: DEFAULT_PERIOD,
                secret: decode_b32(&key),
            }
        };

        Ok(params)
    }
}

/// Derive the Steam OTP from the hash with the given number of digits.
fn derive_steam_otp(binary: u32, digits: u32) -> String {
    let mut full_code = binary & 0x7fffffff;

    (0..digits)
        .map(|_| {
            let index = full_code as usize % STEAM_CHARS.len();
            let char = STEAM_CHARS
                .chars()
                .nth(index)
                .expect("Should always be within range");
            full_code /= STEAM_CHARS.len() as u32;
            char
        })
        .collect()
}

/// Derive the OTP from the hash with the given number of digits.
fn derive_binary(hash: Vec<u8>) -> u32 {
    let offset = (hash.last().unwrap_or(&0) & 15) as usize;

    ((hash[offset] & 127) as u32) << 24
        | (hash[offset + 1] as u32) << 16
        | (hash[offset + 2] as u32) << 8
        | hash[offset + 3] as u32
}

/// This code is migrated from our javascript implementation and is not technically a correct base32
/// decoder since we filter out various characters, and use exact chunking.
fn decode_b32(s: &str) -> Vec<u8> {
    let s = s.to_uppercase();

    let mut bits = String::new();
    for c in s.chars() {
        if let Some(i) = BASE32_CHARS.find(c) {
            bits.push_str(&format!("{:05b}", i));
        }
    }
    let mut bytes = Vec::new();

    for chunk in bits.as_bytes().chunks_exact(8) {
        let byte_str = std::str::from_utf8(chunk).expect("The value is a valid string");
        let byte = u8::from_str_radix(byte_str, 2).expect("The value is a valid binary string");
        bytes.push(byte);
    }

    bytes
}

#[cfg(test)]
mod tests {
    use bitwarden_crypto::{CryptoError, SymmetricCryptoKey};
    use chrono::Utc;
    use uuid::Uuid;

    use super::*;
    use crate::{cipher::cipher::CipherListViewType, CipherRepromptType};

    #[test]
    fn test_decode_b32() {
        let res = decode_b32("WQIQ25BRKZYCJVYP");
        assert_eq!(res, vec![180, 17, 13, 116, 49, 86, 112, 36, 215, 15]);

        let res = decode_b32("ABCD123");
        assert_eq!(res, vec![0, 68, 61]);
    }

    #[test]
    fn test_generate_totp() {
        let cases = vec![
            ("WQIQ25BRKZYCJVYP", "194506"), // valid base32
            ("wqiq25brkzycjvyp", "194506"), // lowercase
            ("PIUDISEQYA", "829846"),       // non padded
            ("PIUDISEQYA======", "829846"), // padded
            ("PIUD1IS!EQYA=", "829846"),    // sanitized
            // Steam
            ("steam://HXDMVJECJJWSRB3HWIZR4IFUGFTMXBOZ", "7W6CJ"),
            ("StEam://HXDMVJECJJWSRB3HWIZR4IFUGFTMXBOZ", "7W6CJ"),
            ("steam://ABCD123", "N26DF"),
            // Various weird lengths
            ("ddfdf", "932653"),
            ("HJSGFJHDFDJDJKSDFD", "000034"),
            ("xvdsfasdfasdasdghsgsdfg", "403786"),
            ("KAKFJWOSFJ12NWL", "093430"),
        ];

        let time = Some(
            DateTime::parse_from_rfc3339("2023-01-01T00:00:00.000Z")
                .unwrap()
                .with_timezone(&Utc),
        );

        for (key, expected_code) in cases {
            let response = generate_totp(key.to_string(), time).unwrap();

            assert_eq!(response.code, expected_code, "wrong code for key: {key}");
            assert_eq!(response.period, 30);
        }
    }

    #[test]
    fn test_generate_otpauth() {
        let key = "otpauth://totp/test-account?secret=WQIQ25BRKZYCJVYP".to_string();
        let time = Some(
            DateTime::parse_from_rfc3339("2023-01-01T00:00:00.000Z")
                .unwrap()
                .with_timezone(&Utc),
        );
        let response = generate_totp(key, time).unwrap();

        assert_eq!(response.code, "194506".to_string());
        assert_eq!(response.period, 30);
    }

    #[test]
    fn test_generate_otpauth_uppercase() {
        let key = "OTPauth://totp/test-account?secret=WQIQ25BRKZYCJVYP".to_string();
        let time = Some(
            DateTime::parse_from_rfc3339("2023-01-01T00:00:00.000Z")
                .unwrap()
                .with_timezone(&Utc),
        );
        let response = generate_totp(key, time).unwrap();

        assert_eq!(response.code, "194506".to_string());
        assert_eq!(response.period, 30);
    }

    #[test]
    fn test_generate_otpauth_period() {
        let key = "otpauth://totp/test-account?secret=WQIQ25BRKZYCJVYP&period=60".to_string();
        let time = Some(
            DateTime::parse_from_rfc3339("2023-01-01T00:00:00.000Z")
                .unwrap()
                .with_timezone(&Utc),
        );
        let response = generate_totp(key, time).unwrap();

        assert_eq!(response.code, "730364".to_string());
        assert_eq!(response.period, 60);
    }

    #[test]
    fn test_generate_otpauth_algorithm_sha256() {
        let key = "otpauth://totp/test-account?secret=WQIQ25BRKZYCJVYP&algorithm=SHA256".to_string();
        let time = Some(
            DateTime::parse_from_rfc3339("2023-01-01T00:00:00.000Z")
                .unwrap()
                .with_timezone(&Utc),
        );
        let response = generate_totp(key, time).unwrap();

        assert_eq!(response.code, "842615".to_string());
        assert_eq!(response.period, 30);
    }

    #[test]
    fn test_generate_totp_cipher_view() {
        let view = CipherListView {
            id: Some("090c19ea-a61a-4df6-8963-262b97bc6266".parse().unwrap()),
            organization_id: None,
            folder_id: None,
            collection_ids: vec![],
            key: None,
            name: "My test login".to_string(),
            sub_title: "test_username".to_string(),
            r#type: CipherListViewType::Login {
                has_fido2: true,
                totp: Some("2.hqdioUAc81FsKQmO1XuLQg==|oDRdsJrQjoFu9NrFVy8tcJBAFKBx95gHaXZnWdXbKpsxWnOr2sKipIG43pKKUFuq|3gKZMiboceIB5SLVOULKg2iuyu6xzos22dfJbvx0EHk=".parse().unwrap()),
            },
            favorite: false,
            reprompt: CipherRepromptType::None,
            edit: true,
            view_password: true,
            attachments: 0,
            creation_date: "2024-01-30T17:55:36.150Z".parse().unwrap(),
            deleted_date: None,
            revision_date: "2024-01-30T17:55:36.150Z".parse().unwrap(),
        };

        struct MockKeyContainer(SymmetricCryptoKey);
        impl KeyContainer for MockKeyContainer {
            fn get_key<'a>(
                &'a self,
                _: &Option<Uuid>,
            ) -> Result<&'a SymmetricCryptoKey, CryptoError> {
                Ok(&self.0)
            }
        }

        let enc = MockKeyContainer("w2LO+nwV4oxwswVYCxlOfRUseXfvU03VzvKQHrqeklPgiMZrspUe6sOBToCnDn9Ay0tuCBn8ykVVRb7PWhub2Q==".to_string().try_into().unwrap());
        let time = DateTime::parse_from_rfc3339("2023-01-01T00:00:00.000Z")
            .unwrap()
            .with_timezone(&Utc);

        let response = generate_totp_cipher_view(&enc, view, Some(time)).unwrap();
        assert_eq!(response.code, "559388".to_string());
        assert_eq!(response.period, 30);
    }
}
