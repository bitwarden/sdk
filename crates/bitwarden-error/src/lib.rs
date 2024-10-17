mod flat_error;

pub use flat_error::FlatError;

pub mod prelude {
    pub use crate::FlatError;
    pub use bitwarden_error_macro::FlatError;
}
