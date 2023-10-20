use bitwarden_api_api::models::CipherIdentityModel;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    client::encryption_settings::EncryptionSettings,
    crypto::{Decryptable, EncString, Encryptable},
    error::{Error, Result},
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct Identity {
    pub title: Option<EncString>,
    pub first_name: Option<EncString>,
    pub middle_name: Option<EncString>,
    pub last_name: Option<EncString>,
    pub address1: Option<EncString>,
    pub address2: Option<EncString>,
    pub address3: Option<EncString>,
    pub city: Option<EncString>,
    pub state: Option<EncString>,
    pub postal_code: Option<EncString>,
    pub country: Option<EncString>,
    pub company: Option<EncString>,
    pub email: Option<EncString>,
    pub phone: Option<EncString>,
    pub ssn: Option<EncString>,
    pub username: Option<EncString>,
    pub passport_number: Option<EncString>,
    pub license_number: Option<EncString>,
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

impl TryFrom<CipherIdentityModel> for Identity {
    type Error = Error;

    fn try_from(identity: CipherIdentityModel) -> Result<Self> {
        Ok(Self {
            title: EncString::try_from(identity.title)?,
            first_name: EncString::try_from(identity.first_name)?,
            middle_name: EncString::try_from(identity.middle_name)?,
            last_name: EncString::try_from(identity.last_name)?,
            address1: EncString::try_from(identity.address1)?,
            address2: EncString::try_from(identity.address2)?,
            address3: EncString::try_from(identity.address3)?,
            city: EncString::try_from(identity.city)?,
            state: EncString::try_from(identity.state)?,
            postal_code: EncString::try_from(identity.postal_code)?,
            country: EncString::try_from(identity.country)?,
            company: EncString::try_from(identity.company)?,
            email: EncString::try_from(identity.email)?,
            phone: EncString::try_from(identity.phone)?,
            ssn: EncString::try_from(identity.ssn)?,
            username: EncString::try_from(identity.username)?,
            passport_number: EncString::try_from(identity.passport_number)?,
            license_number: EncString::try_from(identity.license_number)?,
        })
    }
}
