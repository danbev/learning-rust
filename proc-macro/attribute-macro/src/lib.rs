extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn attribute_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("hello attribute macro..attr: {:?}, item: {:?}", attr, item);
    /*
    for i in item {
        println!("{}", i);
    }
    */
    item
}
