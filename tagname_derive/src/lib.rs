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
use syn::punctuated::Pair;
use syn::{Ident, Variant};

struct TaggedUnion {
    name: Ident,
    variants_with_fields: Vec<Ident>,
    variants_without_fields: Vec<Ident>,
}

/// Generates derive implementation for TagName
#[proc_macro_derive(TagName)]
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

    let variants_with_fields = variants.iter().cloned().filter(|v| !v.fields.is_empty());
    let variants_without_fields = variants.iter().cloned().filter(|v| v.fields.is_empty());
    let with_field_tags: Vec<Ident> = variants_with_fields.into_iter().map(|v| v.ident).collect();
    let no_field_tags: Vec<Ident> = variants_without_fields
        .into_iter()
        .map(|v| v.ident)
        .collect();
    TaggedUnion {
        name: name.clone(),
        variants_without_fields: no_field_tags,
        variants_with_fields: with_field_tags,
    }
}

fn generate_code(tagged_union: TaggedUnion) -> TokenStream {
    let name = tagged_union.name;
    let variants_without_fields = tagged_union.variants_without_fields;
    let variants_with_fields = tagged_union.variants_with_fields;

    let comma = if variants_with_fields.is_empty() {
        quote!()
    } else {
        quote!(,)
    };

    let gen = quote! {
        impl TagName for #name {
            fn tag_name(&self) -> &'static str {
                match self {
                    #(
                        #name::#variants_with_fields(_) => stringify!(#variants_with_fields)
                    ),*#comma
                    #(
                        #name::#variants_without_fields => stringify!(#variants_without_fields)
                    ),*
                }
            }
        }
    };
    gen.into()
}
