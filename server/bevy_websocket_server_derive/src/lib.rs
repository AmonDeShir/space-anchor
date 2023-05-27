use syn::{parse_macro_input, DeriveInput};
use quote::quote;
use proc_macro::TokenStream;

/// Derive macro generating an impl of the trait ReceivableMessage.
#[proc_macro_derive(ReceivableMessage)]
pub fn derive_receivable_message(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl ReceivableMessage for #ident {}

        impl Default for #ident {
            fn default() -> Self {
                Self::NONE
            }
        }
    };
    output.into()
}

/// Derive macro generating an impl of the trait SendableMessage
#[proc_macro_derive(SendableMessage)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl SendableMessage for #ident {}
    };
    output.into()
}
