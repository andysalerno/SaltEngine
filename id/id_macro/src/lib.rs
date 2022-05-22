extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, Parser};
use syn::{parse, parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn id(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let name = item_struct.ident.clone();
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
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #item_struct

        impl #name {
            pub fn new() -> Self {
                Self {
                    id: id::Id::new()
                }
            }
        }

    }
    .into();

    // Useful during debug:
    // println!("{:?}", stream);

    return stream;
}
