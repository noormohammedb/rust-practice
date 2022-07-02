// use proc_macro;

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]

struct Pancakes;

pub fn run() {
    Pancakes::hello_macro();
}
/*
#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
    //...
}
*/
