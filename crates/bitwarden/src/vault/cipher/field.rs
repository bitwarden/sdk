use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::crypto::CipherString;

#[derive(Clone, Copy, Serialize, Deserialize, Debug, JsonSchema)]
pub enum FieldType {
    Text = 0,
    Hidden = 1,
    Boolean = 2,
    Linked = 3,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, JsonSchema)]
pub enum LinkedIdType {
    // Login
    Username = 100,
    Password = 101,

    // Card
    CardholderName = 300,
    ExpMonth = 301,
    ExpYear = 302,
    Code = 303,
    Brand = 304,
    Number = 305,

    // Identity
    Title = 400,
    MiddleName = 401,
    Address1 = 402,
    Address2 = 403,
    Address3 = 404,
    City = 405,
    State = 406,
    PostalCode = 407,
    Country = 408,
    Company = 409,
    Email = 410,
    Phone = 411,
    Ssn = 412,
    IdentityUsername = 413,
    PassportNumber = 414,
    LicenseNumber = 415,
    FirstName = 416,
    LastName = 417,
    FullName = 418,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Field {
    name: CipherString,
    value: CipherString,
    r#type: FieldType,

    linked_id: Option<LinkedIdType>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FieldView {
    name: String,
    value: String,
    r#type: FieldType,

    linked_id: Option<LinkedIdType>,
}
