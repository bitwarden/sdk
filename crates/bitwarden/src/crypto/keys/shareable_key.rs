use aes::cipher::{generic_array::GenericArray, typenum::U64};
use hmac::{Hmac, Mac};

use crate::crypto::{hkdf_expand, SymmetricCryptoKey};

use super::KeyPurpose;

/// Marker trait to annotate that the key is intended shareable beyond the current account
pub trait ShareableKey : KeyPurpose {}

impl<TKeyPurpose : ShareableKey> SymmetricCryptoKey<TKeyPurpose> {
    pub fn generate(name: &str) -> Self {
        use rand::Rng;
        let secret: [u8; 16] = rand::thread_rng().gen();
        derive_shareable_key::<TKeyPurpose>(secret, name, None)
    }
}

/// Derive a shareable key using hkdf from secret and name.
///
/// A specialized variant of this function was called `CryptoService.makeSendKey` in the Bitwarden
/// `clients` repository.
pub(crate) fn derive_shareable_key<TKeyPurpose: ShareableKey>(
    secret: [u8; 16],
    name: &str,
    info: Option<&str>,
) -> SymmetricCryptoKey<TKeyPurpose> {
    // Because all inputs are fixed size, we can unwrap all errors here without issue

    // TODO: Are these the final `key` and `info` parameters or should we change them? I followed the pattern used for sends
    let res = Hmac::<sha2::Sha256>::new_from_slice(format!("bitwarden-{}", name).as_bytes())
        .unwrap()
        .chain_update(secret)
        .finalize()
        .into_bytes();

    let key: GenericArray<u8, U64> = hkdf_expand(&res, info).unwrap();

    SymmetricCryptoKey::try_from(key.as_slice()).unwrap()
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
