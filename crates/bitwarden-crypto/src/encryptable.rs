use std::{collections::HashMap, hash::Hash};

use uuid::Uuid;

use crate::{CryptoError, KeyDecryptable, KeyEncryptable, Result, SymmetricCryptoKey};

pub trait KeyContainer {
    fn get_key(&self, org_id: &Option<Uuid>) -> Option<&SymmetricCryptoKey>;
}

pub trait LocateKey {
    fn locate_key<'a>(
        &self,
        enc: &'a dyn KeyContainer,
        org_id: &Option<Uuid>,
    ) -> Option<&'a SymmetricCryptoKey> {
        enc.get_key(org_id)
    }
}

/// Deprecated: please use LocateKey and KeyDecryptable instead
pub trait Encryptable<Output> {
    fn encrypt(self, enc: &dyn KeyContainer, org_id: &Option<Uuid>) -> Result<Output>;
}

/// Deprecated: please use LocateKey and KeyDecryptable instead
pub trait Decryptable<Output> {
    fn decrypt(&self, enc: &dyn KeyContainer, org_id: &Option<Uuid>) -> Result<Output>;
}

impl<T: KeyEncryptable<SymmetricCryptoKey, Output> + LocateKey, Output> Encryptable<Output> for T {
    fn encrypt(self, enc: &dyn KeyContainer, org_id: &Option<Uuid>) -> Result<Output> {
        let key = self
            .locate_key(enc, org_id)
            .ok_or(CryptoError::MissingKey)?;
        self.encrypt_with_key(key)
    }
}

impl<T: KeyDecryptable<SymmetricCryptoKey, Output> + LocateKey, Output> Decryptable<Output> for T {
    fn decrypt(&self, enc: &dyn KeyContainer, org_id: &Option<Uuid>) -> Result<Output> {
        let key = self
            .locate_key(enc, org_id)
            .ok_or(CryptoError::MissingKey)?;
        self.decrypt_with_key(key)
    }
}

impl<T: Encryptable<Output>, Output> Encryptable<Vec<Output>> for Vec<T> {
    fn encrypt(self, enc: &dyn KeyContainer, org_id: &Option<Uuid>) -> Result<Vec<Output>> {
        self.into_iter().map(|e| e.encrypt(enc, org_id)).collect()
    }
}

impl<T: Decryptable<Output>, Output> Decryptable<Vec<Output>> for Vec<T> {
    fn decrypt(&self, enc: &dyn KeyContainer, org_id: &Option<Uuid>) -> Result<Vec<Output>> {
        self.iter().map(|e| e.decrypt(enc, org_id)).collect()
    }
}

impl<T: Encryptable<Output>, Output, Id: Hash + Eq> Encryptable<HashMap<Id, Output>>
    for HashMap<Id, T>
{
    fn encrypt(self, enc: &dyn KeyContainer, org_id: &Option<Uuid>) -> Result<HashMap<Id, Output>> {
        self.into_iter()
            .map(|(id, e)| Ok((id, e.encrypt(enc, org_id)?)))
            .collect()
    }
}

impl<T: Decryptable<Output>, Output, Id: Hash + Eq + Copy> Decryptable<HashMap<Id, Output>>
    for HashMap<Id, T>
{
    fn decrypt(
        &self,
        enc: &dyn KeyContainer,
        org_id: &Option<Uuid>,
    ) -> Result<HashMap<Id, Output>> {
        self.iter()
            .map(|(id, e)| Ok((*id, e.decrypt(enc, org_id)?)))
            .collect()
    }
}
