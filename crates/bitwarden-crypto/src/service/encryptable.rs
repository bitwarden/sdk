use super::{
    key_ref::{AsymmetricKeyRef, KeyRef, SymmetricKeyRef},
    CryptoServiceContext,
};
use crate::{AsymmetricEncString, CryptoError, EncString};

///////////////////////

// Just like LocateKey but this time we're not locating anything, just returning a ref

pub trait UsesKey<Key: KeyRef> {
    fn uses_key(&self) -> Key;
}

// This extension trait allows any type to be wrapped with `KeyProvided`
// to make it easy to encrypt/decrypt it with the desired key
pub trait KeyProvidedExt<Key: KeyRef>: Sized {
    fn using_key(self, key: Key) -> KeyProvided<Key, Self> {
        KeyProvided { key, value: self }
    }
}
impl<Key: KeyRef, T> KeyProvidedExt<Key> for T {}
pub struct KeyProvided<Key: KeyRef, T: ?Sized> {
    key: Key,
    value: T,
}
impl<Key: KeyRef, T> UsesKey<Key> for KeyProvided<Key, T> {
    fn uses_key(&self) -> Key {
        self.key
    }
}
impl<
        SymmKeyRef: SymmetricKeyRef,
        AsymmKeyRef: AsymmetricKeyRef,
        Key: KeyRef,
        T: Encryptable<SymmKeyRef, AsymmKeyRef, Key, Output>,
        Output,
    > Encryptable<SymmKeyRef, AsymmKeyRef, Key, Output> for KeyProvided<Key, T>
{
    fn encrypt(
        &self,
        ctx: &mut CryptoServiceContext<SymmKeyRef, AsymmKeyRef>,
        _key: Key,
    ) -> Result<Output, crate::CryptoError> {
        self.value.encrypt(ctx, self.key)
    }
}
impl<
        SymmKeyRef: SymmetricKeyRef,
        AsymmKeyRef: AsymmetricKeyRef,
        Key: KeyRef,
        T: Decryptable<SymmKeyRef, AsymmKeyRef, Key, Output>,
        Output,
    > Decryptable<SymmKeyRef, AsymmKeyRef, Key, Output> for KeyProvided<Key, T>
{
    fn decrypt(
        &self,
        ctx: &mut CryptoServiceContext<SymmKeyRef, AsymmKeyRef>,
        _key: Key,
    ) -> Result<Output, crate::CryptoError> {
        self.value.decrypt(ctx, self.key)
    }
}

/////////////////////

pub trait Encryptable<
    SymmKeyRef: SymmetricKeyRef,
    AsymmKeyRef: AsymmetricKeyRef,
    Key: KeyRef,
    Output,
>
{
    fn encrypt(
        &self,
        ctx: &mut CryptoServiceContext<SymmKeyRef, AsymmKeyRef>,
        key: Key,
    ) -> Result<Output, crate::CryptoError>;
}

pub trait Decryptable<
    SymmKeyRef: SymmetricKeyRef,
    AsymmKeyRef: AsymmetricKeyRef,
    Key: KeyRef,
    Output,
>
{
    fn decrypt(
        &self,
        ctx: &mut CryptoServiceContext<SymmKeyRef, AsymmKeyRef>,
        key: Key,
    ) -> Result<Output, crate::CryptoError>;
}

impl<SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef>
    Decryptable<SymmKeyRef, AsymmKeyRef, SymmKeyRef, Vec<u8>> for EncString
{
    fn decrypt(
        &self,
        ctx: &mut CryptoServiceContext<SymmKeyRef, AsymmKeyRef>,
        key: SymmKeyRef,
    ) -> Result<Vec<u8>, crate::CryptoError> {
        ctx.engine.decrypt_data_with_symmetric_key(key, self)
    }
}

impl<SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef>
    Decryptable<SymmKeyRef, AsymmKeyRef, AsymmKeyRef, Vec<u8>> for AsymmetricEncString
{
    fn decrypt(
        &self,
        ctx: &mut CryptoServiceContext<SymmKeyRef, AsymmKeyRef>,
        key: AsymmKeyRef,
    ) -> Result<Vec<u8>, crate::CryptoError> {
        ctx.engine.decrypt_data_with_asymmetric_key(key, self)
    }
}

impl<SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef>
    Encryptable<SymmKeyRef, AsymmKeyRef, SymmKeyRef, EncString> for &[u8]
{
    fn encrypt(
        &self,
        ctx: &mut CryptoServiceContext<SymmKeyRef, AsymmKeyRef>,
        key: SymmKeyRef,
    ) -> Result<EncString, crate::CryptoError> {
        ctx.engine.encrypt_data_with_symmetric_key(key, self)
    }
}

impl<SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef>
    Encryptable<SymmKeyRef, AsymmKeyRef, AsymmKeyRef, AsymmetricEncString> for &[u8]
{
    fn encrypt(
        &self,
        ctx: &mut CryptoServiceContext<SymmKeyRef, AsymmKeyRef>,
        key: AsymmKeyRef,
    ) -> Result<AsymmetricEncString, crate::CryptoError> {
        ctx.engine.encrypt_data_with_asymmetric_key(key, self)
    }
}

impl<SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef>
    Decryptable<SymmKeyRef, AsymmKeyRef, SymmKeyRef, String> for EncString
{
    fn decrypt(
        &self,
        ctx: &mut CryptoServiceContext<SymmKeyRef, AsymmKeyRef>,
        key: SymmKeyRef,
    ) -> Result<String, crate::CryptoError> {
        let bytes: Vec<u8> = self.decrypt(ctx, key)?;
        String::from_utf8(bytes).map_err(|_| CryptoError::InvalidUtf8String)
    }
}

impl<SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef>
    Decryptable<SymmKeyRef, AsymmKeyRef, AsymmKeyRef, String> for AsymmetricEncString
{
    fn decrypt(
        &self,
        ctx: &mut CryptoServiceContext<SymmKeyRef, AsymmKeyRef>,
        key: AsymmKeyRef,
    ) -> Result<String, crate::CryptoError> {
        let bytes: Vec<u8> = self.decrypt(ctx, key)?;
        String::from_utf8(bytes).map_err(|_| CryptoError::InvalidUtf8String)
    }
}

impl<SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef>
    Encryptable<SymmKeyRef, AsymmKeyRef, SymmKeyRef, EncString> for &str
{
    fn encrypt(
        &self,
        ctx: &mut CryptoServiceContext<SymmKeyRef, AsymmKeyRef>,
        key: SymmKeyRef,
    ) -> Result<EncString, crate::CryptoError> {
        self.as_bytes().encrypt(ctx, key)
    }
}

impl<SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef>
    Encryptable<SymmKeyRef, AsymmKeyRef, AsymmKeyRef, AsymmetricEncString> for &str
{
    fn encrypt(
        &self,
        ctx: &mut CryptoServiceContext<SymmKeyRef, AsymmKeyRef>,
        key: AsymmKeyRef,
    ) -> Result<AsymmetricEncString, crate::CryptoError> {
        self.as_bytes().encrypt(ctx, key)
    }
}
