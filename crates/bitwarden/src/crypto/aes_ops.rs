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
use subtle::ConstantTimeEq;

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
    let res = generate_mac(&mac_key, iv, &data)?;
    if res.ct_ne(mac).into() {
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
#[allow(unused)]
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
    let mac = generate_mac(&mac_key, &iv, &data)?;

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

/// Generate a MAC using HMAC-SHA256.
fn generate_mac(mac_key: &[u8], iv: &[u8], data: &[u8]) -> Result<[u8; 32]> {
    let mut hmac = PbkdfSha256Hmac::new_from_slice(mac_key).expect("HMAC can take key of any size");
    hmac.update(iv);
    hmac.update(data);
    let mac: [u8; PBKDF_SHA256_HMAC_OUT_SIZE] = (*hmac.finalize().into_bytes())
        .try_into()
        .map_err(|_| CryptoError::InvalidMac)?;

    Ok(mac)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_mac() {
        let key = &[
            195, 14, 183, 53, 190, 121, 107, 16, 20, 192, 149, 196, 224, 9, 130, 104, 238, 8, 50,
            45,
        ];
        let iv = &[
            226, 6, 27, 67, 114, 12, 246, 255, 192, 90, 129, 21, 247, 200, 238, 154,
        ];
        let data = &[
            153, 234, 143, 119, 19, 195, 39, 51, 7, 16, 185, 219, 162, 85, 48, 247, 21, 126, 142,
            37, 0, 157, 107, 216, 98, 218, 128, 173, 18, 126, 0, 254, 87, 178, 169,
        ];
        let mac = generate_mac(key, iv, data).unwrap();
        assert_eq!(
            mac,
            [
                186, 117, 103, 89, 51, 36, 29, 13, 170, 14, 241, 155, 239, 212, 159, 78, 157, 45,
                165, 62, 233, 108, 125, 175, 153, 49, 110, 184, 74, 226, 77, 1
            ]
        );
    }
}
