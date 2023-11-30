use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    crypto::{EncString, KeyDecryptable, KeyEncryptable, SymmetricCryptoKey},
    error::Result,
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct Card {
    pub cardholder_name: Option<EncString>,
    pub exp_month: Option<EncString>,
    pub exp_year: Option<EncString>,
    pub code: Option<EncString>,
    pub brand: Option<EncString>,
    pub number: Option<EncString>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct CardView {
    pub cardholder_name: Option<String>,
    pub exp_month: Option<String>,
    pub exp_year: Option<String>,
    pub code: Option<String>,
    pub brand: Option<String>,
    pub number: Option<String>,
}

impl KeyEncryptable<Card> for CardView {
    fn encrypt_with_key(self, key: &SymmetricCryptoKey) -> Result<Card> {
        Ok(Card {
            cardholder_name: self.cardholder_name.encrypt_with_key(key)?,
            exp_month: self.exp_month.encrypt_with_key(key)?,
            exp_year: self.exp_year.encrypt_with_key(key)?,
            code: self.code.encrypt_with_key(key)?,
            brand: self.brand.encrypt_with_key(key)?,
            number: self.number.encrypt_with_key(key)?,
        })
    }
}

impl KeyDecryptable<CardView> for Card {
    fn decrypt_with_key(&self, key: &SymmetricCryptoKey) -> Result<CardView> {
        Ok(CardView {
            cardholder_name: self.cardholder_name.decrypt_with_key(key)?,
            exp_month: self.exp_month.decrypt_with_key(key)?,
            exp_year: self.exp_year.decrypt_with_key(key)?,
            code: self.code.decrypt_with_key(key)?,
            brand: self.brand.decrypt_with_key(key)?,
            number: self.number.decrypt_with_key(key)?,
        })
    }
}
