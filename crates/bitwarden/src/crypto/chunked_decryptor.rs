use std::io::Write;

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

        // Buffer for storing the last block from the chunk,
        // so we can unpad it when hitting the end of the stream
        last_block: [u8; 16],
        last_block_filled: bool,
    },
}

struct ChunkedDecryptorConfigured {
    enc_type: EncryptionType,

    // Block size of the cipher used, the data passed to the decryptor must be exactly this size
    block_size: usize,
}

// The second variant is big enough to trigger the lint, but it's not
// a problem as the first variant is only used briefly during setup
#[allow(clippy::large_enum_variant)]
enum ChunkedDecryptorState<'a> {
    Initial(&'a SymmetricCryptoKey),
    Configured(ChunkedDecryptorConfigured),
}

pub struct ChunkedDecryptor<'a, Output: Write> {
    state: ChunkedDecryptorState<'a>,
    output: Output,
    buffer: Vec<u8>,
}

const INTERNAL_BUFFER_SIZE: usize = 4096;
const MIN_UNBUFFERED_SIZE: usize = 64;

impl<'a, Output: Write> ChunkedDecryptor<'a, Output> {
    pub fn new(key: &'a SymmetricCryptoKey, output: Output) -> Self {
        Self {
            state: ChunkedDecryptorState::Initial(key),
            output,
            buffer: Vec::with_capacity(INTERNAL_BUFFER_SIZE),
        }
    }

