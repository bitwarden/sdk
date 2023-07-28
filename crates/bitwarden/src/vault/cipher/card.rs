use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    client::encryption_settings::EncryptionSettings,
    crypto::{CipherString, Encryptable},
    error::Result,
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Card {
    pub cardholder_name: Option<CipherString>,
    pub exp_month: Option<CipherString>,
    pub exp_year: Option<CipherString>,
    pub code: Option<CipherString>,
    pub brand: Option<CipherString>,
    pub number: Option<CipherString>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
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
