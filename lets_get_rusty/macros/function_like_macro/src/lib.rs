extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    println!("{}", input.to_string());
    let foo = quote!();
    foo.into()
}
