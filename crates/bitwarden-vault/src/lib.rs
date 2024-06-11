#[cfg(feature = "uniffi")]
uniffi::setup_scaffolding!();
#[cfg(feature = "uniffi")]
mod uniffi_support;

mod cipher;
pub use cipher::*;
mod collection;
pub use collection::{Collection, CollectionView};
mod folder;
pub use folder::{Folder, FolderView};
mod password_history;
pub use password_history::{PasswordHistory, PasswordHistoryView};
mod domain;
pub use domain::GlobalDomains;
mod totp;
pub use totp::{generate_totp, TotpError, TotpResponse};
mod error;
pub use error::VaultParseError;
