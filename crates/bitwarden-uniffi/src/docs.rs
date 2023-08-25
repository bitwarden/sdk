use bitwarden::{
    client::auth_settings::Kdf,
    mobile::crypto::InitCryptoRequest,
    vault::{Cipher, CipherView, Collection, Folder, FolderView},
};
use schemars::JsonSchema;

#[derive(JsonSchema)]
pub enum DocRef {
    // Vault
    Cipher(Cipher),
    CipherView(CipherView),
    Collection(Collection),
    Folder(Folder),
    FolderView(FolderView),

    // Crypto
    InitCryptoRequest(InitCryptoRequest),

    // Kdf
    Kdf(Kdf),
}
