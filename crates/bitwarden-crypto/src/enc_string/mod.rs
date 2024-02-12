/// Encrypted string types
mod asymmetric;
mod symmetric;

use std::str::FromStr;

pub use asymmetric::AsymmetricEncString;
use base64::{engine::general_purpose::STANDARD, Engine};
pub use symmetric::EncString;

use crate::error::{EncStringParseError, Result};

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
    Ok(STANDARD
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_length_less_than_expected() {
        let buf = [1, 2, 3];
        let expected = 5;
        let result = check_length(&buf, expected);
        assert!(result.is_err());
    }

    #[test]
    fn test_check_length_equal_to_expected() {
        let buf = [1, 2, 3, 4, 5];
        let expected = 5;
        let result = check_length(&buf, expected);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_length_greater_than_expected() {
        let buf = [1, 2, 3, 4, 5, 6];
        let expected = 5;
        let result = check_length(&buf, expected);
        assert!(result.is_ok());
    }

    #[test]
    fn test_split_enc_string_new_format() {
        let s = "2.abc|def|ghi";
        let (header, parts) = split_enc_string(s);
        assert_eq!(header, "2");
        assert_eq!(parts, vec!["abc", "def", "ghi"]);
    }

    #[test]
    fn test_split_enc_string_old_format_three_parts() {
        let s = "abc|def|ghi";
        let (header, parts) = split_enc_string(s);
        assert_eq!(header, "1");
        assert_eq!(parts, vec!["abc", "def", "ghi"]);
    }

    #[test]
    fn test_split_enc_string_old_format_fewer_parts() {
        let s = "abc|def";
        let (header, parts) = split_enc_string(s);
        assert_eq!(header, "0");
        assert_eq!(parts, vec!["abc", "def"]);
    }
}
