use std::collections::HashMap;

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

#[derive(Clone, Copy, Debug)]
enum TotpAlgorithm {
    Sha1,
    Sha256,
    Sha512,
    Steam,
}

#[derive(Debug)]
struct TotpParams {
    algorithm: TotpAlgorithm,
    digits: u32,
    period: u32,
    secret: String,
}

impl Default for TotpParams {
    fn default() -> Self {
        Self {
            algorithm: TotpAlgorithm::Sha1,
            digits: 6,
            period: 30,
            secret: "".to_string(),
        }
    }
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
pub async fn generate_totp(key: String, time: Option<DateTime<Utc>>) -> Result<TotpResponse> {
    let params = get_params(key)?;

    // TODO: Should we swap the expected time to timestamp?
    let time = time.unwrap_or_else(Utc::now);
    print!("{:?}", params);

    let t = time.timestamp() / params.period as i64;
    let secret = BASE32.decode(params.secret.as_ref()).map_err(|e| {
        println!("{:?}", e);
        Error::Internal("Unable to decode secret")
    })?;

    let hash = derive_hash(params.algorithm, &secret, t.to_be_bytes().as_ref())?;
    let binary = derive_binary(hash);

    let otp = if let TotpAlgorithm::Steam = params.algorithm {
        derive_steam_otp(binary, params.digits)
    } else {
        let otp = binary % 10_u32.pow(params.digits);
        format!("{1:00$}", params.digits as usize, otp)
    };

    Ok(TotpResponse {
        code: otp,
        period: params.period,
    })
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

/// Parses the provided key and returns the corresponding `TotpParams`.
///
/// Key can be either:
/// - A base32 encoded string
/// - OTP Auth URI
/// - Steam URI
fn get_params(key: String) -> Result<TotpParams> {
    let params = if key.starts_with("otpauth://") {
        let url = Url::parse(&key).map_err(|_| Error::Internal("Unable to parse URL"))?;
        let parts: HashMap<_, _> = url.query_pairs().collect();

        let defaults = TotpParams::default();

        TotpParams {
            algorithm: parts
                .get("algorithm")
                .and_then(|v| match v.to_uppercase().as_ref() {
                    "SHA1" => Some(TotpAlgorithm::Sha1),
                    "SHA256" => Some(TotpAlgorithm::Sha256),
                    "SHA512" => Some(TotpAlgorithm::Sha512),
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
        TotpParams {
            algorithm: TotpAlgorithm::Steam,
            digits: 5,
            secret: key
                .strip_prefix("steam://")
                .expect("Prefix is defined")
                .to_string(),
            ..TotpParams::default()
        }
    } else {
        TotpParams {
            secret: key,
            ..TotpParams::default()
        }
    };

    Ok(params)
}

/// Derive the OTP from the hash with the given number of digits.
fn derive_binary(hash: Vec<u8>) -> u32 {
    let offset = (hash.last().unwrap_or(&0) & 15) as usize;

    ((hash[offset] & 127) as u32) << 24
        | (hash[offset + 1] as u32) << 16
        | (hash[offset + 2] as u32) << 8
        | hash[offset + 3] as u32
}

impl From<aes::cipher::InvalidLength> for Error {
    fn from(_: aes::cipher::InvalidLength) -> Self {
        Error::Internal("Invalid length")
    }
}

// Derive the HMAC hash for the given algorithm
fn derive_hash(algorithm: TotpAlgorithm, key: &[u8], time: &[u8]) -> Result<Vec<u8>> {
    fn compute_digest<D: Mac>(mut digest: D, time: &[u8]) -> Vec<u8> {
        digest.update(time);
        digest.finalize().into_bytes().to_vec()
    }

    Ok(match algorithm {
        TotpAlgorithm::Sha1 => compute_digest(HmacSha1::new_from_slice(key)?, time),
        TotpAlgorithm::Sha256 => compute_digest(HmacSha256::new_from_slice(key)?, time),
        TotpAlgorithm::Sha512 => compute_digest(HmacSha512::new_from_slice(key)?, time),
        TotpAlgorithm::Steam => compute_digest(HmacSha1::new_from_slice(key)?, time),
    })
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
