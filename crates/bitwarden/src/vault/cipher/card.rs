use bitwarden_api_api::models::CipherCardModel;
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

impl Encryptable<Card> for CardView {
    fn encrypt(self, enc: &EncryptionSettings, org_id: &Option<Uuid>) -> Result<Card> {
        Ok(Card {
            cardholder_name: self.cardholder_name.encrypt(enc, org_id)?,
            exp_month: self.exp_month.encrypt(enc, org_id)?,
            exp_year: self.exp_year.encrypt(enc, org_id)?,
            code: self.code.encrypt(enc, org_id)?,
            brand: self.brand.encrypt(enc, org_id)?,
            number: self.number.encrypt(enc, org_id)?,
        })
    }
}

impl Decryptable<CardView> for Card {
    fn decrypt(&self, enc: &EncryptionSettings, org_id: &Option<Uuid>) -> Result<CardView> {
        Ok(CardView {
            cardholder_name: self.cardholder_name.decrypt(enc, org_id)?,
            exp_month: self.exp_month.decrypt(enc, org_id)?,
            exp_year: self.exp_year.decrypt(enc, org_id)?,
            code: self.code.decrypt(enc, org_id)?,
            brand: self.brand.decrypt(enc, org_id)?,
            number: self.number.decrypt(enc, org_id)?,
        })
    }
}

impl TryFrom<CipherCardModel> for Card {
    type Error = Error;

    fn try_from(card: CipherCardModel) -> Result<Self> {
        Ok(Self {
            cardholder_name: EncString::try_from(card.cardholder_name)?,
            exp_month: EncString::try_from(card.exp_month)?,
            exp_year: EncString::try_from(card.exp_year)?,
            code: EncString::try_from(card.code)?,
            brand: EncString::try_from(card.brand)?,
            number: EncString::try_from(card.number)?,
        })
    }
}
