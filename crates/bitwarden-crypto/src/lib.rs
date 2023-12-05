use ::aes::cipher::{generic_array::GenericArray, ArrayLength, Unsigned};
use base64::{
    alphabet,
    engine::{DecodePaddingMode, GeneralPurpose, GeneralPurposeConfig},
};
use hmac::digest::OutputSizeUser;

pub mod aes;
mod error;
pub mod shareable_key;
pub mod symmetric_crypto_key;

pub use error::CryptoError;
use error::Result;

// TODO: Move into a util crate
const BASE64_ENGINE_CONFIG: GeneralPurposeConfig = GeneralPurposeConfig::new()
    .with_encode_padding(true)
    .with_decode_padding_mode(DecodePaddingMode::Indifferent);

pub const BASE64_ENGINE: GeneralPurpose =
    GeneralPurpose::new(&alphabet::STANDARD, BASE64_ENGINE_CONFIG);

pub(crate) type PbkdfSha256Hmac = hmac::Hmac<sha2::Sha256>;
pub(crate) const PBKDF_SHA256_HMAC_OUT_SIZE: usize =
    <<PbkdfSha256Hmac as OutputSizeUser>::OutputSize as Unsigned>::USIZE;

/// RFC5869 HKDF-Expand operation
fn hkdf_expand<T: ArrayLength<u8>>(prk: &[u8], info: Option<&str>) -> Result<GenericArray<u8, T>> {
    let hkdf = hkdf::Hkdf::<sha2::Sha256>::from_prk(prk).map_err(|_| CryptoError::InvalidKeyLen)?;
    let mut key = GenericArray::<u8, T>::default();

    let i = info.map(|i| i.as_bytes()).unwrap_or(&[]);
    hkdf.expand(i, &mut key)
        .map_err(|_| CryptoError::InvalidKeyLen)?;

    Ok(key)
}
