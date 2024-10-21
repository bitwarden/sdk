use crate::{AsymmetricCryptoKey, SymmetricCryptoKey};

// Hide the `KeyRef` trait from the public API, to avoid confusion
// the trait itself needs to be public to reference it in the macro, so wrap it in a hidden module
#[doc(hidden)]
pub mod __internal {
    use std::{fmt::Debug, hash::Hash};

    use zeroize::ZeroizeOnDrop;

    use crate::CryptoKey;

    /// This trait represents a key reference that can be used to identify cryptographic keys in the
    /// key store. It is used to abstract over the different types of keys that can be used in
    /// the system, an end user would not implement this trait directly, and would instead use
    /// the `SymmetricKeyRef` and `AsymmetricKeyRef` traits.
    pub trait KeyRef:
        Debug + Clone + Copy + Hash + Eq + PartialEq + Ord + PartialOrd + Send + Sync + 'static
    {
        type KeyValue: CryptoKey + Send + Sync + ZeroizeOnDrop;

        /// Returns whether the key is local to the current context or shared globally by the
        /// service.
        fn is_local(&self) -> bool;
    }
}
pub(crate) use __internal::KeyRef;

// These traits below are just basic aliases of the `KeyRef` trait, but they allow us to have two
// separate trait bounds

pub trait SymmetricKeyRef: KeyRef<KeyValue = SymmetricCryptoKey> {}
pub trait AsymmetricKeyRef: KeyRef<KeyValue = AsymmetricCryptoKey> {}

// Just a small derive_like macro that can be used to generate the key reference enums.
// Example usage:
// ```rust
// key_refs! {
//     #[symmetric]
//     pub enum KeyRef {
//         User,
//         Org(Uuid),
//         #[local]
//         Local(String),
//     }
// }
#[macro_export]
macro_rules! key_refs {
    ( $(
        #[$meta_type:tt]
        $(pub)? enum $name:ident {
            $(
                $( #[$variant_tag:tt] )?
                $variant:ident $( ( $inner:ty ) )?
            ,)+
        }
    )+ ) => {  $(
        #[derive(std::fmt::Debug, Clone, Copy, std::hash::Hash, Eq, PartialEq, Ord, PartialOrd)]
        pub enum $name { $(
                $variant  $( ($inner) )?
        ,)+ }

        impl $crate::key_ref::__internal::KeyRef for $name {
            type KeyValue = key_refs!(@key_type $meta_type);

            fn is_local(&self) -> bool {
                use $name::*;
                match self { $(
                    key_refs!(@variant_match $variant $( ( $inner ) )?) =>
                        key_refs!(@variant_tag $( $variant_tag )? ),
                )+ }
            }
        }

        key_refs!(@key_trait $meta_type $name);
    )+ };

    ( @key_type symmetric ) => { $crate::SymmetricCryptoKey };
    ( @key_type asymmetric ) => { $crate::AsymmetricCryptoKey };

    ( @key_trait symmetric $name:ident ) => { impl $crate::key_ref::SymmetricKeyRef for $name {} };
    ( @key_trait asymmetric $name:ident ) => { impl $crate::key_ref::AsymmetricKeyRef for $name {} };

    ( @variant_match $variant:ident ( $inner:ty ) ) => { $variant (_) };
    ( @variant_match $variant:ident ) => { $variant };

    ( @variant_tag local ) => { true };
    ( @variant_tag ) => { false };
}
