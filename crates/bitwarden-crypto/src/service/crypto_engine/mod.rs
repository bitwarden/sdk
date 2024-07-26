use crate::{
    service::{AsymmetricKeyRef, SymmetricKeyRef},
    AsymmetricEncString, EncString, SymmetricCryptoKey,
};

mod rust_impl;

pub(crate) use rust_impl::RustCryptoEngine;

// This trait represents a service that can store cryptographic keys and perform operations with
// them, ideally within a Secure Enclave or HSM. Users of this trait will not handle the keys
// directly but will use references to them.
//
// For the cases where a secure element capable of doing cryptographic operations is not available,
// but there is a secure way to store keys, `KeyStore` can be implemented and then used in
// conjunction with `RustCryptoEngine`.
pub(crate) trait CryptoEngine<SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef> {
    /// Create a new context for this service. This allows the user to perform cryptographic
    // operations with keys that are only relevant to the current context.
    ///
    /// NOTE: This is an advanced API, and should be used with care. Particularly, it's important
    /// to ensure the context is dropped when it's no longer needed, to avoid holding a reference to
    /// the RwLock for too long. It's also important to ensure that the context is cleared of
    /// keys after every use if it's being reused, to avoid it growing indefinitely.
    fn context(&'_ self) -> Box<dyn CryptoEngineContext<'_, SymmKeyRef, AsymmKeyRef> + '_>;

    #[deprecated(note = "We should be generating/decrypting the keys into the service directly")]
    fn insert_symmetric_key(&self, key_ref: SymmKeyRef, key: SymmetricCryptoKey);

    fn clear(&self);
}

// This trait represents a context for a `CryptoEngine`. It allows the user to perform cryptographic
// operations with keys that are only relevant to the current context.
#[allow(dead_code)]
pub(crate) trait CryptoEngineContext<'a, SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef>
{
    fn clear(&mut self);

    // Symmetric key operations

    fn decrypt_data_with_symmetric_key(
        &self,
        key: SymmKeyRef,
        data: &EncString,
    ) -> Result<Vec<u8>, crate::CryptoError>;

    fn encrypt_data_with_symmetric_key(
        &self,
        key: SymmKeyRef,
        data: &[u8],
    ) -> Result<EncString, crate::CryptoError>;

    fn decrypt_and_store_symmetric_key(
        &mut self,
        encryption_key: SymmKeyRef,
        new_key_ref: SymmKeyRef,
        encrypted_key: &EncString,
    ) -> Result<SymmKeyRef, crate::CryptoError>;

    fn encrypt_symmetric_key(
        &self,
        encryption_key: SymmKeyRef,
        key_to_encrypt: SymmKeyRef,
    ) -> Result<EncString, crate::CryptoError>;

    // Asymmetric key operations

    fn decrypt_data_with_asymmetric_key(
        &self,
        key: AsymmKeyRef,
        data: &AsymmetricEncString,
    ) -> Result<Vec<u8>, crate::CryptoError>;

    fn encrypt_data_with_asymmetric_key(
        &self,
        key: AsymmKeyRef,
        data: &[u8],
    ) -> Result<AsymmetricEncString, crate::CryptoError>;

    fn decrypt_and_store_asymmetric_key(
        &mut self,
        encryption_key: AsymmKeyRef,
        new_key_ref: AsymmKeyRef,
        encrypted_key: &AsymmetricEncString,
    ) -> Result<AsymmKeyRef, crate::CryptoError>;

    fn encrypt_asymmetric_key(
        &self,
        encryption_key: AsymmKeyRef,
        key_to_encrypt: AsymmKeyRef,
    ) -> Result<AsymmetricEncString, crate::CryptoError>;
}

fn _ensure_that_traits_are_object_safe<
    SymmKeyRef: SymmetricKeyRef,
    AsymmKeyRef: AsymmetricKeyRef,
>(
    _: Box<dyn CryptoEngine<SymmKeyRef, AsymmKeyRef>>,
    _: Box<dyn CryptoEngineContext<SymmKeyRef, AsymmKeyRef>>,
) {
}
