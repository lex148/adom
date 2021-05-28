use crate::utils;
use proc_macro::TokenStream;

pub fn impl_auditable_macro(ast: &syn::DeriveInput) -> TokenStream {
    let gen = quote! {};
    gen.into()
}
