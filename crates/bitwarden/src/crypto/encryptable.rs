use std::{collections::HashMap, hash::Hash};

use uuid::Uuid;

use crate::{client::encryption_settings::EncryptionSettings, error::Result};

pub trait Encryptable<Output> {
    fn encrypt(self, enc: &EncryptionSettings, org_id: &Option<Uuid>) -> Result<Output>;
}

pub trait Decryptable<Output> {
    fn decrypt(&self, enc: &EncryptionSettings, org_id: &Option<Uuid>) -> Result<Output>;
}

impl<T: Encryptable<Output>, Output> Encryptable<Option<Output>> for Option<T> {
    fn encrypt(self, enc: &EncryptionSettings, org_id: &Option<Uuid>) -> Result<Option<Output>> {
        self.map(|e| e.encrypt(enc, org_id)).transpose()
    }
}

impl<T: Decryptable<Output>, Output> Decryptable<Option<Output>> for Option<T> {
    fn decrypt(&self, enc: &EncryptionSettings, org_id: &Option<Uuid>) -> Result<Option<Output>> {
        self.as_ref().map(|e| e.decrypt(enc, org_id)).transpose()
    }
}

impl<T: Encryptable<Output>, Output> Encryptable<Vec<Output>> for Vec<T> {
    fn encrypt(self, enc: &EncryptionSettings, org_id: &Option<Uuid>) -> Result<Vec<Output>> {
        self.into_iter().map(|e| e.encrypt(enc, org_id)).collect()
    }
}

impl<T: Decryptable<Output>, Output> Decryptable<Vec<Output>> for Vec<T> {
    fn decrypt(&self, enc: &EncryptionSettings, org_id: &Option<Uuid>) -> Result<Vec<Output>> {
        self.iter().map(|e| e.decrypt(enc, org_id)).collect()
    }
}

impl<T: Encryptable<Output>, Output, Id: Hash + Eq> Encryptable<HashMap<Id, Output>>
    for HashMap<Id, T>
{
    fn encrypt(
        self,
        enc: &EncryptionSettings,
        org_id: &Option<Uuid>,
    ) -> Result<HashMap<Id, Output>> {
        self.into_iter()
            .map(|(id, e)| Ok((id, e.encrypt(enc, org_id)?)))
            .collect::<Result<HashMap<_, _>>>()
    }
}

impl<T: Decryptable<Output>, Output, Id: Hash + Eq + Copy> Decryptable<HashMap<Id, Output>>
    for HashMap<Id, T>
{
    fn decrypt(
        &self,
        enc: &EncryptionSettings,
        org_id: &Option<Uuid>,
    ) -> Result<HashMap<Id, Output>> {
        self.iter()
            .map(|(id, e)| Ok((*id, e.decrypt(enc, org_id)?)))
            .collect::<Result<HashMap<_, _>>>()
    }
}

impl<T: Encryptable<Output>, Output> Encryptable<Output> for Box<T> {
    fn encrypt(self, enc: &EncryptionSettings, org_id: &Option<Uuid>) -> Result<Output> {
        (*self).encrypt(enc, org_id)
    }
}

impl<T: Decryptable<Output>, Output> Decryptable<Output> for Box<T> {
    fn decrypt(&self, enc: &EncryptionSettings, org_id: &Option<Uuid>) -> Result<Output> {
        (**self).decrypt(enc, org_id)
    }
}
