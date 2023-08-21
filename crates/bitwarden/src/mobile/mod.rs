#[cfg(feature = "internal")]
pub mod crypto;
pub mod kdf;
pub mod vault;

pub(crate) mod client_crypto;
pub(crate) mod client_kdf;

// Usually we wouldn't want to expose CipherStrings in the API or the schemas,
// but we need them in the mobile API, so define it here to limit the scope
impl schemars::JsonSchema for crate::crypto::CipherString {
    fn schema_name() -> String {
        "CipherString".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        gen.subschema_for::<String>()
    }
}
