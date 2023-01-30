use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(FileConfig)]
pub fn derive_config(input: TokenStream) -> TokenStream {
    let name = syn::parse_macro_input!(input as DeriveInput).ident;

    TokenStream::from(quote! { impl FileConfig for #name {} })
}