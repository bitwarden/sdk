//! # AES operations
//!
//! Contains low level AES operations used by the rest of the library.
//!
//! In most cases you should use the [EncString][crate::EncString] with
//! [KeyEncryptable][crate::KeyEncryptable] & [KeyDecryptable][crate::KeyDecryptable] instead.

use aes::cipher::{
    block_padding::Pkcs7, typenum::U32, BlockDecryptMut, BlockEncryptMut, KeyIvInit,
};
use generic_array::GenericArray;
use hmac::Mac;
use subtle::ConstantTimeEq;

use crate::{
    error::{CryptoError, Result},
    util::{PbkdfSha256Hmac, PBKDF_SHA256_HMAC_OUT_SIZE},
};

/// Decrypt using AES-256 in CBC mode.
///
/// Behaves similar to [decrypt_aes256_hmac], but does not validate the MAC.
pub(crate) fn decrypt_aes256(
    iv: &[u8; 16],
    data: Vec<u8>,
    key: &GenericArray<u8, U32>,
) -> Result<Vec<u8>> {
    // Decrypt data
    let iv = GenericArray::from_slice(iv);
    let mut data = data;
    let decrypted_key_slice = cbc::Decryptor::<aes::Aes256>::new(key, iv)
        .decrypt_padded_mut::<Pkcs7>(&mut data)
        .map_err(|_| CryptoError::KeyDecrypt)?;

    // Data is decrypted in place and returns a subslice of the original Vec, to avoid cloning it,
    // we truncate to the subslice length
    let decrypted_len = decrypted_key_slice.len();
    data.truncate(decrypted_len);

    Ok(data)
}

/// Decrypt using AES-256 in CBC mode with MAC.
///
/// Behaves similar to [decrypt_aes256], but also validates the MAC.
pub(crate) fn decrypt_aes256_hmac(
    iv: &[u8; 16],
    mac: &[u8; 32],
    data: Vec<u8>,
    mac_key: &GenericArray<u8, U32>,
    key: &GenericArray<u8, U32>,
) -> Result<Vec<u8>> {
    let res = generate_mac(mac_key, iv, &data)?;
    if res.ct_ne(mac).into() {
        return Err(CryptoError::InvalidMac);
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
pub(crate) fn encrypt_aes256(data_dec: &[u8], key: &GenericArray<u8, U32>) -> ([u8; 16], Vec<u8>) {
    let rng = rand::thread_rng();
    let (iv, data) = encrypt_aes256_internal(rng, data_dec, key);

    (iv, data)
}

/// Encrypt using AES-256 in CBC mode with MAC.
///
/// Behaves similar to [encrypt_aes256], but also generate a MAC.
///
/// ## Returns
///
/// A AesCbc256_HmacSha256_B64 EncString
pub(crate) fn encrypt_aes256_hmac(
    data_dec: &[u8],
    mac_key: &GenericArray<u8, U32>,
    key: &GenericArray<u8, U32>,
) -> Result<([u8; 16], [u8; 32], Vec<u8>)> {
    let rng = rand::thread_rng();
    let (iv, data) = encrypt_aes256_internal(rng, data_dec, key);
    let mac = generate_mac(mac_key, &iv, &data)?;

    Ok((iv, mac, data))
}

/// Encrypt using AES-256 in CBC mode.
///
/// Used internally by:
/// - [encrypt_aes256]
/// - [encrypt_aes256_hmac]
fn encrypt_aes256_internal(
    mut rng: impl rand::RngCore,
    data_dec: &[u8],
    key: &GenericArray<u8, U32>,
) -> ([u8; 16], Vec<u8>) {
    let mut iv = [0u8; 16];
    rng.fill_bytes(&mut iv);
    let data = cbc::Encryptor::<aes::Aes256>::new(key, &iv.into())
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
    use base64::{engine::general_purpose::STANDARD, Engine};
    use generic_array::sequence::GenericSequence;
    use rand::SeedableRng;

    use super::*;

    /// Helper function for generating a `GenericArray` of size 32 with each element being
    /// a multiple of a given increment, starting from a given offset.
    fn generate_generic_array(offset: u8, increment: u8) -> GenericArray<u8, U32> {
        GenericArray::generate(|i| offset + i as u8 * increment)
    }

    /// Helper function for generating a vector of a given size with each element being
    /// a multiple of a given increment, starting from a given offset.
    fn generate_vec(length: usize, offset: u8, increment: u8) -> Vec<u8> {
        (0..length).map(|i| offset + i as u8 * increment).collect()
    }

    #[test]
    fn test_encrypt_aes256_internal() {
        let key = generate_generic_array(0, 1);

        let rng = rand_chacha::ChaCha8Rng::from_seed([0u8; 32]);
        let result = encrypt_aes256_internal(rng, "EncryptMe!".as_bytes(), &key);
        assert_eq!(
            result,
            (
                [62, 0, 239, 47, 137, 95, 64, 214, 127, 91, 184, 232, 31, 9, 165, 161],
                vec![214, 76, 187, 97, 58, 146, 212, 140, 95, 164, 177, 204, 179, 133, 172, 148]
            )
        );
    }

    #[test]
    fn test_generate_mac() {
        let mac_key = generate_vec(16, 0, 16);

        let iv = generate_vec(16, 0, 16);
        let data = generate_vec(16, 0, 16);

        let result = generate_mac(&mac_key, &iv, &data);

        assert!(result.is_ok());
        let mac = result.unwrap();
        assert_eq!(mac.len(), 32);
    }

    #[test]
    fn test_decrypt_aes256() {
        let iv = generate_vec(16, 0, 1);
        let iv: &[u8; 16] = iv.as_slice().try_into().unwrap();
        let key = generate_generic_array(0, 1);
        let data = STANDARD.decode("ByUF8vhyX4ddU9gcooznwA==").unwrap();

        let decrypted = decrypt_aes256(iv, data, &key).unwrap();

        assert_eq!(String::from_utf8(decrypted).unwrap(), "EncryptMe!");
    }

    #[test]
    fn test_encrypt_decrypt_aes256() {
        let key = generate_generic_array(0, 1);
        let data = "EncryptMe!";

        let (iv, encrypted) = encrypt_aes256(data.as_bytes(), &key);
        let decrypted = decrypt_aes256(&iv, encrypted, &key).unwrap();

        assert_eq!(String::from_utf8(decrypted).unwrap(), "EncryptMe!");
    }
}
