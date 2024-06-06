use bitwarden::{
    auth::password::MasterPasswordPolicyOptions,
    generators::{PassphraseGeneratorRequest, PasswordGeneratorRequest},
    mobile::crypto::{InitOrgCryptoRequest, InitUserCryptoRequest},
    platform::FingerprintRequest,
    send::{Send, SendListView, SendView},
    tool::ExportFormat,
    vault::{Cipher, CipherView, Collection, Folder, FolderView, TotpResponse},
};
use bitwarden_crypto::{HashPurpose, Kdf};
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
    InitUserCryptoRequest(InitUserCryptoRequest),
    InitOrgCryptoRequest(InitOrgCryptoRequest),
    HashPurpose(HashPurpose),

    // Generators
    PasswordGeneratorRequest(PasswordGeneratorRequest),
    PassphraseGeneratorRequest(PassphraseGeneratorRequest),

    // Exporters
    ExportFormat(ExportFormat),

    // Platform
    FingerprintRequest(FingerprintRequest),

    // Auth
    MasterPasswordPolicyOptions(MasterPasswordPolicyOptions),

    // Kdf
    Kdf(Kdf),

    /// TOTP
    TotpResponse(TotpResponse),
}
