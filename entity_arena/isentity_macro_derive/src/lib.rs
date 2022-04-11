extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemStruct};

#[proc_macro_attribute]
pub fn entity(args: TokenStream, mut input: TokenStream) -> TokenStream {
    // "Input" is the definition of the struct this attribute is decorating
    let cloned = input.clone();

    let struct_definition = parse_macro_input!(cloned as ItemStruct);
    let name: Ident = struct_definition.ident;

    // `args` is the `TokenStream` of the attribute argument
    let a = parse_macro_input!(args as syn::ExprLit);
    let a = match a.lit {
        syn::Lit::Str(v) => v.value(),
        _ => panic!("Expected a string-formatted guid. For example: #[entity(\"0c45096a-5bd7-4879-a5d7-1d7258f04fb0\")]"),
    };

    let q: TokenStream = quote! {
       impl IsEntity for #name {
            fn entity_type_id() -> entity_arena::id::EntityTypeId {
                EntityTypeId::parse_str(#a)
            }
       }
    }
    .into();

    // Our output is an exact copy of the input struct definition, plus the trait implementation
    input.extend(q);

    input
}
