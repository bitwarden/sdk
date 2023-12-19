mod asymmetric;
mod symmetric;

use std::str::FromStr;

pub use asymmetric::AsymmEncString;
use base64::Engine;
pub use symmetric::EncString;

use crate::{
    error::{EncStringParseError, Result},
    util::BASE64_ENGINE,
};

#[cfg(feature = "mobile")]
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

fn from_b64_vec(s: &str) -> Result<Vec<u8>> {
    Ok(BASE64_ENGINE
        .decode(s)
        .map_err(EncStringParseError::InvalidBase64)?)
}

fn from_b64<const N: usize>(s: &str) -> Result<[u8; N]> {
    Ok(from_b64_vec(s)?
        .try_into()
        .map_err(|e: Vec<_>| EncStringParseError::InvalidLength {
            expected: N,
            got: e.len(),
        })?)
}

fn split_enc_string(s: &str) -> (&str, Vec<&str>) {
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
}

struct FromStrVisitor<T>(std::marker::PhantomData<T>);
impl<T> FromStrVisitor<T> {
    fn new() -> Self {
        Self(Default::default())
    }
}
impl<T: FromStr> serde::de::Visitor<'_> for FromStrVisitor<T>
where
    T::Err: std::fmt::Debug,
{
    type Value = T;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "a valid string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        T::from_str(v).map_err(|e| E::custom(format!("{:?}", e)))
    }
}
