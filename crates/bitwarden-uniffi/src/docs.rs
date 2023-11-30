use bitwarden::{
    auth::password::MasterPasswordPolicyOptions,
    client::kdf::Kdf,
    mobile::crypto::{InitOrgCryptoRequest, InitUserCryptoRequest},
    tool::{ExportFormat, PassphraseGeneratorRequest, PasswordGeneratorRequest},
    vault::{
        Cipher, CipherView, Collection, Folder, FolderView, Send, SendListView, SendView,
        TotpResponse,
    },
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
    InitUserCryptoRequest(InitUserCryptoRequest),
    InitOrgCryptoRequest(InitOrgCryptoRequest),

    // Generators
    PasswordGeneratorRequest(PasswordGeneratorRequest),
    PassphraseGeneratorRequest(PassphraseGeneratorRequest),

    // Exporters
    ExportFormat(ExportFormat),

    // Auth
    MasterPasswordPolicyOptions(MasterPasswordPolicyOptions),

    // Kdf
    Kdf(Kdf),

    /// TOTP
    TotpResponse(TotpResponse),
}
