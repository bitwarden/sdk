use std::{collections::HashMap, hash::Hash};

use crate::error::Result;

use super::SymmetricCryptoKey;

pub trait KeyEncryptable<Output> {
    fn encrypt_with_key(self, key: &SymmetricCryptoKey) -> Result<Output>;
}

pub trait KeyDecryptable<Output> {
    fn decrypt_with_key(&self, key: &SymmetricCryptoKey) -> Result<Output>;
}

impl<T: KeyEncryptable<Output>, Output> KeyEncryptable<Option<Output>> for Option<T> {
    fn encrypt_with_key(self, key: &SymmetricCryptoKey) -> Result<Option<Output>> {
        self.map(|e| e.encrypt_with_key(key)).transpose()
    }
}

impl<T: KeyDecryptable<Output>, Output> KeyDecryptable<Option<Output>> for Option<T> {
    fn decrypt_with_key(&self, key: &SymmetricCryptoKey) -> Result<Option<Output>> {
        self.as_ref().map(|e| e.decrypt_with_key(key)).transpose()
    }
}

impl<T: KeyEncryptable<Output>, Output> KeyEncryptable<Output> for Box<T> {
    fn encrypt_with_key(self, key: &SymmetricCryptoKey) -> Result<Output> {
        (*self).encrypt_with_key(key)
    }
}

impl<T: KeyDecryptable<Output>, Output> KeyDecryptable<Output> for Box<T> {
    fn decrypt_with_key(&self, key: &SymmetricCryptoKey) -> Result<Output> {
        (**self).decrypt_with_key(key)
    }
}

impl<T: KeyEncryptable<Output>, Output> KeyEncryptable<Vec<Output>> for Vec<T> {
    fn encrypt_with_key(self, key: &SymmetricCryptoKey) -> Result<Vec<Output>> {
        self.into_iter().map(|e| e.encrypt_with_key(key)).collect()
    }
}

impl<T: KeyDecryptable<Output>, Output> KeyDecryptable<Vec<Output>> for Vec<T> {
    fn decrypt_with_key(&self, key: &SymmetricCryptoKey) -> Result<Vec<Output>> {
        self.iter().map(|e| e.decrypt_with_key(key)).collect()
    }
}

impl<T: KeyEncryptable<Output>, Output, Id: Hash + Eq> KeyEncryptable<HashMap<Id, Output>>
    for HashMap<Id, T>
{
    fn encrypt_with_key(self, key: &SymmetricCryptoKey) -> Result<HashMap<Id, Output>> {
        self.into_iter()
            .map(|(id, e)| Ok((id, e.encrypt_with_key(key)?)))
            .collect()
    }
}

impl<T: KeyDecryptable<Output>, Output, Id: Hash + Eq + Copy> KeyDecryptable<HashMap<Id, Output>>
    for HashMap<Id, T>
{
    fn decrypt_with_key(&self, key: &SymmetricCryptoKey) -> Result<HashMap<Id, Output>> {
        self.iter()
            .map(|(id, e)| Ok((*id, e.decrypt_with_key(key)?)))
            .collect()
    }
}
