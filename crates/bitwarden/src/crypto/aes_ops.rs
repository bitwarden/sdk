//! # AES operations
//!
//! Contains low level AES operations used by the rest of the library.
//!
//! **Warning**: Consider carefully if you have to use these functions directly, as generally we
//! expose higher level functions that are easier to use and more secure.
//!
//! In most cases you should use the [EncString] with [KeyEncryptable][super::KeyEncryptable] &
//! [KeyDecryptable][super::KeyDecryptable] instead.

use aes::cipher::{
    block_padding::Pkcs7, generic_array::GenericArray, typenum::U32, BlockDecryptMut,
    BlockEncryptMut, KeyIvInit,
};
use hmac::Mac;
use rand::RngCore;

use crate::{
    crypto::{EncString, PbkdfSha256Hmac, PBKDF_SHA256_HMAC_OUT_SIZE},
    error::{CryptoError, Result},
};

/// Decrypt using AES-256 in CBC mode.
///
/// Behaves similar to [decrypt_aes256_hmac], but does not validate the MAC.
pub fn decrypt_aes256(iv: &[u8; 16], data: Vec<u8>, key: GenericArray<u8, U32>) -> Result<Vec<u8>> {
    // Decrypt data
    let iv = GenericArray::from_slice(iv);
    let mut data = data;
    let decrypted_key_slice = cbc::Decryptor::<aes::Aes256>::new(&key, iv)
        .decrypt_padded_mut::<Pkcs7>(&mut data)
        .map_err(|_| CryptoError::KeyDecrypt)?;

    // Data is decrypted in place and returns a subslice of the original Vec, to avoid cloning it, we truncate to the subslice length
    let decrypted_len = decrypted_key_slice.len();
    data.truncate(decrypted_len);

    Ok(data)
}

/// Decrypt using AES-256 in CBC mode with MAC.
///
/// Behaves similar to [decrypt_aes256], but also validates the MAC.
pub fn decrypt_aes256_hmac(
    iv: &[u8; 16],
    mac: &[u8; 32],
    data: Vec<u8>,
    mac_key: GenericArray<u8, U32>,
    key: GenericArray<u8, U32>,
) -> Result<Vec<u8>> {
    let res = validate_mac(&mac_key, iv, &data)?;
    if res != *mac {
        return Err(CryptoError::InvalidMac.into());
    }
    decrypt_aes256(iv, data, key)
}

/// Encrypt using AES-256 in CBC mode.
///
/// Behaves similar to [encrypt_aes256_hmac], but does't generate a MAC.
///
/// ## Returns
///
/// A AesCbc256_B64 EncString
pub fn encrypt_aes256(data_dec: &[u8], key: GenericArray<u8, U32>) -> Result<EncString> {
    let (iv, data) = encrypt_aes256_internal(data_dec, key);

    Ok(EncString::AesCbc256_B64 { iv, data })
}

/// Encrypt using AES-256 in CBC mode with MAC.
///
/// Behaves similar to [encrypt_aes256], but also generate a MAC.
///
/// ## Returns
///
/// A AesCbc256_HmacSha256_B64 EncString
pub fn encrypt_aes256_hmac(
    data_dec: &[u8],
    mac_key: GenericArray<u8, U32>,
    key: GenericArray<u8, U32>,
) -> Result<EncString> {
    let (iv, data) = encrypt_aes256_internal(data_dec, key);
    let mac = validate_mac(&mac_key, &iv, &data)?;

    Ok(EncString::AesCbc256_HmacSha256_B64 { iv, mac, data })
}

/// Encrypt using AES-256 in CBC mode.
///
/// Used internally by:
/// - [encrypt_aes256]
/// - [encrypt_aes256_hmac]
fn encrypt_aes256_internal(data_dec: &[u8], key: GenericArray<u8, U32>) -> ([u8; 16], Vec<u8>) {
    let mut iv = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut iv);
    let data = cbc::Encryptor::<aes::Aes256>::new(&key, &iv.into())
        .encrypt_padded_vec_mut::<Pkcs7>(data_dec);

    (iv, data)
}

/// Validate a MAC using HMAC-SHA256.
fn validate_mac(mac_key: &[u8], iv: &[u8], data: &[u8]) -> Result<[u8; 32]> {
    let mut hmac = PbkdfSha256Hmac::new_from_slice(mac_key).expect("HMAC can take key of any size");
    hmac.update(iv);
    hmac.update(data);
    let mac: [u8; PBKDF_SHA256_HMAC_OUT_SIZE] = (*hmac.finalize().into_bytes())
        .try_into()
        .map_err(|_| CryptoError::InvalidMac)?;

    Ok(mac)
}
