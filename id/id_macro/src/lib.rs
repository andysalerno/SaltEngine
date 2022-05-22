extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, Parser};
use syn::{parse, parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn id(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let _ = parse_macro_input!(args as parse::Nothing);

    if let syn::Fields::Unit = item_struct.fields {
        let tokens = quote! { { pub id: id::Id, } };
        let fields = syn::FieldsNamed::parse.parse2(tokens).unwrap();
        let named = syn::Fields::Named(fields);
        item_struct.fields = named;
    } else {
        panic!("This macro is only valid on unit structs.");
    }

    let stream: TokenStream = quote! {
        #[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #item_struct
    }
    .into();

    // Useful during debug:
    // println!("{:?}", stream);

    return stream;
}
