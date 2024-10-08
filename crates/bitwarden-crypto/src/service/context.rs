use std::sync::{RwLockReadGuard, RwLockWriteGuard};

use rsa::Oaep;
use zeroize::Zeroizing;

use super::Keys;
use crate::{
    derive_shareable_key,
    service::{key_store::KeyStore, AsymmetricKeyRef, SymmetricKeyRef},
    AsymmetricCryptoKey, AsymmetricEncString, CryptoError, EncString, Result, SymmetricCryptoKey,
};

// This is to abstract over the read-only and read-write access to the global keys
// inside the CryptoServiceContext. The read-write access should only be used internally
// in this crate to avoid users leaving the crypto store in an inconsistent state,
// but for the moment we have some operations that require access to it.
pub trait GlobalAccessMode<'a, SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef> {
    fn get(&self) -> &Keys<SymmKeyRef, AsymmKeyRef>;
    fn get_mut(&mut self) -> Result<&mut Keys<SymmKeyRef, AsymmKeyRef>>;
}

pub struct ReadOnlyGlobal<'a, SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef>(
    pub(super) RwLockReadGuard<'a, Keys<SymmKeyRef, AsymmKeyRef>>,
);

impl<'a, SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef>
    GlobalAccessMode<'a, SymmKeyRef, AsymmKeyRef> for ReadOnlyGlobal<'a, SymmKeyRef, AsymmKeyRef>
{
    fn get(&self) -> &Keys<SymmKeyRef, AsymmKeyRef> {
        &self.0
    }

    fn get_mut(&mut self) -> Result<&mut Keys<SymmKeyRef, AsymmKeyRef>> {
        Err(crate::CryptoError::ReadOnlyCryptoStore)
    }
}

pub struct ReadWriteGlobal<'a, SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef>(
    pub(super) RwLockWriteGuard<'a, Keys<SymmKeyRef, AsymmKeyRef>>,
);

impl<'a, SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef>
    GlobalAccessMode<'a, SymmKeyRef, AsymmKeyRef> for ReadWriteGlobal<'a, SymmKeyRef, AsymmKeyRef>
{
    fn get(&self) -> &Keys<SymmKeyRef, AsymmKeyRef> {
        &self.0
    }

    fn get_mut(&mut self) -> Result<&mut Keys<SymmKeyRef, AsymmKeyRef>> {
        Ok(&mut self.0)
    }
}

pub struct CryptoServiceContext<
    'a,
    SymmKeyRef: SymmetricKeyRef,
    AsymmKeyRef: AsymmetricKeyRef,
    AccessMode: GlobalAccessMode<'a, SymmKeyRef, AsymmKeyRef> = ReadOnlyGlobal<
        'a,
        SymmKeyRef,
        AsymmKeyRef,
    >,
> {
    pub(super) global: AccessMode,

    pub(super) local_symmetric_keys: Box<dyn KeyStore<SymmKeyRef>>,
    pub(super) local_asymmetric_keys: Box<dyn KeyStore<AsymmKeyRef>>,

    pub(super) _phantom: std::marker::PhantomData<&'a ()>,
}

