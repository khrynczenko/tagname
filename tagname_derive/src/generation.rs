use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Ident;

use super::{Case, Tag, TaggedUnion};

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
    let (tag_data, wildcard) = match tag {
        Tag::Empty(data) => (data, quote!()),
        Tag::NotEmpty(data) => (data, quote!((..))),
    };
    let ident = tag_data.ident;
    let output_ident = match tag_data.case {
        Case::Lower => quote::format_ident!("{}", ident.to_string().to_lowercase()),
        Case::Upper => quote::format_ident!("{}", ident.to_string().to_uppercase()),
        Case::Unchanged => ident.clone(),
    };
    quote!(#enum_name::#ident #wildcard=> stringify!(#output_ident),)
}
