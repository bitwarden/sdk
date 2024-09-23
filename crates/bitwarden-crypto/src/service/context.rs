use rsa::Oaep;

use crate::{
    service::{key_store::KeyStore, AsymmetricKeyRef, SymmetricKeyRef},
    AsymmetricCryptoKey, AsymmetricEncString, CryptoError, EncString, SymmetricCryptoKey,
};

use super::RustCryptoServiceKeys;

pub(crate) struct RustCryptoServiceContext<
    'a,
    SymmKeyRef: SymmetricKeyRef,
    AsymmKeyRef: AsymmetricKeyRef,
> {
    // We hold a RwLock read guard to avoid having any nested
    //calls locking it again and potentially causing a deadlock
    pub(crate) global_keys:
        std::sync::RwLockReadGuard<'a, RustCryptoServiceKeys<SymmKeyRef, AsymmKeyRef>>,

    pub(crate) local_symmetric_keys: Box<dyn KeyStore<SymmKeyRef>>,
    pub(crate) local_asymmetric_keys: Box<dyn KeyStore<AsymmKeyRef>>,
}

impl<'a, SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef>
    RustCryptoServiceContext<'a, SymmKeyRef, AsymmKeyRef>
{
    fn get_symmetric_key(
        &self,
        key_ref: SymmKeyRef,
    ) -> Result<&SymmetricCryptoKey, crate::CryptoError> {
        if key_ref.is_local() {
            self.local_symmetric_keys.get(key_ref)
        } else {
            self.global_keys.symmetric_keys.get(key_ref)
        }
        .ok_or_else(|| crate::CryptoError::MissingKey2(format!("{key_ref:?}")))
    }

    fn get_asymmetric_key(
        &self,
        key_ref: AsymmKeyRef,
    ) -> Result<&AsymmetricCryptoKey, crate::CryptoError> {
        if key_ref.is_local() {
            self.local_asymmetric_keys.get(key_ref)
        } else {
            self.global_keys.asymmetric_keys.get(key_ref)
        }
        .ok_or_else(|| crate::CryptoError::MissingKey2(format!("{key_ref:?}")))
    }

    pub fn clear(&mut self) {
        self.local_symmetric_keys.clear();
        self.local_asymmetric_keys.clear();
    }

    pub fn remove_symmetric_key(&mut self, key_ref: SymmKeyRef) {
        self.local_symmetric_keys.remove(key_ref);
    }

    pub fn decrypt_data_with_symmetric_key(
        &self,
        key: SymmKeyRef,
        data: &EncString,
    ) -> Result<Vec<u8>, crate::CryptoError> {
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

    pub fn decrypt_and_store_symmetric_key(
        &mut self,
        encryption_key: SymmKeyRef,
        new_key_ref: SymmKeyRef,
        encrypted_key: &EncString,
    ) -> Result<(), crate::CryptoError> {
        let mut new_key_material =
            self.decrypt_data_with_symmetric_key(encryption_key, encrypted_key)?;

        let new_key = SymmetricCryptoKey::try_from(new_key_material.as_mut_slice())?;
        self.local_symmetric_keys.insert(new_key_ref, new_key);
        Ok(())
    }

    pub fn encrypt_data_with_symmetric_key(
        &self,
        key: SymmKeyRef,
        data: &[u8],
    ) -> Result<EncString, crate::CryptoError> {
        let key = self.get_symmetric_key(key)?;
        EncString::encrypt_aes256_hmac(
            data,
            key.mac_key.as_ref().ok_or(CryptoError::InvalidMac)?,
            &key.key,
        )
    }

    pub fn encrypt_symmetric_key(
        &self,
        encryption_key: SymmKeyRef,
        key_to_encrypt: SymmKeyRef,
    ) -> Result<EncString, crate::CryptoError> {
        let key_to_encrypt = self.get_symmetric_key(key_to_encrypt)?;
        self.encrypt_data_with_symmetric_key(encryption_key, &key_to_encrypt.to_vec())
    }

    pub fn remove_asymmetric_key(&mut self, key_ref: AsymmKeyRef) {
        self.local_asymmetric_keys.remove(key_ref);
    }

    pub fn decrypt_data_with_asymmetric_key(
        &self,
        key: AsymmKeyRef,
        data: &AsymmetricEncString,
    ) -> Result<Vec<u8>, crate::CryptoError> {
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

    pub fn decrypt_and_store_asymmetric_key(
        &mut self,
        encryption_key: AsymmKeyRef,
        new_key_ref: AsymmKeyRef,
        encrypted_key: &AsymmetricEncString,
    ) -> Result<(), crate::CryptoError> {
        let new_key_material =
            self.decrypt_data_with_asymmetric_key(encryption_key, encrypted_key)?;

        let new_key = AsymmetricCryptoKey::from_der(&new_key_material)?;
        self.local_asymmetric_keys.insert(new_key_ref, new_key);
        Ok(())
    }
    pub fn encrypt_data_with_asymmetric_key(
        &self,
        key: AsymmKeyRef,
        data: &[u8],
    ) -> Result<AsymmetricEncString, crate::CryptoError> {
        let key = self.get_asymmetric_key(key)?;
        AsymmetricEncString::encrypt_rsa2048_oaep_sha1(data, key)
    }

    pub fn encrypt_asymmetric_key(
        &self,
        encryption_key: AsymmKeyRef,
        key_to_encrypt: AsymmKeyRef,
    ) -> Result<AsymmetricEncString, crate::CryptoError> {
        let encryption_key = self.get_asymmetric_key(encryption_key)?;
        let key_to_encrypt = self.get_asymmetric_key(key_to_encrypt)?;

        AsymmetricEncString::encrypt_rsa2048_oaep_sha1(
            key_to_encrypt.to_der()?.as_slice(),
            encryption_key,
        )
    }
}
