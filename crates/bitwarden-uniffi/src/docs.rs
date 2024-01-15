use bitwarden::{
    auth::password::MasterPasswordPolicyOptions,
    mobile::crypto::{InitOrgCryptoRequest, InitUserCryptoRequest},
    platform::FingerprintRequest,
    tool::{ExportFormat, PassphraseGeneratorRequest, PasswordGeneratorRequest},
    vault::{
        Cipher, CipherView, Collection, Folder, FolderView, Send, SendListView, SendView,
        TotpResponse,
    },
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
