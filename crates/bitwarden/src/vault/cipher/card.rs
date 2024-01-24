use bitwarden_api_api::models::CipherCardModel;
use bitwarden_crypto::{
    CryptoError, EncString, KeyDecryptable, KeyEncryptable, SymmetricCryptoKey,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

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

impl KeyEncryptable<SymmetricCryptoKey, Card> for CardView {
    fn encrypt_with_key(self, key: &SymmetricCryptoKey) -> Result<Card, CryptoError> {
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

impl KeyDecryptable<SymmetricCryptoKey, CardView> for Card {
    fn decrypt_with_key(&self, key: &SymmetricCryptoKey) -> Result<CardView, CryptoError> {
        Ok(CardView {
            cardholder_name: self.cardholder_name.decrypt_with_key(key).ok().flatten(),
            exp_month: self.exp_month.decrypt_with_key(key).ok().flatten(),
            exp_year: self.exp_year.decrypt_with_key(key).ok().flatten(),
            code: self.code.decrypt_with_key(key).ok().flatten(),
            brand: self.brand.decrypt_with_key(key).ok().flatten(),
            number: self.number.decrypt_with_key(key).ok().flatten(),
        })
    }
}

impl TryFrom<CipherCardModel> for Card {
    type Error = Error;

    fn try_from(card: CipherCardModel) -> Result<Self> {
        Ok(Self {
            cardholder_name: EncString::try_from_optional(card.cardholder_name)?,
            exp_month: EncString::try_from_optional(card.exp_month)?,
            exp_year: EncString::try_from_optional(card.exp_year)?,
            code: EncString::try_from_optional(card.code)?,
            brand: EncString::try_from_optional(card.brand)?,
            number: EncString::try_from_optional(card.number)?,
        })
    }
}
