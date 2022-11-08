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

mod generation;
mod traversal;

use proc_macro::TokenStream;
use syn::Ident;

enum Case {
    Lower,
    Upper,
    Unchanged,
}

struct TagData {
    ident: Ident,
    case: Case,
}

enum Tag {
    Unit(TagData),
    Unnamed(TagData),
    Named(TagData),
}

struct TaggedUnion {
    name: Ident,
    tags: Vec<Tag>,
}

/// Generates derive implementation for TagName
#[proc_macro_derive(TagName, attributes(tag))]
pub fn tagname_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    generation::generate_code(traversal::traverse_ast(ast))
}
