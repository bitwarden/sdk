use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use bitwarden_api_api::models::cipher_details_response_model::CipherDetailsMetaDataResponseModel;
use bitwarden_crypto::{
    CryptoError, EncString, KeyDecryptable, KeyEncryptable, SymmetricCryptoKey,
};

use super::migration::{Downgrader, Migrator};

pub struct V2Migrator {}

impl Migrator for V2Migrator {
    fn migrate(
        _metadata: &CipherDetailsMetaDataResponseModel,
        input: &serde_json::Value,
        key: &SymmetricCryptoKey,
    ) -> Result<serde_json::Value, CryptoError> {
        // TODO: Fix clone
        let mut data = input.clone();

        if (data["version"].as_i64().unwrap_or(0)) != 1 {
            return Ok(data);
        }

        let default = vec![];
        let mut fido2_credentials: Vec<serde_json::Value> = data["login"]["fido2Credentials"]
            .as_array()
            .unwrap_or(&default)
            .clone();

        for fido2_credential in fido2_credentials.iter_mut() {
            let credential_id = fido2_credential["credentialId"]
                .as_str()
                .expect("Fido2Credential missing ID")
                .to_owned();
            let enc_string: EncString = credential_id.parse()?;
            let dec_credential_id: String = enc_string.decrypt_with_key(key)?;

            let byte_credential_id = string_to_guid_bytes(&dec_credential_id);
            let b64_credential_id = URL_SAFE_NO_PAD.encode(byte_credential_id);

            let enc_credential_id = b64_credential_id.encrypt_with_key(key)?;

            fido2_credential["credentialIdType"] = serde_json::Value::String("b64".to_owned());
            fido2_credential["credentialId"] =
                serde_json::Value::String(enc_credential_id.to_string());
        }

        if !fido2_credentials.is_empty() {
            data["login"]["fido2Credentials"] = serde_json::Value::Array(fido2_credentials);
        }

        data["version"] = 2.into();

        Ok(data)
    }
}

impl Downgrader for V2Migrator {
    fn downgrade(
        _metadata: &CipherDetailsMetaDataResponseModel,
        input: &serde_json::Value,
        key: &SymmetricCryptoKey,
    ) -> Result<serde_json::Value, CryptoError> {
        // TODO: Fix clone
        let mut data = input.clone();

        if (data["version"].as_i64().unwrap_or(0)) != 2 {
            return Ok(data);
        }

        let default = vec![];
        let mut fido2_credentials: Vec<serde_json::Value> = data["login"]["fido2Credentials"]
            .as_array()
            .unwrap_or(&default)
            .clone();

        for fido2_credential in fido2_credentials.iter_mut() {
            let credential_id = fido2_credential["credentialId"]
                .as_str()
                .expect("Fido2Credential missing ID")
                .to_owned();
            let enc_string: EncString = credential_id.parse()?;
            let dec_credential_id: String = enc_string.decrypt_with_key(key)?;

            // TODO: Use proper errors
            let byte_credential_id = URL_SAFE_NO_PAD
                .decode(&dec_credential_id)
                .map_err(|_| CryptoError::MissingField("data"))?;
            let guid_credential_id = guid_bytes_to_string(&byte_credential_id);

            match guid_credential_id {
                Ok(guid_credential_id) => {
                    // This credential can be represented as a UUID
                    let enc_credential_id = guid_credential_id.encrypt_with_key(key)?;
                    fido2_credential["version"] = 1.into();
                    fido2_credential["credentialId"] =
                        serde_json::Value::String(enc_credential_id.to_string());
                }
                Err(_) => {
                    // This credential cannot be represented as a UUID and cannot be downgraded,
                    // abort the downgrade and return the original data.
                    return Ok(data);
                }
            }
        }

        if !fido2_credentials.is_empty() {
            data["login"]["fido2Credentials"] = serde_json::Value::Array(fido2_credentials);
        }

        data["version"] = 1.into();

        Ok(data)
    }
}

pub fn string_to_guid_bytes(source: &String) -> Vec<u8> {
    uuid::Uuid::try_parse(source).unwrap().as_bytes().to_vec()
}

fn guid_bytes_to_string(source: &[u8]) -> Result<String, uuid::Error> {
    Ok(uuid::Uuid::from_slice(source)?.to_string())
}

#[cfg(test)]
mod test {
    use bitwarden_crypto::KeyEncryptable;

    use super::*;

    #[test]
    fn test_migrate_v2() {
        let key = SymmetricCryptoKey::try_from("UY4B5N4DA4UisCNClgZtRr6VLy9ZF5BXXC7cDZRqourKi4ghEMgISbCsubvgCkHf5DZctQjVot11/vVvN9NNHQ==".to_owned()).unwrap();
        let credential_id = "cba2f506-b21a-482a-920d-1f86fe08a52b".to_owned();
        let enc_credential_id = credential_id
            .encrypt_with_key(&key)
            .expect("Failed to encrypt");

        let metadata = CipherDetailsMetaDataResponseModel::default();
        let data = serde_json::json!({
            "version": 1,
            "type": "login",
            "login": {
                "fido2Credentials": [
                    { "credentialId": enc_credential_id.to_string() }
                ]
            }
        });

        let result = V2Migrator::migrate(&metadata, &data, &key).expect("Failed to migrate");

        let b64_credential_id = "y6L1BrIaSCqSDR-G_gilKw".to_owned();
        let enc_b64_credential_id = b64_credential_id
            .encrypt_with_key(&key)
            .expect("Failed to encrypt");

        let expected = serde_json::json!({
            "version": 2,
            "type": "login",
            "login": {
                "fido2Credentials": [
                    {
                        "credentialIdType": "b64",
                        "credentialId": enc_b64_credential_id.to_string()
                    }
                ]
            }
        });

        // TODO: Fix. The EncString result seems to be indeterministic
        assert_eq!(result, expected);
    }
}
