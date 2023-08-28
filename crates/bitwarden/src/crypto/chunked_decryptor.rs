use aes::{
    cipher::{generic_array::GenericArray, Unsigned},
    Aes256,
};
use cbc::Decryptor;
use hmac::Mac;

use crate::{
    crypto::{PbkdfSha256Hmac, SymmetricCryptoKey},
    error::{EncStringParseError, Error, Result},
};

#[cfg(feature = "mobile")]
use {
    crate::error::CryptoError,
    aes::{
        cipher::{
            block_padding::{Padding, Pkcs7},
            BlockDecryptMut, BlockSizeUser, KeyIvInit,
        },
        Aes128,
    },
    hmac::digest::CtOutput,
};

#[allow(unused, non_camel_case_types, clippy::large_enum_variant)]
pub enum EncryptionType {
    // 0
    AesCbc256_B64 {
        iv: [u8; 16],
    },
    // 1
    AesCbc128_HmacSha256_B64 {
        iv: [u8; 16],
        mac: [u8; 32],
        hmac: PbkdfSha256Hmac,
    },
    // 2
    AesCbc256_HmacSha256_B64 {
        iv: [u8; 16],
        mac: [u8; 32],
        hmac: PbkdfSha256Hmac,
        decryptor: Decryptor<Aes256>,
    },
}

// To avoid issues, we need to make sure this is bigger or equal than all the ciphers block sizes
#[cfg(feature = "mobile")]
const MAX_BLOCK_SIZE: usize = 16;

#[cfg(feature = "mobile")]
pub struct ChunkedDecryptor {
    enc_type: EncryptionType,

    // Block size of the cipher used, the data passed to the decryptor must this size
    block_size: usize,

    // Buffer for storing the last full block and the last partial block (if any) from the previous chunk
    buf: [u8; 2 * MAX_BLOCK_SIZE],
    buf_len: usize,
}

#[cfg(feature = "mobile")]
impl ChunkedDecryptor {
    /// Creates a new decryptor for a chunked cipher string
    /// Important: The first chunk must contain the encryption type, MAC and IV (which are contained in the first bytes
    /// of the encrypted blob) plus at least one block, so make sure that the initial chunk is at least 65 bytes long
    pub fn new(key: SymmetricCryptoKey, initial_chunk: &[u8]) -> Result<(Self, Vec<u8>)> {
        let remaining_chunk;
        let block_size;

        // The first byte of the message indicates the encryption type
        let Some(enc_type_num) = initial_chunk.first() else {
            return Err(EncStringParseError::InvalidType {
                enc_type: "Missing".to_string(),
                parts: 1,
            }
            .into());
        };
        let enc_type = match enc_type_num {
            0 => unimplemented!(),
            1 | 2 => {
                if initial_chunk.len() < 49 {
                    return Err(EncStringParseError::InvalidLength {
                        expected: 49,
                        got: initial_chunk.len(),
                    }
                    .into());
                }

                // Extract IV and MAC from the initial chunk, and separate the rest of the chunk
                let iv: [u8; 16] = initial_chunk[1..17].try_into().unwrap();
                let mac: [u8; 32] = initial_chunk[17..49].try_into().unwrap();
                remaining_chunk = &initial_chunk[49..];
                let Some(mac_key) = &key.mac_key else {
                    return Err(CryptoError::InvalidMac.into());
                };

                // Initialize HMAC and decryptor
                let mut hmac = PbkdfSha256Hmac::new_from_slice(mac_key)
                    .expect("HMAC can take key of any size");
                hmac.update(&iv);

                match enc_type_num {
                    1 => {
                        block_size = <Decryptor<Aes128> as BlockSizeUser>::BlockSize::USIZE;
                        EncryptionType::AesCbc128_HmacSha256_B64 { iv, mac, hmac }
                    }
                    2 => {
                        let decryptor = Decryptor::new(&key.key, GenericArray::from_slice(&iv));
                        block_size = <Decryptor<Aes256> as BlockSizeUser>::BlockSize::USIZE;
                        EncryptionType::AesCbc256_HmacSha256_B64 {
                            iv,
                            mac,
                            hmac,
                            decryptor,
                        }
                    }
                    _ => unreachable!(),
                }
            }
            _ => {
                return Err(EncStringParseError::InvalidType {
                    enc_type: enc_type_num.to_string(),
                    parts: 1,
                }
                .into())
            }
        };

        let mut decryptor = Self {
            enc_type,
            block_size,
            buf: [0u8; 2 * MAX_BLOCK_SIZE],
            buf_len: 0,
        };
        // Process the rest of the initial chunk
        let decrypted_initial_chunk = decryptor.decrypt_chunk(remaining_chunk)?;
        Ok((decryptor, decrypted_initial_chunk))
    }

