use bitwarden::{
    auth::{password::MasterPasswordPolicyOptions, RegisterResponse},
    client::auth_settings::Kdf,
    mobile::crypto::InitCryptoRequest,
    tool::{ExportFormat, PassphraseGeneratorRequest, PasswordGeneratorRequest},
    vault::{Cipher, CipherView, Collection, Folder, FolderView, Send, SendListView, SendView},
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
    Send(Send),
    SendView(SendView),
    SendListView(SendListView),

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
