use super::key_ref::{AsymmetricKeyRef, KeyRef, SymmetricKeyRef};
use crate::{service::CryptoServiceContext, AsymmetricEncString, CryptoError, EncString};

// Just like LocateKey but this time we're not locating anything, just returning a ref

pub trait UsesKey<Key: KeyRef> {
    fn uses_key(&self) -> Key;
}

// This extension trait allows any type to be wrapped with `UsingKey`
// to make it easy to encrypt/decrypt it with the desired key,
// this way we don't need to have a separate `encrypt_with_key` function
pub trait UsingKeyExt<Key: KeyRef>: Sized {
    fn using_key(self, key: Key) -> UsingKey<Key, Self> {
        UsingKey { key, value: self }
    }
}
impl<Key: KeyRef, T> UsingKeyExt<Key> for T {}
pub struct UsingKey<Key: KeyRef, T: ?Sized> {
    key: Key,
    value: T,
}
impl<Key: KeyRef, T> UsesKey<Key> for UsingKey<Key, T> {
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
    > Encryptable<SymmKeyRef, AsymmKeyRef, Key, Output> for UsingKey<Key, T>
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
    > Decryptable<SymmKeyRef, AsymmKeyRef, Key, Output> for UsingKey<Key, T>
{
    fn decrypt(
        &self,
        ctx: &mut CryptoServiceContext<SymmKeyRef, AsymmKeyRef>,
        _key: Key,
    ) -> Result<Output, crate::CryptoError> {
        self.value.decrypt(ctx, self.key)
    }
}

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

// Basic Encryptable/Decryptable implementations to and from bytes

impl<SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef>
    Decryptable<SymmKeyRef, AsymmKeyRef, SymmKeyRef, Vec<u8>> for EncString
{
    fn decrypt(
        &self,
        ctx: &mut CryptoServiceContext<SymmKeyRef, AsymmKeyRef>,
        key: SymmKeyRef,
    ) -> Result<Vec<u8>, crate::CryptoError> {
        ctx.decrypt_data_with_symmetric_key(key, self)
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
        ctx.decrypt_data_with_asymmetric_key(key, self)
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
        ctx.encrypt_data_with_symmetric_key(key, self)
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
        ctx.encrypt_data_with_asymmetric_key(key, self)
    }
}

// Encryptable/Decryptable implementations to and from strings

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

impl<SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef>
    Encryptable<SymmKeyRef, AsymmKeyRef, SymmKeyRef, EncString> for String
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
    Encryptable<SymmKeyRef, AsymmKeyRef, AsymmKeyRef, AsymmetricEncString> for String
{
    fn encrypt(
        &self,
        ctx: &mut CryptoServiceContext<SymmKeyRef, AsymmKeyRef>,
        key: AsymmKeyRef,
    ) -> Result<AsymmetricEncString, crate::CryptoError> {
        self.as_bytes().encrypt(ctx, key)
    }
}

// Generic implementations for Optional values

impl<
        SymmKeyRef: SymmetricKeyRef,
        AsymmKeyRef: AsymmetricKeyRef,
        Key: KeyRef,
        T: Encryptable<SymmKeyRef, AsymmKeyRef, Key, Output>,
        Output,
    > Encryptable<SymmKeyRef, AsymmKeyRef, Key, Option<Output>> for Option<T>
{
    fn encrypt(
        &self,
        ctx: &mut CryptoServiceContext<SymmKeyRef, AsymmKeyRef>,
        key: Key,
    ) -> Result<Option<Output>, crate::CryptoError> {
        self.as_ref()
            .map(|value| value.encrypt(ctx, key))
            .transpose()
    }
}

impl<
        SymmKeyRef: SymmetricKeyRef,
        AsymmKeyRef: AsymmetricKeyRef,
        Key: KeyRef,
        T: Decryptable<SymmKeyRef, AsymmKeyRef, Key, Output>,
        Output,
    > Decryptable<SymmKeyRef, AsymmKeyRef, Key, Option<Output>> for Option<T>
{
    fn decrypt(
        &self,
        ctx: &mut CryptoServiceContext<SymmKeyRef, AsymmKeyRef>,
        key: Key,
    ) -> Result<Option<Output>, crate::CryptoError> {
        self.as_ref()
            .map(|value| value.decrypt(ctx, key))
            .transpose()
    }
}

// Generic implementations for Vec values

impl<
        SymmKeyRef: SymmetricKeyRef,
        AsymmKeyRef: AsymmetricKeyRef,
        Key: KeyRef,
        T: Encryptable<SymmKeyRef, AsymmKeyRef, Key, Output>,
        Output,
    > Encryptable<SymmKeyRef, AsymmKeyRef, Key, Vec<Output>> for Vec<T>
{
    fn encrypt(
        &self,
        ctx: &mut CryptoServiceContext<SymmKeyRef, AsymmKeyRef>,
        key: Key,
    ) -> Result<Vec<Output>, crate::CryptoError> {
        self.iter().map(|value| value.encrypt(ctx, key)).collect()
    }
}

impl<
        SymmKeyRef: SymmetricKeyRef,
        AsymmKeyRef: AsymmetricKeyRef,
        Key: KeyRef,
        T: Decryptable<SymmKeyRef, AsymmKeyRef, Key, Output>,
        Output,
    > Decryptable<SymmKeyRef, AsymmKeyRef, Key, Vec<Output>> for Vec<T>
{
    fn decrypt(
        &self,
        ctx: &mut CryptoServiceContext<SymmKeyRef, AsymmKeyRef>,
        key: Key,
    ) -> Result<Vec<Output>, crate::CryptoError> {
        self.iter().map(|value| value.decrypt(ctx, key)).collect()
    }
}
