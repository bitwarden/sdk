use aes::cipher::{generic_array::GenericArray, typenum::U64};

use crate::crypto::SymmetricCryptoKey;

pub(crate) fn stretch_key(secret: [u8; 16], name: &str, info: Option<&str>) -> SymmetricCryptoKey {
    use hmac::{Hmac, Mac};
    // Because all inputs are fixed size, we can unwrap all errors here without issue

    // TODO: Are these the final `key` and `info` parameters or should we change them? I followed the pattern used for sends
    let res = Hmac::<sha2::Sha256>::new_from_slice(format!("bitwarden-{}", name).as_bytes())
        .unwrap()
        .chain_update(secret)
        .finalize()
        .into_bytes();

    let hkdf = hkdf::Hkdf::<sha2::Sha256>::from_prk(&res).unwrap();

    let mut key = GenericArray::<u8, U64>::default();

    // TODO: Should we have a default value for info?
    //  Should it be required?
    let i = info.map(|i| i.as_bytes()).unwrap_or(&[]);
    hkdf.expand(i, &mut key).unwrap();

    SymmetricCryptoKey::try_from(key.as_slice()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::stretch_key;

    #[test]
    fn test_key_stretch() {
        let key = stretch_key(*b"&/$%F1a895g67HlX", "test_key", None);
        assert_eq!(key.to_base64(), "4PV6+PcmF2w7YHRatvyMcVQtI7zvCyssv/wFWmzjiH6Iv9altjmDkuBD1aagLVaLezbthbSe+ktR+U6qswxNnQ==");

        let key = stretch_key(*b"67t9b5g67$%Dh89n", "test_key", Some("test"));
        assert_eq!(key.to_base64(), "F9jVQmrACGx9VUPjuzfMYDjr726JtL300Y3Yg+VYUnVQtQ1s8oImJ5xtp1KALC9h2nav04++1LDW4iFD+infng==");
    }
}