    /// Decrypts a chunk of data, the chunk size must greater than the cipher's block size (16 bytes)
    pub fn decrypt_chunk(&mut self, chunk: &[u8]) -> Result<Vec<u8>> {
        match &mut self.enc_type {
            EncryptionType::AesCbc256_B64 { .. } => unimplemented!(),
            EncryptionType::AesCbc128_HmacSha256_B64 { .. } => unimplemented!(),
            EncryptionType::AesCbc256_HmacSha256_B64 {
                hmac, decryptor, ..
            } => {
                // Only work with chunks larger than the block size
                if chunk.len() < self.block_size {
                    return Err(Error::Internal("Chunk size too small"));
                }

                // Update HMAC, this doesn't care about block sizes, so just pass the whole chunk
                hmac.update(chunk);

                // Preallocate the result vector based on the chunk size plus an extra block to account for partial blocks
                let mut result = Vec::with_capacity(chunk.len() + self.block_size);

                let mut process_block = |block: &[u8]| {
                    let block: [u8; 16] = block.try_into().unwrap();
                    let mut block = block.into();
                    decryptor.decrypt_block_mut(&mut block);
                    result.extend_from_slice(&block);
                };

                let skip_initial_bytes = if self.buf_len > 0 {
                    // Process previous full block
                    let previous_full_block = &self.buf[..self.block_size];
                    process_block(previous_full_block);

                    // Process partial block if there is one
                    if self.buf_len > self.block_size {
                        let bytes_to_complete_partial = self.buf.len() - self.buf_len;

                        // Fill up the partial block with the first bytes of the chunk
                        self.buf[self.buf_len..]
                            .copy_from_slice(&chunk[0..bytes_to_complete_partial]);

                        // Process the now filled partial block
                        let previous_partial_block = &self.buf[self.block_size..];
                        process_block(previous_partial_block);

                        bytes_to_complete_partial
                    } else {
                        0
                    }
                } else {
                    0
                };

                // Check how many bytes we need to process the previous partial data and the current chunk
                let full_chunk_size = chunk.len() - skip_initial_bytes;
                let remainder_bytes = (full_chunk_size % self.block_size) + self.block_size;

                let chunk_to_process = &chunk[skip_initial_bytes..(chunk.len() - remainder_bytes)];

                for block in chunk_to_process.chunks_exact(self.block_size) {
                    process_block(block)
                }

                self.buf[0..remainder_bytes]
                    .copy_from_slice(&chunk[chunk.len() - remainder_bytes..]);
                self.buf_len = remainder_bytes;

                Ok(result)
            }
        }
    }

    pub fn finalize(mut self) -> Result<Vec<u8>> {
        // Process last block separately and handle it's padding
        let last_buf = match &mut self.enc_type {
            EncryptionType::AesCbc256_B64 { .. } => unimplemented!(),
            EncryptionType::AesCbc128_HmacSha256_B64 { .. } => unimplemented!(),
            EncryptionType::AesCbc256_HmacSha256_B64 { decryptor, .. } => {
                #[allow(clippy::comparison_chain)]
                if self.buf_len > self.block_size {
                    return Err(Error::Internal("Partial block at the end of the data"));
                } else if self.buf_len == self.block_size {
                    let block: [u8; 16] = self.buf[..self.block_size].try_into().unwrap();
                    let mut block = block.into();
                    decryptor.decrypt_block_mut(&mut block);

                    Pkcs7::unpad(&block).unwrap().to_vec()
                } else {
                    Vec::new()
                }
            }
        };

        // Validate MAC
        match self.enc_type {
            EncryptionType::AesCbc256_B64 { iv: _ } => unimplemented!(),
            EncryptionType::AesCbc128_HmacSha256_B64 { mac, hmac, .. }
            | EncryptionType::AesCbc256_HmacSha256_B64 { mac, hmac, .. } => {
                if hmac.finalize() != CtOutput::new(mac.into()) {
                    return Err(CryptoError::InvalidMac.into());
                }
            }
        }

        Ok(last_buf)
    }
}

#[cfg(feature = "mobile")]
pub fn decrypt_file(
    key: SymmetricCryptoKey,
    encrypted_file_path: &std::path::Path,
    decrypted_file_path: &std::path::Path,
) -> Result<()> {
    // TODO: Move to use an async file implementation
    use std::{
        fs::File,
        io::{Read, Write},
    };

    let mut encrypted_file = File::open(encrypted_file_path)?;
    let mut decrypted_file = File::create(decrypted_file_path)?;

    let mut buffer = [0; 4096];
    let bytes_read = encrypted_file.read(&mut buffer)?;
    if bytes_read == 0 {
        return Err(Error::Internal("Empty file"));
    }
    let (mut decryptor, initial_chunk) = ChunkedDecryptor::new(key, &buffer[..bytes_read])?;
    decrypted_file.write_all(&initial_chunk)?;

    loop {
        let bytes_read = encrypted_file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        let chunk = decryptor.decrypt_chunk(&buffer[..bytes_read])?;
        decrypted_file.write_all(&chunk)?;
    }

    let chunk = decryptor.finalize()?;
    decrypted_file.write_all(&chunk)?;

    Ok(())
}
