use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Entity, attributes(table, primary_key))]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl Entity for #name {
            fn table_name() -> &'static str {
                stringify!(#name)
            }
            fn primary_key() -> &'static str {
                "id"
            }
        }
    };

    TokenStream::from(expanded)
}
