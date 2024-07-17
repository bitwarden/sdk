use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use bitwarden_crypto::{
    CryptoError, EncString, KeyDecryptable, KeyEncryptable, SymmetricCryptoKey,
};

use crate::cipher_data::v1::CipherDataV1;

use super::CipherDataV2;

pub fn migrate_v2(
    data: &CipherDataV1,
    key: &SymmetricCryptoKey,
) -> Result<CipherDataV2, CryptoError> {
    // TODO: meh, fix default
    let mut data = data.data.clone();
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

        fido2_credential["credentialId"] = serde_json::Value::String(enc_credential_id.to_string());
    }

    if !fido2_credentials.is_empty() {
        data["login"]["fido2Credentials"] = serde_json::Value::Array(fido2_credentials);
    }

    Ok(CipherDataV2 { data })
}

pub fn string_to_guid_bytes(source: &String) -> Vec<u8> {
    uuid::Uuid::try_parse(source).unwrap().as_bytes().to_vec()
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

        let data = CipherDataV1 {
            data: serde_json::json!({
                "type": "login",
                "login": {
                    "fido2Credentials": [
                        { "credentialId": enc_credential_id.to_string() }
                    ]
                }
            }),
        };

        let result = migrate_v2(&data, &key).expect("Failed to migrate");

        let b64_credential_id = "y6L1BrIaSCqSDR-G_gilKw".to_owned();
        let enc_b64_credential_id = b64_credential_id
            .encrypt_with_key(&key)
            .expect("Failed to encrypt");

        let expected = CipherDataV2 {
            data: serde_json::json!({
                "type": "login",
                "login": {
                    "fido2Credentials": [
                        { "credentialId": enc_b64_credential_id.to_string() }
                    ]
                }
            }),
        };

        // TODO: Fix. The EncString result seems to be indeterministic
        assert_eq!(result.data, expected.data);
    }
}