    fn read_initial(
        key: &SymmetricCryptoKey,
        buf: &[u8],
    ) -> Result<(Option<ChunkedDecryptorConfigured>, usize)> {
        // The first byte of the message indicates the encryption type
        let Some(&enc_type_num) = buf.first() else {
            return Ok((None, 0));
        };
        let (enc_type, block_size, bytes_read) = match enc_type_num {
            0 => unimplemented!(),
            1 | 2 => {
                const HEADER_SIZE: usize = 49;

                if buf.len() < HEADER_SIZE {
                    return Ok((None, 0));
                }

                // Extract IV and MAC from the initial chunk, and separate the rest of the chunk
                let iv: [u8; 16] = buf[1..17].try_into().unwrap();
                let mac: [u8; 32] = buf[17..HEADER_SIZE].try_into().unwrap();
                let Some(mac_key) = &key.mac_key else {
                    return Err(CryptoError::InvalidMac.into());
                };

                // Initialize HMAC and decryptor
                let mut hmac = PbkdfSha256Hmac::new_from_slice(mac_key)
                    .expect("HMAC can take key of any size");
                hmac.update(&iv);

                match enc_type_num {
                    1 => (
                        EncryptionType::AesCbc128_HmacSha256_B64 { iv, mac, hmac },
                        <Decryptor<Aes128> as BlockSizeUser>::BlockSize::USIZE,
                        HEADER_SIZE,
                    ),
                    2 => (
                        EncryptionType::AesCbc256_HmacSha256_B64 {
                            iv,
                            mac,
                            hmac,
                            decryptor: Decryptor::new(&key.key, (&iv).into()),
                            last_block: [0u8; 16],
                            last_block_filled: false,
                        },
                        <Decryptor<Aes256> as BlockSizeUser>::BlockSize::USIZE,
                        HEADER_SIZE,
                    ),
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

        Ok((
            Some(ChunkedDecryptorConfigured {
                enc_type,
                block_size,
            }),
            bytes_read,
        ))
    }

    fn read_blocks(
        state: &mut ChunkedDecryptorConfigured,
        output: &mut Output,
        buf: &mut [u8],
    ) -> Result<usize> {
        match &mut state.enc_type {
            EncryptionType::AesCbc256_B64 { .. } => unimplemented!(),
            EncryptionType::AesCbc128_HmacSha256_B64 { .. } => unimplemented!(),
            EncryptionType::AesCbc256_HmacSha256_B64 {
                hmac,
                decryptor,
                last_block,
                last_block_filled,
                ..
            } => {
                // If we got less than a block, we need to wait for more data
                if buf.len() < state.block_size {
                    return Ok(0);
                }

                // Make sure we only process full blocks
                // We decrypt one block less than we have so we can remove the padding in finalize
                let bytes_to_process = buf.len() - (buf.len() % state.block_size);
                let bytes_to_decrypt = bytes_to_process - state.block_size;

                // Update HMAC value for all the processed bytes
                hmac.update(&buf[..bytes_to_process]);

                // Process the last block from the previous call, as we are not at the end of the stream
                if *last_block_filled {
                    decryptor.decrypt_block_mut(last_block.into());
                    output.write_all(last_block)?;
                } else {
                    *last_block_filled = true;
                }

                // Store the last block for later, in case this is the end of the stream
                last_block
                    .copy_from_slice(&buf[bytes_to_decrypt..bytes_to_decrypt + state.block_size]);

                // Split the buffer into blocks and decrypt them in place
                for block in buf[..bytes_to_decrypt].chunks_exact_mut(state.block_size) {
                    decryptor.decrypt_block_mut(block.into());
                }
                // Write all the decrypted blocks at once
                output.write_all(&buf[..bytes_to_decrypt])?;

                Ok(bytes_to_process)
            }
        }
    }

    pub fn finalize(mut self) -> Result<()> {
        // Flush internal buffer before processing last block
        self.flush()?;

        let ChunkedDecryptorState::Configured(mut state) = self.state else {
            return Err(Error::Internal("ChunkedDecryptor has not been written to"));
        };

        // Process last block separately and handle it's padding
        match &mut state.enc_type {
            EncryptionType::AesCbc256_B64 { .. } => unimplemented!(),
            EncryptionType::AesCbc128_HmacSha256_B64 { .. } => unimplemented!(),
            EncryptionType::AesCbc256_HmacSha256_B64 {
                decryptor,
                last_block,
                last_block_filled,
                ..
            } => {
                if *last_block_filled {
                    let block: &mut GenericArray<_, _> = last_block.into();
                    decryptor.decrypt_block_mut(block);

                    let Ok(block_unpadded) = Pkcs7::unpad(block) else {
                        return Err(Error::Internal("Invalid padding"));
                    };
                    self.output.write_all(block_unpadded)?;
                    self.output.flush()?;
                } else {
                    return Err(Error::Internal("Invalid block at the end of the data"));
                }
            }
        };

        // Validate MAC
        match state.enc_type {
            EncryptionType::AesCbc256_B64 { iv: _ } => { /* No HMAC, nothing to do */ }
            EncryptionType::AesCbc128_HmacSha256_B64 { mac, hmac, .. }
            | EncryptionType::AesCbc256_HmacSha256_B64 { mac, hmac, .. } => {
                if hmac.finalize() != CtOutput::new(mac.into()) {
                    return Err(CryptoError::InvalidMac.into());
                }
            }
        }

        Ok(())
    }
}

impl<'a, Output: Write> Write for ChunkedDecryptor<'a, Output> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // Insert received data into the buffer
        self.buffer.extend_from_slice(buf);
        if self.buffer.is_empty() {
            return Ok(0);
        }

        // If we have a small amount of bytes and enough space, copy them to the internal buffer and return
        let incoming_buf_len = buf.len();
        if incoming_buf_len > 0 && self.buffer.len() < MIN_UNBUFFERED_SIZE {
            return Ok(incoming_buf_len);
        }

        let written = match &mut self.state {
            ChunkedDecryptorState::Initial(key) => {
                let (state, bytes_read) = Self::read_initial(key, &self.buffer)?;
                if let Some(state) = state {
                    self.state = ChunkedDecryptorState::Configured(state);
                }
                bytes_read
            }
            ChunkedDecryptorState::Configured(state) => {
                Self::read_blocks(state, &mut self.output, &mut self.buffer)?
            }
        };

        // Remove the processed bytes from the internal buffer
        self.buffer.drain(..written);

        Ok(incoming_buf_len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        // Make sure the internal buffer has been processed entirely
        // Note: We don't need to flush the output here, as that is done in finalize
        self.write(&[]).map(|_| ())
    }
}

pub fn decrypt_file(
    key: SymmetricCryptoKey,
    encrypted_file_path: &std::path::Path,
    decrypted_file_path: &std::path::Path,
) -> Result<()> {
    use std::fs::File;

    let mut encrypted_file = File::open(encrypted_file_path)?;
    let mut decrypted_file = File::create(decrypted_file_path)?;

    let mut decryptor = ChunkedDecryptor::new(&key, &mut decrypted_file);
    std::io::copy(&mut encrypted_file, &mut decryptor)?;
    decryptor.finalize()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use rand::RngCore;

    use crate::crypto::{encrypt_aes256, SymmetricCryptoKey};

    use super::ChunkedDecryptor;

    #[test]
    fn test_chunk_decryption() {
        // Test different combinations of cipher and chunk sizes
        for size in [64, 500, 100_000, 9_000_000] {
            let mut initial_buf = Vec::with_capacity(size);
            initial_buf.resize(size, 0);
            rand::thread_rng().fill_bytes(&mut initial_buf[..size]);
            let key: SymmetricCryptoKey = SymmetricCryptoKey::generate("test");
            let encrypted_buf = encrypt_aes256(&initial_buf, key.key)
                .unwrap()
                .to_buffer()
                .unwrap();

            let mut decrypted_buf = Vec::with_capacity(size);

            for chunk_size in [1, 15, 16, 64, 1024] {
                decrypted_buf.clear();
                let mut cd = ChunkedDecryptor::new(&key, &mut decrypted_buf);

                for chunk in encrypted_buf.chunks(chunk_size) {
                    cd.write_all(chunk).unwrap();
                }
                cd.finalize().unwrap();

                assert_eq!(initial_buf, decrypted_buf);
            }
        }
    }
}
