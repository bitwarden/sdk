use std::path::Path;

use zeroize::Zeroize;

pub const TEST_STRING: &str = "THIS IS USED TO CHECK THAT THE MEMORY IS DUMPED CORRECTLY";

pub fn load_cases(base_dir: &Path) -> Cases {
    let mut json_str = std::fs::read_to_string(base_dir.join("cases.json")).unwrap();
    let cases: Cases = serde_json::from_str(&json_str).unwrap();

    // Make sure that we don't leave extra copies of the data in memory
    json_str.zeroize();
    cases
}

// Note: We don't actively zeroize these structs here because we want the code in bitwarden_crypto
// to handle it for us
#[derive(serde::Deserialize)]
pub struct Cases {
    pub symmetric_key: Vec<SymmetricKeyCases>,
}

#[derive(serde::Deserialize)]
pub struct SymmetricKeyCases {
    pub key: String,

    pub decrypted_key_hex: String,
    pub decrypted_mac_hex: String,
}
