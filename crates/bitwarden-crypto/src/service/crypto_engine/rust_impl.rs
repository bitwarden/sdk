use std::sync::RwLock;

use rsa::Oaep;

use crate::{
    service::{
        crypto_engine::{CryptoEngine, CryptoEngineContext},
        key_ref::KeyRef,
        key_store::KeyStore,
        AsymmetricKeyRef, SymmetricKeyRef,
    },
    AsymmetricCryptoKey, AsymmetricEncString, CryptoError, EncString, SymmetricCryptoKey,
};

fn create_key_store<Key: KeyRef>() -> Box<dyn KeyStore<Key>> {
    #[cfg(target_os = "linux")]
    if let Some(key_store) = crate::service::key_store::LinuxMemfdSecretKeyStore::<Key>::new() {
        return Box::new(key_store);
    }

    Box::new(crate::service::key_store::RustKeyStore::new())
}

pub(crate) struct RustCryptoEngine<SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef> {
    key_stores: RwLock<RustCryptoEngineKeys<SymmKeyRef, AsymmKeyRef>>,
}

// This is just a wrapper around the keys so we only deal with one RwLock
struct RustCryptoEngineKeys<SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef> {
    symmetric_keys: Box<dyn KeyStore<SymmKeyRef>>,
    asymmetric_keys: Box<dyn KeyStore<AsymmKeyRef>>,
}

impl<SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef>
    RustCryptoEngine<SymmKeyRef, AsymmKeyRef>
{
    pub(crate) fn new() -> Self {
        Self {
            key_stores: RwLock::new(RustCryptoEngineKeys {
                symmetric_keys: create_key_store(),
                asymmetric_keys: create_key_store(),
            }),
        }
    }
}

impl<SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef>
    CryptoEngine<SymmKeyRef, AsymmKeyRef> for RustCryptoEngine<SymmKeyRef, AsymmKeyRef>
{
    fn context(&'_ self) -> Box<dyn CryptoEngineContext<'_, SymmKeyRef, AsymmKeyRef> + '_> {
        // TODO: Cache these?, or maybe initialize them lazily? or both?
        Box::new(RustCryptoEngineContext {
            global_keys: self.key_stores.read().expect("RwLock is poisoned"),
            local_symmetric_keys: create_key_store(),
            local_asymmetric_keys: create_key_store(),
        })
    }

    fn insert_symmetric_key(&self, key_ref: SymmKeyRef, key: SymmetricCryptoKey) {
        self.key_stores
            .write()
            .expect("RwLock is poisoned")
            .symmetric_keys
            .insert(key_ref, key);
    }

    fn clear(&self) {
        let mut keys = self.key_stores.write().expect("RwLock is poisoned");
        keys.symmetric_keys.clear();
        keys.asymmetric_keys.clear();
    }
}

pub(crate) struct RustCryptoEngineContext<
    'a,
    SymmKeyRef: SymmetricKeyRef,
    AsymmKeyRef: AsymmetricKeyRef,
> {
    // We hold a RwLock read guard to avoid having any nested
    //calls locking it again and potentially causing a deadlock
    global_keys: std::sync::RwLockReadGuard<'a, RustCryptoEngineKeys<SymmKeyRef, AsymmKeyRef>>,

    local_symmetric_keys: Box<dyn KeyStore<SymmKeyRef>>,
    local_asymmetric_keys: Box<dyn KeyStore<AsymmKeyRef>>,
}

impl<'a, SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef>
    RustCryptoEngineContext<'a, SymmKeyRef, AsymmKeyRef>
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
}

impl<'a, SymmKeyRef: SymmetricKeyRef, AsymmKeyRef: AsymmetricKeyRef>
    CryptoEngineContext<'a, SymmKeyRef, AsymmKeyRef>
    for RustCryptoEngineContext<'a, SymmKeyRef, AsymmKeyRef>
{
    fn clear(&mut self) {
        self.local_symmetric_keys.clear();
        self.local_asymmetric_keys.clear();
    }

    fn decrypt_data_with_symmetric_key(
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

    fn decrypt_and_store_symmetric_key(
        &mut self,
        encryption_key: SymmKeyRef,
        new_key_ref: SymmKeyRef,
        encrypted_key: &EncString,
    ) -> Result<SymmKeyRef, crate::CryptoError> {
        let mut new_key_material =
            self.decrypt_data_with_symmetric_key(encryption_key, encrypted_key)?;

        let new_key = SymmetricCryptoKey::try_from(new_key_material.as_mut_slice())?;
        self.local_symmetric_keys.insert(new_key_ref, new_key);
        Ok(new_key_ref)
    }

    fn encrypt_data_with_symmetric_key(
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

    fn encrypt_symmetric_key(
        &self,
        encryption_key: SymmKeyRef,
        key_to_encrypt: SymmKeyRef,
    ) -> Result<EncString, crate::CryptoError> {
        let key_to_encrypt = self.get_symmetric_key(key_to_encrypt)?;
        self.encrypt_data_with_symmetric_key(encryption_key, &key_to_encrypt.to_vec())
    }

    fn decrypt_data_with_asymmetric_key(
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

    fn decrypt_and_store_asymmetric_key(
        &mut self,
        encryption_key: AsymmKeyRef,
        new_key_ref: AsymmKeyRef,
        encrypted_key: &AsymmetricEncString,
    ) -> Result<AsymmKeyRef, crate::CryptoError> {
        let new_key_material =
            self.decrypt_data_with_asymmetric_key(encryption_key, encrypted_key)?;

        let new_key = AsymmetricCryptoKey::from_der(&new_key_material)?;
        self.local_asymmetric_keys.insert(new_key_ref, new_key);
        Ok(new_key_ref)
    }

    fn encrypt_data_with_asymmetric_key(
        &self,
        key: AsymmKeyRef,
        data: &[u8],
    ) -> Result<AsymmetricEncString, crate::CryptoError> {
        let key = self.get_asymmetric_key(key)?;
        AsymmetricEncString::encrypt_rsa2048_oaep_sha1(data, key)
    }

    fn encrypt_asymmetric_key(
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
