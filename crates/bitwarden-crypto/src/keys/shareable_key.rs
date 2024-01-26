use std::pin::Pin;

use aes::cipher::typenum::U64;
use generic_array::GenericArray;
use hmac::{Hmac, Mac};

use crate::{keys::SymmetricCryptoKey, util::hkdf_expand};

/// Derive a shareable key using hkdf from secret and name.
///
/// A specialized variant of this function was called `CryptoService.makeSendKey` in the Bitwarden
/// `clients` repository.
pub fn derive_shareable_key(
    secret: [u8; 16],
    name: &str,
    info: Option<&str>,
) -> SymmetricCryptoKey {
    // Because all inputs are fixed size, we can unwrap all errors here without issue

    // TODO: Are these the final `key` and `info` parameters or should we change them? I followed
    // the pattern used for sends
    let res = Hmac::<sha2::Sha256>::new_from_slice(format!("bitwarden-{}", name).as_bytes())
        .unwrap()
        .chain_update(secret)
        .finalize()
        .into_bytes();

    let mut key: Pin<Box<GenericArray<u8, U64>>> = hkdf_expand(&res, info).unwrap();

    SymmetricCryptoKey::try_from(key.as_mut_slice()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::derive_shareable_key;

    #[test]
    fn test_derive_shareable_key() {
        let key = derive_shareable_key(*b"&/$%F1a895g67HlX", "test_key", None);
        assert_eq!(key.to_base64(), "4PV6+PcmF2w7YHRatvyMcVQtI7zvCyssv/wFWmzjiH6Iv9altjmDkuBD1aagLVaLezbthbSe+ktR+U6qswxNnQ==");

        let key = derive_shareable_key(*b"67t9b5g67$%Dh89n", "test_key", Some("test"));
        assert_eq!(key.to_base64(), "F9jVQmrACGx9VUPjuzfMYDjr726JtL300Y3Yg+VYUnVQtQ1s8oImJ5xtp1KALC9h2nav04++1LDW4iFD+infng==");
    }
}
