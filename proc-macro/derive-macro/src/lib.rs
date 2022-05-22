use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DataEnum, DataUnion, DeriveInput, FieldsNamed, FieldsUnnamed};

#[proc_macro_derive(Bajja)]
pub fn some_derive_macro(input: TokenStream) -> TokenStream {
    println!("some_derivce_macro. input: {:?}", input);
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let output = quote! {
        impl #ident {
            fn doit() {
                println!("bajja..");
            }
        }
    };

    output.into()
}

