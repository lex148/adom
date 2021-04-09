extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

mod create;
mod delete;
mod select;
mod update;
mod utils;
use proc_macro::TokenStream;

#[proc_macro_derive(AdomCreate, attributes(AdomTable, AdomColumn))]
pub fn adom_create_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    create::impl_create_macro(&ast)
}

#[proc_macro_derive(AdomSelect, attributes(AdomTable, AdomColumn))]
pub fn adom_select_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    select::impl_select_macro(&ast)
}

#[proc_macro_derive(AdomUpdate, attributes(AdomTable, AdomColumn))]
pub fn adom_update_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    update::impl_update_macro(&ast)
}

#[proc_macro_derive(AdomDelete, attributes(AdomDelete))]
pub fn adom_delete_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    delete::impl_delete_macro(&ast)
}
