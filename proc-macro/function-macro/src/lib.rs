#![allow(dead_code, unused_imports, unused_variables)]
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DataEnum, DataUnion, DeriveInput, FieldsNamed, FieldsUnnamed};

#[proc_macro]
pub fn function_macro_declare(input: TokenStream) -> TokenStream {
    println!("function_macro_declare. input: {:?}", input);
    /*
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let output = quote! {
        impl #ident {
            fn doit() {
                println!("bajja..");
            }
        }
    };

    output.into()
    */
    input
}

