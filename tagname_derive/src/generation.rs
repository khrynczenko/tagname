use proc_macro::TokenStream;
use quote::quote;
use syn::Ident;
use syn::__private::TokenStream2;

use super::{Tag, TaggedUnion};

pub(crate) fn generate_code(tagged_union: TaggedUnion) -> TokenStream {
    let name = tagged_union.name;
    let match_arms: Vec<TokenStream2> = tagged_union
        .tags
        .into_iter()
        .map(|tag| generate_match_arm(&name, tag))
        .collect();

    let gen = quote! {
        impl TagName for #name {
            fn tag_name(&self) -> &'static str {
                match self {
                    #(#match_arms)*
                }
            }
        }
    };
    gen.into()
}

fn generate_match_arm(enum_name: &Ident, tag: Tag) -> TokenStream2 {
    match tag {
        Tag::Empty(data) => {
            let ident = data.ident;
            quote!(#enum_name::#ident => stringify!(#ident),)
        }
        Tag::NotEmpty(data) => {
            let ident = data.ident;
            quote!(#enum_name::#ident(..) => stringify!(#ident),)
        }
    }
}
