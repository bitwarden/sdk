use bitwarden_api_api::models::CipherIdentityModel;
use bitwarden_crypto::{
    CryptoError, EncString, KeyDecryptable, KeyEncryptable, SymmetricCryptoKey,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

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

impl KeyEncryptable<SymmetricCryptoKey, Identity> for IdentityView {
    fn encrypt_with_key(self, key: &SymmetricCryptoKey) -> Result<Identity, CryptoError> {
        Ok(Identity {
            title: self.title.encrypt_with_key(key)?,
            first_name: self.first_name.encrypt_with_key(key)?,
            middle_name: self.middle_name.encrypt_with_key(key)?,
            last_name: self.last_name.encrypt_with_key(key)?,
            address1: self.address1.encrypt_with_key(key)?,
            address2: self.address2.encrypt_with_key(key)?,
            address3: self.address3.encrypt_with_key(key)?,
            city: self.city.encrypt_with_key(key)?,
            state: self.state.encrypt_with_key(key)?,
            postal_code: self.postal_code.encrypt_with_key(key)?,
            country: self.country.encrypt_with_key(key)?,
            company: self.company.encrypt_with_key(key)?,
            email: self.email.encrypt_with_key(key)?,
            phone: self.phone.encrypt_with_key(key)?,
            ssn: self.ssn.encrypt_with_key(key)?,
            username: self.username.encrypt_with_key(key)?,
            passport_number: self.passport_number.encrypt_with_key(key)?,
            license_number: self.license_number.encrypt_with_key(key)?,
        })
    }
}

impl KeyDecryptable<SymmetricCryptoKey, IdentityView> for Identity {
    fn decrypt_with_key(&self, key: &SymmetricCryptoKey) -> Result<IdentityView, CryptoError> {
        Ok(IdentityView {
            title: self.title.decrypt_with_key(key).ok().flatten(),
            first_name: self.first_name.decrypt_with_key(key).ok().flatten(),
            middle_name: self.middle_name.decrypt_with_key(key).ok().flatten(),
            last_name: self.last_name.decrypt_with_key(key).ok().flatten(),
            address1: self.address1.decrypt_with_key(key).ok().flatten(),
            address2: self.address2.decrypt_with_key(key).ok().flatten(),
            address3: self.address3.decrypt_with_key(key).ok().flatten(),
            city: self.city.decrypt_with_key(key).ok().flatten(),
            state: self.state.decrypt_with_key(key).ok().flatten(),
            postal_code: self.postal_code.decrypt_with_key(key).ok().flatten(),
            country: self.country.decrypt_with_key(key).ok().flatten(),
            company: self.company.decrypt_with_key(key).ok().flatten(),
            email: self.email.decrypt_with_key(key).ok().flatten(),
            phone: self.phone.decrypt_with_key(key).ok().flatten(),
            ssn: self.ssn.decrypt_with_key(key).ok().flatten(),
            username: self.username.decrypt_with_key(key).ok().flatten(),
            passport_number: self.passport_number.decrypt_with_key(key).ok().flatten(),
            license_number: self.license_number.decrypt_with_key(key).ok().flatten(),
        })
    }
}

impl TryFrom<CipherIdentityModel> for Identity {
    type Error = Error;

    fn try_from(identity: CipherIdentityModel) -> Result<Self> {
        Ok(Self {
            title: EncString::try_from_optional(identity.title)?,
            first_name: EncString::try_from_optional(identity.first_name)?,
            middle_name: EncString::try_from_optional(identity.middle_name)?,
            last_name: EncString::try_from_optional(identity.last_name)?,
            address1: EncString::try_from_optional(identity.address1)?,
            address2: EncString::try_from_optional(identity.address2)?,
            address3: EncString::try_from_optional(identity.address3)?,
            city: EncString::try_from_optional(identity.city)?,
            state: EncString::try_from_optional(identity.state)?,
            postal_code: EncString::try_from_optional(identity.postal_code)?,
            country: EncString::try_from_optional(identity.country)?,
            company: EncString::try_from_optional(identity.company)?,
            email: EncString::try_from_optional(identity.email)?,
            phone: EncString::try_from_optional(identity.phone)?,
            ssn: EncString::try_from_optional(identity.ssn)?,
            username: EncString::try_from_optional(identity.username)?,
            passport_number: EncString::try_from_optional(identity.passport_number)?,
            license_number: EncString::try_from_optional(identity.license_number)?,
        })
    }
}
