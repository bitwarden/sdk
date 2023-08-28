use aes::cipher::{
    block_padding::Pkcs7,
    generic_array::GenericArray,
    typenum::{U32, U64},
    BlockEncryptMut, KeyIvInit,
};
use rand::RngCore;

use crate::error::Result;

use super::CipherString;

pub fn encrypt_aes256(data_dec: &[u8], key: GenericArray<u8, U32>) -> Result<CipherString> {
    let mut iv = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut iv);
    let data = cbc::Encryptor::<aes::Aes256>::new(&key, &iv.into())
        .encrypt_padded_vec_mut::<Pkcs7>(data_dec);

    Ok(CipherString::AesCbc256_B64 { iv, data })
}
