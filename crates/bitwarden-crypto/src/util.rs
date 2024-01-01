use crate::{CryptoError, Result};
use ::aes::cipher::{generic_array::GenericArray, ArrayLength, Unsigned};
use hmac::digest::OutputSizeUser;

pub(crate) type PbkdfSha256Hmac = hmac::Hmac<sha2::Sha256>;
pub(crate) const PBKDF_SHA256_HMAC_OUT_SIZE: usize =
    <<PbkdfSha256Hmac as OutputSizeUser>::OutputSize as Unsigned>::USIZE;

/// RFC5869 HKDF-Expand operation
pub(crate) fn hkdf_expand<T: ArrayLength<u8>>(
    prk: &[u8],
    info: Option<&str>,
) -> Result<GenericArray<u8, T>> {
    let hkdf = hkdf::Hkdf::<sha2::Sha256>::from_prk(prk).map_err(|_| CryptoError::InvalidKeyLen)?;
    let mut key = GenericArray::<u8, T>::default();

    let i = info.map(|i| i.as_bytes()).unwrap_or(&[]);
    hkdf.expand(i, &mut key)
        .map_err(|_| CryptoError::InvalidKeyLen)?;

    Ok(key)
}
