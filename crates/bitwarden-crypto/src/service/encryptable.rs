use super::{
    key_ref::{AsymmetricKeyRef, KeyRef, SymmetricKeyRef},
    CryptoServiceContext,
};

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
    Encryptable<SymmKeyRef, AsymmKeyRef, SymmKeyRef, EncString> for [u8]
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
    Encryptable<SymmKeyRef, AsymmKeyRef, AsymmKeyRef, AsymmetricEncString> for [u8]
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
    Encryptable<SymmKeyRef, AsymmKeyRef, SymmKeyRef, EncString> for str
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
    Encryptable<SymmKeyRef, AsymmKeyRef, AsymmKeyRef, AsymmetricEncString> for str
{
    fn encrypt(
        &self,
        ctx: &mut CryptoServiceContext<SymmKeyRef, AsymmKeyRef>,
        key: AsymmKeyRef,
    ) -> Result<AsymmetricEncString, crate::CryptoError> {
        self.as_bytes().encrypt(ctx, key)
    }
}
