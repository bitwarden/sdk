use std::path::Path;

use bitwarden_crypto::Kdf;
use zeroize::{Zeroize, Zeroizing};

pub const TEST_STRING: &str = "THIS IS USED TO CHECK THAT THE MEMORY IS DUMPED CORRECTLY";

pub fn load_cases(base_dir: &Path) -> Cases {
    let mut json_str = std::fs::read_to_string(base_dir.join("cases.json")).unwrap();
    let cases: Cases = serde_json::from_str(&json_str).unwrap();

    // Make sure that we don't leave extra copies of the string data in memory
    json_str.zeroize();
    cases
}

#[derive(serde::Deserialize)]
pub struct Cases {
    pub cases: Vec<Case>,
}

#[derive(serde::Deserialize)]
pub struct Case {
    pub name: String,
    #[serde(flatten)]
    pub command: CaseCommand,
    pub memory_lookups: Vec<MemoryLookup>,
}

// We don't actively zeroize this struct because we want the code in bitwarden_crypto
// to handle it for us
#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaseCommand {
    SymmetricKey {
        key: String,
    },
    AsymmetricKey {
        private_key: String,
    },
    MasterKey {
        password: String,
        email: String,
        kdf: Kdf,
    },
}

#[derive(serde::Deserialize)]
pub struct MemoryLookup {
    pub name: String,

    #[serde(flatten)]
    pub value: MemoryLookupValue,

    #[serde(default)]
    pub allowed_count: Option<usize>,
}

// We don't actually want these values to be caught by the memory testing,
// so this enum should be always zeroized
#[derive(serde::Deserialize)]
#[serde(untagged)]
pub enum MemoryLookupValue {
    String { string: Zeroizing<String> },
    Binary { hex: Zeroizing<String> },
}
