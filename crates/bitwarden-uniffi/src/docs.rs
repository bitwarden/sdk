use bitwarden::{
    auth::password::MasterPasswordPolicyOptions,
    client::auth_settings::Kdf,
    mobile::crypto::InitCryptoRequest,
    tool::{ExportFormat, PassphraseGeneratorRequest, PasswordGeneratorRequest},
    vault::{Cipher, CipherView, Collection, Folder, FolderView},
};
use schemars::JsonSchema;

#[derive(JsonSchema)]
#[allow(clippy::large_enum_variant)]
pub enum DocRef {
    // Vault
    Cipher(Cipher),
    CipherView(CipherView),
    Collection(Collection),
    Folder(Folder),
    FolderView(FolderView),

    // Crypto
    InitCryptoRequest(InitCryptoRequest),

    // Generators
    PasswordGeneratorRequest(PasswordGeneratorRequest),
    PassphraseGeneratorRequest(PassphraseGeneratorRequest),

    // Exporters
    ExportFormat(ExportFormat),

    // Auth
    MasterPasswordPolicyOptions(MasterPasswordPolicyOptions),

    // Kdf
    Kdf(Kdf),
}
