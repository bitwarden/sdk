use std::{collections::HashMap, hash::Hash, sync::Arc};

use rayon::prelude::*;
use uuid::Uuid;

use crate::{error::Result, CryptoError, SymmetricCryptoKey};

pub trait KeyContainer: Send + Sync {
    fn get_key(&self, org_id: &Option<Uuid>) -> Result<&SymmetricCryptoKey, CryptoError>;
}

impl<T: KeyContainer> KeyContainer for Arc<T> {
    fn get_key(&self, org_id: &Option<Uuid>) -> Result<&SymmetricCryptoKey, CryptoError> {
        self.as_ref().get_key(org_id)
    }
}

pub trait CryptoKey {}

pub trait KeyEncryptable<Key: CryptoKey, Output> {
    fn encrypt_with_key(self, key: &Key) -> Result<Output>;
}

pub trait KeyDecryptable<Key: CryptoKey, Output> {
    fn decrypt_with_key(&self, key: &Key) -> Result<Output>;
}

impl<T: KeyEncryptable<Key, Output>, Key: CryptoKey, Output> KeyEncryptable<Key, Option<Output>>
    for Option<T>
{
    fn encrypt_with_key(self, key: &Key) -> Result<Option<Output>> {
        self.map(|e| e.encrypt_with_key(key)).transpose()
    }
}

impl<T: KeyDecryptable<Key, Output>, Key: CryptoKey, Output> KeyDecryptable<Key, Option<Output>>
    for Option<T>
{
    fn decrypt_with_key(&self, key: &Key) -> Result<Option<Output>> {
        self.as_ref().map(|e| e.decrypt_with_key(key)).transpose()
    }
}

impl<T: KeyEncryptable<Key, Output>, Key: CryptoKey, Output> KeyEncryptable<Key, Output>
    for Box<T>
{
    fn encrypt_with_key(self, key: &Key) -> Result<Output> {
        (*self).encrypt_with_key(key)
    }
}

impl<T: KeyDecryptable<Key, Output>, Key: CryptoKey, Output> KeyDecryptable<Key, Output>
    for Box<T>
{
    fn decrypt_with_key(&self, key: &Key) -> Result<Output> {
        (**self).decrypt_with_key(key)
    }
}

impl<
        T: KeyEncryptable<Key, Output> + Send + Sync,
        Key: CryptoKey + Send + Sync,
        Output: Send + Sync,
    > KeyEncryptable<Key, Vec<Output>> for Vec<T>
{
    fn encrypt_with_key(self, key: &Key) -> Result<Vec<Output>> {
        self.into_par_iter()
            .map(|e| e.encrypt_with_key(key))
            .collect()
    }
}

impl<
        T: KeyDecryptable<Key, Output> + Send + Sync,
        Key: CryptoKey + Send + Sync,
        Output: Send + Sync,
    > KeyDecryptable<Key, Vec<Output>> for Vec<T>
{
    fn decrypt_with_key(&self, key: &Key) -> Result<Vec<Output>> {
        self.into_par_iter()
            .map(|e| e.decrypt_with_key(key))
            .collect()
    }
}

impl<
        T: KeyEncryptable<Key, Output> + Send + Sync,
        Key: CryptoKey + Send + Sync,
        Output: Send + Sync,
        Id: Hash + Eq + Send + Sync,
    > KeyEncryptable<Key, HashMap<Id, Output>> for HashMap<Id, T>
{
    fn encrypt_with_key(self, key: &Key) -> Result<HashMap<Id, Output>> {
        self.into_par_iter()
            .map(|(id, e)| Ok((id, e.encrypt_with_key(key)?)))
            .collect()
    }
}

impl<
        T: KeyDecryptable<Key, Output> + Send + Sync,
        Key: CryptoKey + Send + Sync,
        Output: Send + Sync,
        Id: Hash + Eq + Copy + Send + Sync,
    > KeyDecryptable<Key, HashMap<Id, Output>> for HashMap<Id, T>
{
    fn decrypt_with_key(&self, key: &Key) -> Result<HashMap<Id, Output>> {
        self.into_par_iter()
            .map(|(id, e)| Ok((*id, e.decrypt_with_key(key)?)))
            .collect()
    }
}
