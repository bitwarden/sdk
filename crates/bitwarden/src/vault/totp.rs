use std::{collections::HashMap, str::FromStr};

use crate::error::{Error, Result};
use chrono::{DateTime, Utc};
use data_encoding::BASE32;
use hmac::{Hmac, Mac};
use reqwest::Url;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

type HmacSha1 = Hmac<sha1::Sha1>;
type HmacSha256 = Hmac<sha2::Sha256>;
type HmacSha512 = Hmac<sha2::Sha512>;

const STEAM_CHARS: &str = "23456789BCDFGHJKMNPQRTVWXY";

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
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
pub async fn generate_totp(key: String, time: Option<DateTime<Utc>>) -> Result<TotpResponse> {
    let params: Totp = key.parse()?;

    let time = time.unwrap_or_else(Utc::now);

    let otp = params.derive_otp(time.timestamp())?;

    Ok(TotpResponse {
        code: otp,
        period: params.period,
    })
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
    fn derive_hash(&self, key: &[u8], time: &[u8]) -> Result<Vec<u8>> {
        fn compute_digest<D: Mac>(mut digest: D, time: &[u8]) -> Vec<u8> {
            digest.update(time);
            digest.finalize().into_bytes().to_vec()
        }

        Ok(match self {
            Algorithm::Sha1 => compute_digest(HmacSha1::new_from_slice(key)?, time),
            Algorithm::Sha256 => compute_digest(HmacSha256::new_from_slice(key)?, time),
            Algorithm::Sha512 => compute_digest(HmacSha512::new_from_slice(key)?, time),
            Algorithm::Steam => compute_digest(HmacSha1::new_from_slice(key)?, time),
        })
    }
}

#[derive(Debug)]
struct Totp {
    algorithm: Algorithm,
    digits: u32,
    period: u32,
    secret: String,
}

impl Default for Totp {
    fn default() -> Self {
        Self {
            algorithm: Algorithm::Sha1,
            digits: 6,
            period: 30,
            secret: "".to_string(),
        }
    }
}

impl Totp {
    fn derive_otp(&self, time: i64) -> Result<String> {
        let time = time / self.period as i64;

        let secret = BASE32.decode(self.secret.as_ref()).map_err(|e| {
            println!("{:?}", e);
            Error::Internal("Unable to decode secret")
        })?;

        let hash = self
            .algorithm
            .derive_hash(&secret, time.to_be_bytes().as_ref())?;
        let binary = derive_binary(hash);

        Ok(if let Algorithm::Steam = self.algorithm {
            derive_steam_otp(binary, self.digits)
        } else {
            let otp = binary % 10_u32.pow(self.digits);
            format!("{1:00$}", self.digits as usize, otp)
        })
    }
}

impl FromStr for Totp {
    type Err = Error;

    /// Parses the provided key and returns the corresponding `Totp`.
    ///
    /// Key can be either:
    /// - A base32 encoded string
    /// - OTP Auth URI
    /// - Steam URI
    fn from_str(key: &str) -> Result<Self> {
        let params = if key.starts_with("otpauth://") {
            let url = Url::parse(key).map_err(|_| Error::Internal("Unable to parse URL"))?;
            let parts: HashMap<_, _> = url.query_pairs().collect();

            let defaults = Totp::default();

            Totp {
                algorithm: parts
                    .get("algorithm")
                    .and_then(|v| match v.to_uppercase().as_ref() {
                        "SHA1" => Some(Algorithm::Sha1),
                        "SHA256" => Some(Algorithm::Sha256),
                        "SHA512" => Some(Algorithm::Sha512),
                        _ => None,
                    })
                    .unwrap_or(defaults.algorithm),
                digits: parts
                    .get("digits")
                    .and_then(|v| v.parse().ok())
                    .map(|v: u32| v.clamp(0, 10))
                    .unwrap_or(defaults.digits),
                period: parts
                    .get("period")
                    .and_then(|v| v.parse().ok())
                    .map(|v: u32| v.max(1))
                    .unwrap_or(defaults.period),
                secret: parts
                    .get("secret")
                    .map(|v| v.to_string())
                    .unwrap_or(defaults.secret),
            }
        } else if key.starts_with("steam://") {
            Totp {
                algorithm: Algorithm::Steam,
                digits: 5,
                secret: key
                    .strip_prefix("steam://")
                    .expect("Prefix is defined")
                    .to_string(),
                ..Totp::default()
            }
        } else {
            Totp {
                secret: key.to_string(),
                ..Totp::default()
            }
        };

        Ok(params)
    }
}

/// Derive the Steam OTP from the hash with the given number of digits.
fn derive_steam_otp(binary: u32, digits: u32) -> String {
    let mut otp = String::new();

    let mut full_code = binary & 0x7fffffff;
    for _ in 0..digits {
        otp.push(
            STEAM_CHARS
                .chars()
                .nth(full_code as usize % STEAM_CHARS.len())
                .expect("Should always be within range"),
        );
        full_code /= STEAM_CHARS.len() as u32;
    }

    otp
}

/// Derive the OTP from the hash with the given number of digits.
fn derive_binary(hash: Vec<u8>) -> u32 {
    let offset = (hash.last().unwrap_or(&0) & 15) as usize;

    ((hash[offset] & 127) as u32) << 24
        | (hash[offset + 1] as u32) << 16
        | (hash[offset + 2] as u32) << 8
        | hash[offset + 3] as u32
}

/// Convert from the dependency `InvalidLength` error into this crate's `Error`.
impl From<aes::cipher::InvalidLength> for Error {
    fn from(_: aes::cipher::InvalidLength) -> Self {
        Error::Internal("Invalid length")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[tokio::test]
    async fn test_generate_totp() {
        let key = "WQIQ25BRKZYCJVYP".to_string();
        let time = Some(
            DateTime::parse_from_rfc3339("2023-01-01T00:00:00.000Z")
                .unwrap()
                .with_timezone(&Utc),
        );
        let response = generate_totp(key, time).await.unwrap();

        assert_eq!(response.code, "194506".to_string());
        assert_eq!(response.period, 30);
    }

    #[tokio::test]
    async fn test_generate_otpauth() {
        let key = "otpauth://totp/test-account?secret=WQIQ25BRKZYCJVYP".to_string();
        let time = Some(
            DateTime::parse_from_rfc3339("2023-01-01T00:00:00.000Z")
                .unwrap()
                .with_timezone(&Utc),
        );
        let response = generate_totp(key, time).await.unwrap();

        assert_eq!(response.code, "194506".to_string());
        assert_eq!(response.period, 30);
    }

    #[tokio::test]
    async fn test_generate_otpauth_period() {
        let key = "otpauth://totp/test-account?secret=WQIQ25BRKZYCJVYP&period=60".to_string();
        let time = Some(
            DateTime::parse_from_rfc3339("2023-01-01T00:00:00.000Z")
                .unwrap()
                .with_timezone(&Utc),
        );
        let response = generate_totp(key, time).await.unwrap();

        assert_eq!(response.code, "730364".to_string());
        assert_eq!(response.period, 60);
    }

    #[tokio::test]
    async fn test_generate_steam() {
        let key = "steam://HXDMVJECJJWSRB3HWIZR4IFUGFTMXBOZ".to_string();
        let time = Some(
            DateTime::parse_from_rfc3339("2023-01-01T00:00:00.000Z")
                .unwrap()
                .with_timezone(&Utc),
        );
        let response = generate_totp(key, time).await.unwrap();

        assert_eq!(response.code, "7W6CJ".to_string());
        assert_eq!(response.period, 30);
    }
}
