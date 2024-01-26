use std::pin::Pin;

use ::aes::cipher::{ArrayLength, Unsigned};
use generic_array::GenericArray;
use hmac::digest::OutputSizeUser;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::{CryptoError, Result};

pub(crate) type PbkdfSha256Hmac = hmac::Hmac<sha2::Sha256>;
pub(crate) const PBKDF_SHA256_HMAC_OUT_SIZE: usize =
    <<PbkdfSha256Hmac as OutputSizeUser>::OutputSize as Unsigned>::USIZE;

/// [RFC5869](https://datatracker.ietf.org/doc/html/rfc5869) HKDF-Expand operation
pub(crate) fn hkdf_expand<T: ArrayLength<u8>>(
    prk: &[u8],
    info: Option<&str>,
) -> Result<Pin<Box<GenericArray<u8, T>>>> {
    let hkdf = hkdf::Hkdf::<sha2::Sha256>::from_prk(prk).map_err(|_| CryptoError::InvalidKeyLen)?;
    let mut key = Box::<GenericArray<u8, T>>::default();

    let i = info.map(|i| i.as_bytes()).unwrap_or(&[]);
    hkdf.expand(i, &mut key)
        .map_err(|_| CryptoError::InvalidKeyLen)?;

    Ok(Box::into_pin(key))
}

/// Generate random bytes that are cryptographically secure
pub fn generate_random_bytes<T>() -> T
where
    Standard: Distribution<T>,
{
    rand::thread_rng().gen()
}

pub fn pbkdf2(password: &[u8], salt: &[u8], rounds: u32) -> [u8; PBKDF_SHA256_HMAC_OUT_SIZE] {
    pbkdf2::pbkdf2_array::<PbkdfSha256Hmac, PBKDF_SHA256_HMAC_OUT_SIZE>(password, salt, rounds)
        .expect("hash is a valid fixed size")
}

#[cfg(test)]
mod tests {
    use aes::cipher::typenum::U64;

    use super::*;

    #[test]
    fn test_hkdf_expand() {
        let prk = &[
            23, 152, 120, 41, 214, 16, 156, 133, 71, 226, 178, 135, 208, 255, 66, 101, 189, 70,
            173, 30, 39, 215, 175, 236, 38, 180, 180, 62, 196, 4, 159, 70,
        ];
        let info = Some("info");

        let result: Pin<Box<GenericArray<u8, U64>>> = hkdf_expand(prk, info).unwrap();

        let expected_output: [u8; 64] = [
            6, 114, 42, 38, 87, 231, 30, 109, 30, 255, 104, 129, 255, 94, 92, 108, 124, 145, 215,
            208, 17, 60, 135, 22, 70, 158, 40, 53, 45, 182, 8, 63, 65, 87, 239, 234, 185, 227, 153,
            122, 115, 205, 144, 56, 102, 149, 92, 139, 217, 102, 119, 57, 37, 57, 251, 178, 18, 52,
            94, 77, 132, 215, 239, 100,
        ];

        assert_eq!(result.as_slice(), expected_output);
    }
}