impl<
        'a,
        SymmKeyRef: SymmetricKeyRef,
        AsymmKeyRef: AsymmetricKeyRef,
        AccessMode: GlobalAccessMode<'a, SymmKeyRef, AsymmKeyRef>,
    > CryptoServiceContext<'a, SymmKeyRef, AsymmKeyRef, AccessMode>
{
    pub fn clear(&mut self) {
        // Clear global keys if we have write access
        if let Ok(keys) = self.global.get_mut() {
            keys.symmetric_keys.clear();
            keys.asymmetric_keys.clear();
        }

        self.local_symmetric_keys.clear();
        self.local_asymmetric_keys.clear();
    }

    pub fn retain_symmetric_keys(&mut self, f: fn(SymmKeyRef) -> bool) {
        if let Ok(keys) = self.global.get_mut() {
            keys.symmetric_keys.retain(f);
        }
        self.local_symmetric_keys.retain(f);
    }

    pub fn retain_asymmetric_keys(&mut self, f: fn(AsymmKeyRef) -> bool) {
        if let Ok(keys) = self.global.get_mut() {
            keys.asymmetric_keys.retain(f);
        }
        self.local_asymmetric_keys.retain(f);
    }

    /// TODO: All these encrypt x key with x key look like they need to be made generic,
    /// but I haven't found the best way to do that yet.

    pub fn decrypt_symmetric_key_with_symmetric_key(
        &mut self,
        encryption_key: SymmKeyRef,
        new_key_ref: SymmKeyRef,
        encrypted_key: &EncString,
    ) -> Result<SymmKeyRef> {
        let mut new_key_material =
            self.decrypt_data_with_symmetric_key(encryption_key, encrypted_key)?;

        #[allow(deprecated)]
        self.set_symmetric_key(
            new_key_ref,
            SymmetricCryptoKey::try_from(new_key_material.as_mut_slice())?,
        )?;

        // Returning the new key reference for convenience
        Ok(new_key_ref)
    }

    pub fn encrypt_symmetric_key_with_symmetric_key(
        &self,
        encryption_key: SymmKeyRef,
        key_to_encrypt: SymmKeyRef,
    ) -> Result<EncString> {
        let key_to_encrypt = self.get_symmetric_key(key_to_encrypt)?;
        self.encrypt_data_with_symmetric_key(encryption_key, &key_to_encrypt.to_vec())
    }

    pub fn decrypt_symmetric_key_with_asymmetric_key(
        &mut self,
        encryption_key: AsymmKeyRef,
        new_key_ref: SymmKeyRef,
        encrypted_key: &AsymmetricEncString,
    ) -> Result<SymmKeyRef> {
        let mut new_key_material =
            self.decrypt_data_with_asymmetric_key(encryption_key, encrypted_key)?;

        #[allow(deprecated)]
        self.set_symmetric_key(
            new_key_ref,
            SymmetricCryptoKey::try_from(new_key_material.as_mut_slice())?,
        )?;

        // Returning the new key reference for convenience
        Ok(new_key_ref)
    }

    pub fn encrypt_symmetric_key_with_asymmetric_key(
        &self,
        encryption_key: AsymmKeyRef,
        key_to_encrypt: SymmKeyRef,
    ) -> Result<AsymmetricEncString> {
        let key_to_encrypt = self.get_symmetric_key(key_to_encrypt)?;
        self.encrypt_data_with_asymmetric_key(encryption_key, &key_to_encrypt.to_vec())
    }

    pub fn decrypt_asymmetric_key(
        &mut self,
        encryption_key: AsymmKeyRef,
        new_key_ref: AsymmKeyRef,
        encrypted_key: &AsymmetricEncString,
    ) -> Result<AsymmKeyRef> {
        let new_key_material =
            self.decrypt_data_with_asymmetric_key(encryption_key, encrypted_key)?;

        #[allow(deprecated)]
        self.set_asymmetric_key(
            new_key_ref,
            AsymmetricCryptoKey::from_der(&new_key_material)?,
        )?;

        // Returning the new key reference for convenience
        Ok(new_key_ref)
    }

    pub fn encrypt_asymmetric_key(
        &self,
        encryption_key: AsymmKeyRef,
        key_to_encrypt: AsymmKeyRef,
    ) -> Result<AsymmetricEncString> {
        let encryption_key = self.get_asymmetric_key(encryption_key)?;
        let key_to_encrypt = self.get_asymmetric_key(key_to_encrypt)?;

        AsymmetricEncString::encrypt_rsa2048_oaep_sha1(
            key_to_encrypt.to_der()?.as_slice(),
            encryption_key,
        )
    }

    pub fn has_symmetric_key(&self, key_ref: SymmKeyRef) -> bool {
        self.get_symmetric_key(key_ref).is_ok()
    }

    pub fn has_asymmetric_key(&self, key_ref: AsymmKeyRef) -> bool {
        self.get_asymmetric_key(key_ref).is_ok()
    }

    pub fn generate_symmetric_key(&mut self, key_ref: SymmKeyRef) -> Result<SymmKeyRef> {
        let key = SymmetricCryptoKey::generate(rand::thread_rng());
        #[allow(deprecated)]
        self.set_symmetric_key(key_ref, key)?;
        Ok(key_ref)
    }

    pub fn derive_shareable_key(
        &mut self,
        key_ref: SymmKeyRef,
        secret: Zeroizing<[u8; 16]>,
        name: &str,
        info: Option<&str>,
    ) -> Result<SymmKeyRef> {
        #[allow(deprecated)]
        self.set_symmetric_key(key_ref, derive_shareable_key(secret, name, info))?;
        Ok(key_ref)
    }

    #[deprecated(note = "This function should ideally never be used outside this crate")]
    pub fn dangerous_get_symmetric_key(&self, key_ref: SymmKeyRef) -> Result<&SymmetricCryptoKey> {
        self.get_symmetric_key(key_ref)
    }

    #[deprecated(note = "This function should ideally never be used outside this crate")]
    pub fn dangerous_get_asymmetric_key(
        &self,
        key_ref: AsymmKeyRef,
    ) -> Result<&AsymmetricCryptoKey> {
        self.get_asymmetric_key(key_ref)
    }

    fn get_symmetric_key(&self, key_ref: SymmKeyRef) -> Result<&SymmetricCryptoKey> {
        if key_ref.is_local() {
            self.local_symmetric_keys.get(key_ref)
        } else {
            self.global.get().symmetric_keys.get(key_ref)
        }
        .ok_or_else(|| crate::CryptoError::MissingKey(format!("{key_ref:?}")))
    }

    fn get_asymmetric_key(&self, key_ref: AsymmKeyRef) -> Result<&AsymmetricCryptoKey> {
        if key_ref.is_local() {
            self.local_asymmetric_keys.get(key_ref)
        } else {
            self.global.get().asymmetric_keys.get(key_ref)
        }
        .ok_or_else(|| crate::CryptoError::MissingKey(format!("{key_ref:?}")))
    }

    #[deprecated(note = "This function should ideally never be used outside this crate")]
    pub fn set_symmetric_key(
        &mut self,
        key_ref: SymmKeyRef,
        key: SymmetricCryptoKey,
    ) -> Result<()> {
        if key_ref.is_local() {
            self.local_symmetric_keys.insert(key_ref, key);
        } else {
            self.global.get_mut()?.symmetric_keys.insert(key_ref, key);
        }
        Ok(())
    }

    #[deprecated(note = "This function should ideally never be used outside this crate")]
    pub fn set_asymmetric_key(
        &mut self,
        key_ref: AsymmKeyRef,
        key: AsymmetricCryptoKey,
    ) -> Result<()> {
        if key_ref.is_local() {
            self.local_asymmetric_keys.insert(key_ref, key);
        } else {
            self.global.get_mut()?.asymmetric_keys.insert(key_ref, key);
        }
        Ok(())
    }

    pub(crate) fn decrypt_data_with_symmetric_key(
        &self,
        key: SymmKeyRef,
        data: &EncString,
    ) -> Result<Vec<u8>> {
        let key = self.get_symmetric_key(key)?;

        match data {
            EncString::AesCbc256_B64 { iv, data } => {
                let dec = crate::aes::decrypt_aes256(iv, data.clone(), &key.key)?;
                Ok(dec)
            }
            EncString::AesCbc128_HmacSha256_B64 { iv, mac, data } => {
                // TODO: SymmetricCryptoKey is designed to handle 32 byte keys only, but this
                // variant uses a 16 byte key This means the key+mac are going to be
                // parsed as a single 32 byte key, at the moment we split it manually
                // When refactoring the key handling, this should be fixed.
                let enc_key = (&key.key[0..16]).into();
                let mac_key = (&key.key[16..32]).into();
                let dec = crate::aes::decrypt_aes128_hmac(iv, mac, data.clone(), mac_key, enc_key)?;
                Ok(dec)
            }
            EncString::AesCbc256_HmacSha256_B64 { iv, mac, data } => {
                let mac_key = key.mac_key.as_ref().ok_or(CryptoError::InvalidMac)?;
                let dec =
                    crate::aes::decrypt_aes256_hmac(iv, mac, data.clone(), mac_key, &key.key)?;
                Ok(dec)
            }
        }
    }

    pub(crate) fn encrypt_data_with_symmetric_key(
        &self,
        key: SymmKeyRef,
        data: &[u8],
    ) -> Result<EncString> {
        let key = self.get_symmetric_key(key)?;
        EncString::encrypt_aes256_hmac(
            data,
            key.mac_key.as_ref().ok_or(CryptoError::InvalidMac)?,
            &key.key,
        )
    }

    pub(crate) fn decrypt_data_with_asymmetric_key(
        &self,
        key: AsymmKeyRef,
        data: &AsymmetricEncString,
    ) -> Result<Vec<u8>> {
        let key = self.get_asymmetric_key(key)?;

        use AsymmetricEncString::*;
        match data {
            Rsa2048_OaepSha256_B64 { data } => key.key.decrypt(Oaep::new::<sha2::Sha256>(), data),
            Rsa2048_OaepSha1_B64 { data } => key.key.decrypt(Oaep::new::<sha1::Sha1>(), data),
            #[allow(deprecated)]
            Rsa2048_OaepSha256_HmacSha256_B64 { data, .. } => {
                key.key.decrypt(Oaep::new::<sha2::Sha256>(), data)
            }
            #[allow(deprecated)]
            Rsa2048_OaepSha1_HmacSha256_B64 { data, .. } => {
                key.key.decrypt(Oaep::new::<sha1::Sha1>(), data)
            }
        }
        .map_err(|_| CryptoError::KeyDecrypt)
    }

    pub(crate) fn encrypt_data_with_asymmetric_key(
        &self,
        key: AsymmKeyRef,
        data: &[u8],
    ) -> Result<AsymmetricEncString> {
        let key = self.get_asymmetric_key(key)?;
        AsymmetricEncString::encrypt_rsa2048_oaep_sha1(data, key)
    }
}
