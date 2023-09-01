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

pub fn decrypt_aes256(
    iv: &[u8; 16],
    mac: Option<&[u8; 32]>,
    data: Vec<u8>,
    mac_key: Option<GenericArray<u8, U32>>,
    key: GenericArray<u8, U32>,
) -> Result<Vec<u8>> {
    match (mac_key, mac) {
        (Some(mac_key), Some(mac)) => {
            // Validate HMAC
            let res = validate_mac(&mac_key, iv, &data)?;
            if res != *mac {
                return Err(CryptoError::InvalidMac.into());
            }
        }
        (None, None) => { /* No mac provided, so we don't do MAC checking */ }
        _ => return Err(CryptoError::InvalidMac.into()),
    };

    // Decrypt data
    let iv = GenericArray::from_slice(iv);
    let mut data = data;
    let decrypted_key_slice = cbc::Decryptor::<aes::Aes256>::new(&key, iv)
        .decrypt_padded_mut::<Pkcs7>(&mut data)
        .map_err(|_| CryptoError::KeyDecrypt)?;

    //Data is decrypted in place and returns a subslice of the original Vec, to avoid cloning it, we truncate to the subslice length
    let decrypted_len = decrypted_key_slice.len();
    data.truncate(decrypted_len);

    Ok(data)
}

pub fn encrypt_aes256(
    data_dec: &[u8],
    mac_key: Option<GenericArray<u8, U32>>,
    key: GenericArray<u8, U32>,
) -> Result<EncString> {
    let mac_key = match mac_key {
        Some(k) => k,
        None => return Err(CryptoError::InvalidMac.into()),
    };

    let mut iv = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut iv);
    let data = cbc::Encryptor::<aes::Aes256>::new(&key, &iv.into())
        .encrypt_padded_vec_mut::<Pkcs7>(data_dec);

    let mac = validate_mac(&mac_key, &iv, &data)?;

    Ok(EncString::AesCbc256_HmacSha256_B64 { iv, mac, data })
}

fn validate_mac(mac_key: &[u8], iv: &[u8], data: &[u8]) -> Result<[u8; 32]> {
    let mut hmac = PbkdfSha256Hmac::new_from_slice(mac_key).expect("HMAC can take key of any size");
    hmac.update(iv);
    hmac.update(data);
    let mac: [u8; PBKDF_SHA256_HMAC_OUT_SIZE] = (*hmac.finalize().into_bytes())
        .try_into()
        .map_err(|_| CryptoError::InvalidMac)?;

    Ok(mac)
}
