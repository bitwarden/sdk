use std::pin::Pin;

use hmac::{KeyInit, Mac};
use zeroize::Zeroize;

use crate::{
    keys::SymmetricCryptoKey,
    util::{hkdf_expand, PbkdfSha256Hmac},
    Sensitive,
};

/// Derive a shareable key using hkdf from secret and name.
///
/// A specialized variant of this function was called `CryptoService.makeSendKey` in the Bitwarden
/// `clients` repository.
pub fn derive_shareable_key(
    secret: Sensitive<[u8; 16]>,
    name: &str,
    info: Option<&str>,
) -> SymmetricCryptoKey {
    // Because all inputs are fixed size, we can unwrap all errors here without issue
    let mut res = PbkdfSha256Hmac::new_from_slice(format!("bitwarden-{}", name).as_bytes())
        .expect("hmac new_from_slice should not fail")
        .chain_update(secret.expose())
        .finalize()
        .into_bytes();

    let mut key: Pin<Box<[u8; 64]>> = hkdf_expand(&res, info).expect("Input is a valid size");

    // Zeroize the temporary buffer
    res.zeroize();

    SymmetricCryptoKey::try_from(key.as_mut_slice()).expect("Key is a valid size")
}

#[cfg(test)]
mod tests {
    use super::derive_shareable_key;
    use crate::Sensitive;

    #[test]
    fn test_derive_shareable_key() {
        let key = derive_shareable_key(
            Sensitive::new(Box::new(*b"&/$%F1a895g67HlX")),
            "test_key",
            None,
        );
        assert_eq!(key.to_base64().expose(), "4PV6+PcmF2w7YHRatvyMcVQtI7zvCyssv/wFWmzjiH6Iv9altjmDkuBD1aagLVaLezbthbSe+ktR+U6qswxNnQ==");

        let key = derive_shareable_key(
            Sensitive::new(Box::new(*b"67t9b5g67$%Dh89n")),
            "test_key",
            Some("test"),
        );
        assert_eq!(key.to_base64().expose(), "F9jVQmrACGx9VUPjuzfMYDjr726JtL300Y3Yg+VYUnVQtQ1s8oImJ5xtp1KALC9h2nav04++1LDW4iFD+infng==");
    }
}
