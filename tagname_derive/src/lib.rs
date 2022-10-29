#![deny(
    warnings,
    missing_debug_implementations,
    rust_2018_idioms,
    nonstandard_style,
    future_incompatible,
    clippy::all,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::pedantic
)]
#![allow(clippy::doc_markdown, clippy::missing_panics_doc)]
#![forbid(unsafe_code)]

use proc_macro::TokenStream;
use quote::quote;
use syn::__private::TokenStream2;
use syn::punctuated::Pair;
use syn::{Ident, Variant};

struct TagData {
    ident: Ident,
}

enum Tag {
    Empty(TagData),
    NotEmpty(TagData),
}

struct TaggedUnion {
    name: Ident,
    tags: Vec<Tag>,
}

/// Generates derive implementation for TagName
#[proc_macro_derive(TagName, attributes(tag))]
pub fn tagname_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    generate_code(traverse_ast(ast))
}

fn traverse_ast(ast: syn::DeriveInput) -> TaggedUnion {
    let name = &ast.ident;
    let variants: Vec<Variant> = match ast.data {
        syn::Data::Enum(enum_data) => enum_data
            .variants
            .into_pairs()
            .into_iter()
            .map(Pair::into_value)
            .collect(),
        _ => panic!("cannot derive TagName for non-enum types"),
    };

    assert!(
        !variants.is_empty(),
        "cannot derive TagName for empty enum types"
    );
    let tags = variants
        .into_iter()
        .map(|v| {
            if v.fields.is_empty() {
                Tag::Empty(TagData { ident: v.ident })
            } else {
                Tag::NotEmpty(TagData { ident: v.ident })
            }
        })
        .collect();

    TaggedUnion {
        name: name.clone(),
        tags,
    }
}

fn generate_match_arm(enum_name: Ident, tag: Tag) -> TokenStream2 {
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

fn generate_code(tagged_union: TaggedUnion) -> TokenStream {
    let name = tagged_union.name;
    let match_arms: Vec<TokenStream2> = tagged_union
        .tags
        .into_iter()
        .map(|tag| generate_match_arm(name.clone(), tag))
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
