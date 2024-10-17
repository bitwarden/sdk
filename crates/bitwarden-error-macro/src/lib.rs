use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(FlatError)]
pub fn derive_flat_error(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Generate match arms without converting variant names to strings
    let variant_matches: Vec<_> = if let Data::Enum(data_enum) = &input.data {
        data_enum
            .variants
            .iter()
            .map(|variant| {
                let variant_ident = &variant.ident;
                let message = match &variant.fields {
                    Fields::Unnamed(_) | Fields::Named(_) => {
                        format!("Error: {}", variant_ident)
                    }
                    Fields::Unit => {
                        format!("{}", variant_ident)
                    }
                };
                quote! {
                    #name::#variant_ident { .. } => (stringify!(#variant_ident), #message),
                }
            })
            .collect()
    } else {
        panic!("FlatError can only be derived for enums");
    };

    let expanded = quote! {
        impl FlatError for #name {
            fn get_variant(&self) -> &str {
                match self {
                    #(#variant_matches)*
                }.0
            }

            fn get_message(&self) -> &str {
                match self {
                    #(#variant_matches)*
                }.1
            }
        }
    };

    TokenStream::from(expanded)
}
