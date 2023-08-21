use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    client::encryption_settings::EncryptionSettings,
    crypto::{CipherString, Decryptable, Encryptable},
    error::Result,
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct Identity {
    pub title: Option<CipherString>,
    pub first_name: Option<CipherString>,
    pub middle_name: Option<CipherString>,
    pub last_name: Option<CipherString>,
    pub address1: Option<CipherString>,
    pub address2: Option<CipherString>,
    pub address3: Option<CipherString>,
    pub city: Option<CipherString>,
    pub state: Option<CipherString>,
    pub postal_code: Option<CipherString>,
    pub country: Option<CipherString>,
    pub company: Option<CipherString>,
    pub email: Option<CipherString>,
    pub phone: Option<CipherString>,
    pub ssn: Option<CipherString>,
    pub username: Option<CipherString>,
    pub passport_number: Option<CipherString>,
    pub license_number: Option<CipherString>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct IdentityView {
    pub title: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub address3: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub company: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub ssn: Option<String>,
    pub username: Option<String>,
    pub passport_number: Option<String>,
    pub license_number: Option<String>,
}

impl Encryptable<Identity> for IdentityView {
    fn encrypt(self, enc: &EncryptionSettings, org_id: &Option<Uuid>) -> Result<Identity> {
        Ok(Identity {
            title: self.title.encrypt(enc, org_id)?,
            first_name: self.first_name.encrypt(enc, org_id)?,
            middle_name: self.middle_name.encrypt(enc, org_id)?,
            last_name: self.last_name.encrypt(enc, org_id)?,
            address1: self.address1.encrypt(enc, org_id)?,
            address2: self.address2.encrypt(enc, org_id)?,
            address3: self.address3.encrypt(enc, org_id)?,
            city: self.city.encrypt(enc, org_id)?,
            state: self.state.encrypt(enc, org_id)?,
            postal_code: self.postal_code.encrypt(enc, org_id)?,
            country: self.country.encrypt(enc, org_id)?,
            company: self.company.encrypt(enc, org_id)?,
            email: self.email.encrypt(enc, org_id)?,
            phone: self.phone.encrypt(enc, org_id)?,
            ssn: self.ssn.encrypt(enc, org_id)?,
            username: self.username.encrypt(enc, org_id)?,
            passport_number: self.passport_number.encrypt(enc, org_id)?,
            license_number: self.license_number.encrypt(enc, org_id)?,
        })
    }
}

impl Decryptable<IdentityView> for Identity {
    fn decrypt(&self, enc: &EncryptionSettings, org_id: &Option<Uuid>) -> Result<IdentityView> {
        Ok(IdentityView {
            title: self.title.decrypt(enc, org_id)?,
            first_name: self.first_name.decrypt(enc, org_id)?,
            middle_name: self.middle_name.decrypt(enc, org_id)?,
            last_name: self.last_name.decrypt(enc, org_id)?,
            address1: self.address1.decrypt(enc, org_id)?,
            address2: self.address2.decrypt(enc, org_id)?,
            address3: self.address3.decrypt(enc, org_id)?,
            city: self.city.decrypt(enc, org_id)?,
            state: self.state.decrypt(enc, org_id)?,
            postal_code: self.postal_code.decrypt(enc, org_id)?,
            country: self.country.decrypt(enc, org_id)?,
            company: self.company.decrypt(enc, org_id)?,
            email: self.email.decrypt(enc, org_id)?,
            phone: self.phone.decrypt(enc, org_id)?,
            ssn: self.ssn.decrypt(enc, org_id)?,
            username: self.username.decrypt(enc, org_id)?,
            passport_number: self.passport_number.decrypt(enc, org_id)?,
            license_number: self.license_number.decrypt(enc, org_id)?,
        })
    }
}
