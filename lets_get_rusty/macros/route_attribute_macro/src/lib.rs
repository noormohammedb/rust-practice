extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn route(
    attr: TokenStream, // GET, "/"
    item: TokenStream, // fn index() {}
) -> TokenStream {
    println!("{}", attr.to_string());
    println!("{}", item.to_string());
    let foo = quote!();
    foo.into()
}
